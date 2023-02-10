import React from "react";
import { createPortal } from "react-dom";
import style from './index.module.less';


interface Props {
  show: boolean;
  mask?: boolean;
  children: React.ReactNode;
}

export default function Modal(props: Props) {

  const content = <div className={style.container}>
    {props.mask && <div className={style.mask}></div>}
    <div className={style.modal}>
      {props.children}
    </div>
  </div>
  const portal = createPortal(content, document.body);
  return props.show ? portal : null;
}