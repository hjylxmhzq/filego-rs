import { useRef } from 'react';
import { formatFileSize } from '../../utils/formatter';
import style from './index.module.less';

interface Props {
  percent: number;
  text?: string;
}

export default function Progress(props: Props) {
  return <div className={style.progress}>
    <div style={{ position: 'absolute', left: 0, width: props.percent * 100 + '%', height: '100%', backgroundColor: '#aaa' }}></div>
    <div className={style.text}>{props.text}</div>
  </div>
}

interface UploadProps {
  total: number;
  uploaded: number;
}

export function UploadProgress(props: UploadProps) {
  const ref = useRef<{ time: number, uploaded: number }[]>([]);
  const now = { time: Date.now(), uploaded: props.uploaded }
  ref.current.push(now);
  while (now.time - ref.current[0].time > 5000) {
    ref.current.shift();
  }
  const duration = (now.time - ref.current[0].time) / 1000;
  const delta = now.uploaded - ref.current[0].uploaded;
  const deltaTime = delta / duration;
  const speed = formatFileSize(deltaTime || 0) + '/s';
  return <Progress percent={props.uploaded / props.total} text={speed} />
}