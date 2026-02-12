import { createContext, useContext, useState } from 'react'

export type SysConfig = {
  theme?: string
  currentApp?: string
}

interface GlobalProviderProps {
  children: React.ReactNode
}

const GlobalContext = createContext({
  config: {},
  setConfig: (config: SysConfig) => {}
})

export const GlobalProvider = ({ children }: GlobalProviderProps) => {
  const [config, setConfig] = useState<SysConfig>({})

  return (
    <GlobalContext.Provider value={{ config, setConfig }}>
      {children}
    </GlobalContext.Provider>
  )
}

export const useGlobalContext = () => useContext(GlobalContext)
