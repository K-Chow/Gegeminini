import { useEffect, useState } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'

type NavProps = {
  isOpen: boolean
  onClose: () => void
}

type ChatItem = {
  id: string
  title: string
  path: string
}

const Nav = ({ onClose }: NavProps) => {
  const { pathname } = useLocation()
  const [chats, setChats] = useState<ChatItem[]>([
    {
      id: '0',
      title: 'New',
      path: '/chat'
    }
  ])
  const [currentChat, setCurrentChat] = useState<ChatItem>(chats[0])

  const getModels = () =>
    invoke('get_model_list')
      .then(res => console.log(res))
      .catch(err => console.log(err))

  useEffect(() => {
    getModels()
  }, [])

  return (
    <div className="drawer-side is-drawer-close:overflow-visible shadow-2xl border-r-1 border-base-300">
      <label className="drawer-overlay" onClick={() => onClose()} />
      <div className="flex min-h-full flex-col items-start bg-base-100 text-base-content is-drawer-close:w-16 is-drawer-open:w-48">
        <div className="w-full h-18" />
        <ul className="menu w-full grow font-bold">
          {chats.map((chat, index) => (
            <li key={index}>
              <Link to={chat.path}>
                <p className="truncate">{chat.title}</p>
              </Link>
            </li>
          ))}
        </ul>
      </div>
    </div>
  )
}

export default Nav
