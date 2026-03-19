import { Navigate } from 'react-router-dom'
import Home from '@/pages/home'
import Chat from '@/pages/chat'
import Settings from '@/pages/settings'
import {
  ChatBubbleLeftRightIcon,
  Cog6ToothIcon
} from '@heroicons/react/24/outline'

export const subRoutes = [
  {
    index: true,
    element: <Navigate to="chat" replace />
  },
  {
    path: 'settings',
    element: <Settings />,
    icon: <Cog6ToothIcon className="w-6 h-6" />
  },
  {
    path: 'chat',
    element: <Chat />,
    icon: <ChatBubbleLeftRightIcon className="w-6 h-6" />
  },
  {
    path: 'chat/:id',
    element: <Chat />
  },
  {
    path: '*',
    element: <Navigate to="chat" replace />
  }
]

const routes = [
  {
    path: '/',
    element: <Home />,
    children: subRoutes
  }
]

export default routes
