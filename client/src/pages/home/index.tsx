import { Outlet, useNavigate } from "react-router";
import { PopButton } from "../../components/button";
import Icon from "../../components/icon/icon";
import { useTheme } from "../../hooks/common";
import Header from "./components/header";
import style from './index.module.less';

export function HomePage() {
  return <div>
    <Header></Header>
    <Outlet />
    <div style={{ position: 'fixed', right: 10, bottom: 10 }}>
      <PopButton button={<div><Icon name="shezhi" size={28} /></div>}>
        <GlobalSettingMenu />
      </PopButton>
    </div>
  </div>
}

function GlobalSettingMenu() {

  const [theme, , toggleTheme] = useTheme();
  const history = useNavigate();

  return <div>
    <div className={style['menu-item']} onClick={toggleTheme} style={{ width: '120px', textAlign: 'left' }}>
      <Icon name="dark" size={18} className={style.icon} />
      {
        theme === 'light' ? 'Dark Mode' : 'Light Mode'
      }
    </div>
    <div className={style['menu-item']} style={{ width: '120px', textAlign: 'left' }} onClick={() => {
      history('/page/setting');
    }}>
      <Icon name="setting" size={16} className={style.icon} />
      App Settings
    </div>
  </div>
}
