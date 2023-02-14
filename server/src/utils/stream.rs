use actix_web::body::{MessageBody, BodySize};
use actix_web::web::Bytes;
use futures::{Stream, ready};
use std::error::Error as StdError;
use std::pin::Pin;
use std::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
  /// Known sized streaming response wrapper.
  ///
  /// This body implementation should be used if total size of stream is known. Data is sent as-is
  /// without using chunked transfer encoding.
  pub struct RangeStream<S> {
      size: u64,
      #[pin]
      stream: S,
      read_bytes: u64,
  }
}

impl<S, E> RangeStream<S>
where
  S: Stream<Item = Result<Bytes, E>>,
  E: Into<Box<dyn StdError>> + 'static,
{
  #[inline]
  pub fn new(size: u64, stream: S) -> Self {
    RangeStream { size, stream, read_bytes: 0 }
  }
}

// TODO: from_infallible method

impl<S, E> MessageBody for RangeStream<S>
where
  S: Stream<Item = Result<Bytes, E>>,
  E: Into<Box<dyn StdError>> + 'static,
{
  type Error = E;

  #[inline]
  fn size(&self) -> BodySize {
    BodySize::Sized(self.size)
  }

  /// Attempts to pull out the next value of the underlying [`Stream`].
  ///
  /// Empty values are skipped to prevent [`SizedStream`]'s transmission being
  /// ended on a zero-length chunk, but rather proceed until the underlying
  /// [`Stream`] ends.
  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    loop {
      let stream = self.as_mut().project().stream;

      let chunk = match ready!(stream.poll_next(cx)) {
        Some(Ok(ref bytes)) if bytes.is_empty() => continue,
        val => {
          let ret;
          if let Some(Ok(ref r)) = val {
            let len = r.len() as u64;
            let p = self.as_mut().project();
            let read_bytes = p.read_bytes;
            let size = *p.size;
            if *read_bytes >= size {
              ret = None;
            } else {
              if *read_bytes + len > size {
                let overflow = size - *read_bytes;
                let r = &r[..overflow as usize];
                ret = Some(Ok(Bytes::copy_from_slice(&r[..overflow as usize])));
              } else {
                ret = val;
              }
            }
            *read_bytes += len;
          } else {
            ret = val;
          }
          ret
        },
      };

      return Poll::Ready(chunk);
    }
  }
}
