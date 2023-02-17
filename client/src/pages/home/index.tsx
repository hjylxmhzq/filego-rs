import { PopButton } from "../../components/button";
import Icon from "../../components/icon/icon";
import { useTheme } from "../../hooks/common";
import FilePage from "../file";
import Header from "./components/header";

export function HomePage() {
  return <div>
    <Header></Header>
    <FilePage />
    <div style={{ position: 'fixed', right: 10, bottom: 10 }}>
      <PopButton button={<div><Icon name="shezhi" size={28} /></div>}>
        <GlobalSettingMenu />
      </PopButton>
    </div>
  </div>
}

function GlobalSettingMenu() {

  const [theme, , toggleTheme] = useTheme();

  return <div>
    <div onClick={toggleTheme} style={{ width: '100px' }}>{
      theme === 'light' ? 'Dark Mode' : 'Light Mode'
    }</div>
  </div>
}
