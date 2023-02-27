import { RouteObject } from 'react-router';
import FilePage from './pages/file';
import { HomePage } from './pages/home';
import LoginPage from './pages/login';
import SettingPage from './pages/setting';

const routers: RouteObject[] = [
  {
    path: '/login',
    element: <LoginPage />
  },
  {
    path: '/',
    element: <HomePage />,
    children: [
      {
        path: '/setting',
        element: <SettingPage />
      },
      {
        path: '',
        element: <FilePage />
      }
    ]
  }
];

export default routers;
