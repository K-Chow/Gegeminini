import { useGlobalContext } from '@/context/GlobalContext'
import { useState } from 'react'
import { Link, useLocation } from 'react-router-dom'

type NavProps = {
  isOpen: boolean
  onClose: () => void
}

const Nav = ({ onClose }: NavProps) => {
  const { chats, setChats } = useGlobalContext()
  const { pathname } = useLocation()
  const [currentChat, setCurrentChat] = useState(chats[0])
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
