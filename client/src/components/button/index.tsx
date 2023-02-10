import { MouseEventHandler, ReactNode } from "react";
import style from './index.module.less';

interface Props {
  onClick?: MouseEventHandler;
  children: ReactNode;
}

export default function Button(props: Props) {
  return <button className={style.btn} onClick={props.onClick}>{props.children}</button>
}