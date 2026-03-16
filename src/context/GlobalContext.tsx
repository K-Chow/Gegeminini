import { createContext, useContext, useState } from 'react'

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
  title: string
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
      title: '',
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
      title: 'Chat',
      path: '/chat'
    }
  ])

  return (
    <GlobalContext.Provider value={{ config, setConfig, chats, setChats }}>
      {children}
    </GlobalContext.Provider>
  )
}

export const useGlobalContext = () => useContext(GlobalContext)
