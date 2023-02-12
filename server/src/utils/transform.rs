use std::{path::PathBuf, process::Stdio};
use tokio::{
  io::{duplex, AsyncWriteExt, AsyncReadExt, DuplexStream},
  process,
};
pub async fn ffmpeg_scale(file: &PathBuf, size: u32, bitrate: u32) -> DuplexStream {
  let stream = scale(file, size, bitrate);
  stream
}

fn scale(file: &PathBuf, size: u32, bitrate: u32) -> DuplexStream {
  let (mut w, r) = duplex(1024);
  let file = file.clone();
  tokio::spawn(async move {
    let mut child = process::Command::new("./ffmpeg")
      .args(&[
        "-i",
        file
          .canonicalize()
          .unwrap()
          .to_str()
          .unwrap(),
        "-filter:v",
        &format!("scale=-1:{size}"),
        "-b:v",
        &format!("{bitrate}k"),
        "-movflags",
        "frag_keyframe+empty_moov",
        "-f",
        "mp4",
        "pipe:1",
      ])
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()
      .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut buf = [0; 1024];

    while let Ok(size) = stdout.read(&mut buf).await {
      if size == 0 {
        break;
      }
      let r = w.write_all(&buf[0..size]).await;
      if let Err(_) = r {
        child.kill().await.unwrap();
        break;
      }
    }
  });
  r
}
