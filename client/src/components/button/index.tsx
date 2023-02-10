import { MouseEventHandler, ReactNode } from "react";
import style from './index.module.less';

interface Props {
  onClick?: MouseEventHandler;
  children: ReactNode;
  height?: number
}

export default function Button(props: Props) {
  return <button style={{ height: props.height || 'auto' }} className={style.btn} onClick={props.onClick}>{props.children}</button>
}