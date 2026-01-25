import { Navigate } from 'react-router-dom'
import Home from '@/pages/home'
import Chat from '@/pages/chat'
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
        path: '*',
        element: <Navigate to="chat" replace />
      }
    ]
  }
]

export default routes
