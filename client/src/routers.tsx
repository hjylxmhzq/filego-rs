import { RouteObject } from 'react-router';
import { HomePage } from './pages/home';
import LoginPage from './pages/login';

const routers: RouteObject[] = [
  {
    path: '/login',
    element: <LoginPage />
  },
  {
    path: '/',
    element: <HomePage />,
  }
];

export default routers;
