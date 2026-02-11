import { createContext, useContext, useState } from 'react'

export type SysConfig = {
  theme?: string
  currentApp?: string
}

interface GlobalProviderProps {
  children: React.ReactNode
}

const GlobalContext = createContext({
  sysConfig: {},
  setSysConfig: (config: SysConfig) => {}
})

export const GlobalProvider = ({ children }: GlobalProviderProps) => {
  const [sysConfig, setSysConfig] = useState<SysConfig>({})

  return (
    <GlobalContext.Provider value={{ sysConfig, setSysConfig }}>
      {children}
    </GlobalContext.Provider>
  )
}

export const useGlobalContext = () => useContext(GlobalContext)
