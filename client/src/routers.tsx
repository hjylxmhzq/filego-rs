import { RouteObject } from 'react-router';
import FilePage from './pages/file';
import GalleryPage from './pages/gallery';
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
        path: '/page/setting',
        element: <SettingPage />
      },
      {
        path: '/page/gallery',
        element: <GalleryPage />
      },
      {
        path: '',
        element: <FilePage />
      }
    ]
  }
];

export default routers;
