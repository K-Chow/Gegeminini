import { Navigate } from 'react-router-dom'
import Home from '@/pages/home'
import Chat from '@/pages/chat'
import Settings from '@/pages/settings'
const routes = [
  {
    path: '/',
    element: <Home />,
    children: [
      {
        index: true,
        element: <Navigate to="chat" replace />
      },
      {
        path: 'chat',
        element: <Chat />
      },
      {
        path: 'settings',
        element: <Settings />
      },
      {
        path: '*',
        element: <Navigate to="chat" replace />
      }
    ]
  }
]

export default routes
