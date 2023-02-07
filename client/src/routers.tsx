import { RouteObject } from 'react-router';
import FilePage from './pages/file';
import LoginPage from './pages/login';

const routers: RouteObject[] = [
  {
    path: '/login',
    element: <LoginPage />
  },
  {
    path: '/',
    element: <FilePage />,
  }
];

export default routers;
