import React from 'react';
import './App.less';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import routers from './routers';

const router = createBrowserRouter(routers);

function App() {
  return (
    <div className="App">
      <RouterProvider router={router} />
    </div>
  );
}

export default App;
