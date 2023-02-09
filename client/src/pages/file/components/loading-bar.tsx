import style from './loading-bar.module.less';

export default function LoadingBar({ loading }: { loading: boolean }) {
  return <div className={style['loading-bar']} style={{opacity: loading ? 1 : 0}}></div>
}