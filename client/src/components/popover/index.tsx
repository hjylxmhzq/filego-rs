import { useEffect, useRef, useState } from "react";
import ReactDOM from 'react-dom';
import style from './index.module.less';

interface Props {
  children: React.ReactElement,
  content: React.ReactElement,
  show: boolean,
}

export function Popover(props: Props) {
  const ref = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const [rect, setRect] = useState({ left: 0, top: 0 });
  useEffect(() => {
    if (ref.current && contentRef.current) {
      const el = ref.current.firstElementChild;
      const contentEl = contentRef.current;
      if (!el) return;
      const rect = el.getBoundingClientRect();
      const rect1 = contentEl.getBoundingClientRect();
      console.log(el, rect, rect1);
      setRect({ left: rect.right - rect1.width, top: rect.top + rect.height + 3 });
    }
  }, [props.show, props.content, props.children]);

  const contentEl = <div ref={contentRef} className={style['popover-item']} style={{ position: 'fixed', left: rect.left, top: rect.top }}>
    {props.content}
  </div>;

  const portal = ReactDOM.createPortal(contentEl, document.body);
  return <div ref={ref}>
    {props.children}
    {props.show && portal}
  </div>
}