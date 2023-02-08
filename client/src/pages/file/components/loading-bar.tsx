import style from './loading-bar.module.less';

export default function LoadingBar({ loading }: { loading: boolean }) {
  return <div className={style['loading-bar']} style={{display: loading ? 'block' : 'none'}}></div>
}