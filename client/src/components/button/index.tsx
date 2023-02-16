import { MouseEventHandler, ReactNode, useEffect, useRef, useState } from "react";
import style from './index.module.less';

interface Props {
  onClick?: MouseEventHandler;
  children: ReactNode;
  height?: number
}

export default function Button(props: Props) {
  return <button style={{ height: props.height || 'auto' }} className={style.btn} onClick={props.onClick}>{props.children}</button>
}


interface PopBtnProps {
  children?: ReactNode;
  button?: ReactNode
}

export function PopButton(props: PopBtnProps) {
  const ref = useRef<HTMLDivElement>(null);
  const rectRef = useRef({ width: 50, height: 50 });
  const [show, setShow] = useState(false);


  useEffect(() => {
    if (ref.current) {
      const rect = ref.current.getBoundingClientRect();
      rectRef.current = { width: rect.width, height: rect.height };
    }
  }, []);

  const onMouseEnter = () => {
    setShow(true);
  }

  const onMouseLeave = () => {
    setShow(false);
  }

  const s = show ? rectRef.current : { width: 50, height: 50 };
  return <div className={style['pop-btn']} style={s} onMouseEnter={onMouseEnter} onMouseLeave={onMouseLeave}>
    <div className={style['pop-content']}>
      <div ref={ref}>{props.children}</div>
    </div>
    <div className={style['pop-inner-btn']}>{props.button}</div>
  </div>
}
