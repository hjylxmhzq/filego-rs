import classNames from "classnames";
import { memo, MouseEventHandler, useEffect, useMemo, useRef, useState } from "react";
import classnames from 'classnames';
import { create_download_link, FileStat } from "../../../../apis/file";
import style from './image-viewer.module.less';
import { debounce } from "../../../../utils/common";

function is_image(file: string) {
  return file.toLowerCase().endsWith('.png') || file.toLowerCase().endsWith('.jpeg') || file.toLowerCase().endsWith('.jpg');
}

export default function ImagePreview({ dir, files, file, onPreviewingChange }: { dir: string, files: FileStat[], file: FileStat, onPreviewingChange?: (file: FileStat) => void }) {

  const pics = useMemo(() => files.filter(f => is_image(f.name)), [files]);
  const idx = useMemo(() => pics.findIndex(f => f.name === file.name), [pics, file]);
  const thumnailRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [show, showThumbnail] = useState(false);
  const [currentIdx, setCurrentIdx] = useState(idx);
  const lastHighlightElRef = useRef<HTMLDivElement>();

  const highlight = (el: HTMLDivElement) => {
    el.classList.add(style['highlight']);
    if (lastHighlightElRef.current) {
      lastHighlightElRef.current.classList.remove(style['highlight']);
    }
    lastHighlightElRef.current = el;
  }

  useEffect(() => {
    const scroll = (idx: number) => {
      if (!thumnailRef.current) return;
      const el = thumnailRef.current;
      const t = el.querySelector(`[data-idx="${idx}"]`) as HTMLDivElement;
      highlight(t);
    }

    const arrowPressed = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft') {
        setCurrentIdx((idx) => {
          if (idx > 0) {
            scroll(idx - 1);
            return idx - 1;
          } else {
            return idx;
          }
        });
      } else if (e.key === 'ArrowRight') {
        setCurrentIdx((idx) => {
          if (idx < pics.length - 1) {
            scroll(idx + 1);
            return idx + 1;
          } else {
            return idx;
          }
        });
      }
    };
    window.addEventListener('keydown', arrowPressed, false);
    return () => {
      window.removeEventListener('keydown', arrowPressed, false);
    };
    // eslint-disable-next-line
  }, [files, onPreviewingChange]);

  useEffect(() => {
    let timer: number | undefined;

    const onMove = (e: MouseEvent) => {
      let el = e.target as HTMLElement;
      if (!el.nodeType || el.nodeType !== document.ELEMENT_NODE) return;
      if (containerRef.current?.contains(el)) {
        showThumbnail(true);
        window.clearTimeout(timer);
        if (thumnailRef.current?.contains(el)) return;
        timer = window.setTimeout(() => {
          if (containerRef.current) {
            showThumbnail(false);
          }
        }, 2000);
      }
    }
    const onMoveDebounced = debounce(onMove, 100);
    window.addEventListener('mousemove', onMoveDebounced, false);
    return () => {
      window.removeEventListener('mousemove', onMoveDebounced, false);
    }
  }, []);

  useEffect(() => {
    onPreviewingChange?.(pics[currentIdx]);
    // eslint-disable-next-line
  }, [onPreviewingChange, currentIdx]);

  const currentSrc = create_download_link(dir, currentIdx === -1 ? file.name : pics[currentIdx].name);
  const clickThumbnail: MouseEventHandler<HTMLDivElement> = (e) => {
    let el = e.target as HTMLDivElement | null;
    while (el && !el?.dataset?.idx) {
      el = el.parentElement as HTMLDivElement;
    }
    if (el?.dataset?.idx) {
      const idx = parseInt(el.dataset.idx);
      setCurrentIdx(idx);
      highlight(el);
    }
  };

  useEffect(() => {
    if (!thumnailRef.current) return;
    const el = thumnailRef.current;
    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      const dy = e.deltaY;
      el.scrollLeft += dy;
    };
    el.addEventListener('wheel', onWheel, false);
    return () => {
      el.removeEventListener('wheel', onWheel, false);
    }
  }, []);

  return <div style={{ display: 'flex', justifyContent: 'center', position: 'relative' }} ref={containerRef}>
    <img loading="lazy" style={{ maxWidth: '100%', height: '90vh', minHeight: 200 }} src={currentSrc} alt={currentSrc} />
    <div onClick={clickThumbnail} className={classNames({ [style['show']]: show }, style['image-thumbnails'], 'scrollbar')} ref={thumnailRef}>
      <Thumbnails pics={pics} dir={dir} />
    </div>
  </ div>
}

const Thumbnails = memo(({ pics, dir }: { pics: FileStat[], dir: string }) => {
  return <>
    {
      pics.map((p, idx) => {
        return <div key={p.name} data-idx={idx} className={classnames(style.thumbnail)}>
          <img loading="lazy" src={create_download_link(dir, p.name)} alt=""></img>
        </div>
      })
    }
  </>
});
