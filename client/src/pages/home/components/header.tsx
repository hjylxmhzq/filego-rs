import { useNavigate } from "react-router";
import { logout } from "../../../apis/auth";
import style from './header.module.less';

export default function Header() {
  const history = useNavigate();

  return <div className={style.header}>
    <span onClick={() => history('/')}>File</span>
    <span>
      <span onClick={async () => {
        await logout();
        window.location.href = '/login';
      }}>Logout</span>
    </span>
  </div>
}