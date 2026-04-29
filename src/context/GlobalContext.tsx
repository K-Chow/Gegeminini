import { createContext, useContext, useState, type ReactElement } from 'react'
import {
  ChatBubbleLeftRightIcon,
  SparklesIcon
} from '@heroicons/react/24/outline'

export type SysConfig = {
  theme: string
  currentApp: string
  model: string
}

interface GlobalProviderProps {
  children: React.ReactNode
}

type ChatItem = {
  id: string
  element: ReactElement
  path: string
}

const GlobalContext = createContext({
  config: {
    theme: '',
    currentApp: '',
    model: ''
  },
  chats: [
    {
      id: '',
      element: <></>,
      path: ''
    }
  ],
  setConfig: (config: SysConfig) => {},
  setChats: (chats: ChatItem[]) => {}
})

export const GlobalProvider = ({ children }: GlobalProviderProps) => {
  const [config, setConfig] = useState<SysConfig>({
    theme: '',
    currentApp: '',
    model: ''
  })

  const [chats, setChats] = useState<ChatItem[]>([
    {
      id: '0',
      element: <ChatBubbleLeftRightIcon className="w-6 h-6" />,
      path: '/chat'
    },
    {
      id: '1',
      element: <SparklesIcon className="w-6 h-6" />,
      path: '/voice'
    }
  ])

  return (
    <GlobalContext.Provider value={{ config, setConfig, chats, setChats }}>
      {children}
    </GlobalContext.Provider>
  )
}

export const useGlobalContext = () => useContext(GlobalContext)
