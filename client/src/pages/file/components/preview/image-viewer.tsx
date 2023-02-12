import classNames from "classnames";
import { MouseEventHandler, useEffect, useMemo, useRef, useState } from "react";
import classnames from 'classnames';
import { create_download_link, FileStat } from "../../../../apis/file";
import style from './image-viewer.module.less';
import { debounce } from "../../../../utils/common";

export default function ImagePreview({ dir, files, file, onPreviewingChange }: { dir: string, files: FileStat[], file: FileStat, onPreviewingChange?: (file: FileStat) => void }) {

  const pics = useMemo(() => files.filter(file => file.name.endsWith('.png') || file.name.endsWith('.jpg')), [files]);
  const idx = useMemo(() => pics.findIndex(f => f.name === file.name), [pics, file]);
  const thumnailRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [show, showThumbnail] = useState(false);
  const [currentIdx, setCurrentIdx] = useState(idx);
  useEffect(() => {
    const scroll = (idx: number) => {
      if (!thumnailRef.current) return;
      const el = thumnailRef.current;
      const t = el.querySelector(`[data-idx="${idx}"]`);
      t?.scrollIntoView();
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
    const el = e.currentTarget;
    const idx = parseInt(el.dataset.idx || '0', 10);
    setCurrentIdx(idx);
  };

  return <div style={{ display: 'flex', justifyContent: 'center', position: 'relative' }} ref={containerRef}>
    <img loading="lazy" style={{ maxWidth: '100%', height: '90vh', minHeight: 200 }} src={currentSrc} alt={currentSrc} />
    <div className={classNames({ [style['show']]: show }, style['image-thumbnails'], 'scrollbar')} ref={thumnailRef}>
      {
        pics.map((p, idx) => {
          return <div key={p.name} data-idx={idx} onClick={clickThumbnail} className={classnames(style.thumbnail, { [style['highlight']]: idx === currentIdx })}>
            <img loading="lazy" src={create_download_link(dir, p.name)} alt=""></img>
          </div>
        })
      }
    </div>
  </ div>
}
