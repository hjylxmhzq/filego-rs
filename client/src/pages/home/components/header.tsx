import { logout } from "../../../apis/auth";
import style from './header.module.less';

export default function Header() {
  return <div className={style.header}>
    <span>Hello</span>
    <span>
      <span onClick={async () => {
        await logout();
        window.location.href = '/login';
      }}>Logout</span>
    </span>
  </div>
}