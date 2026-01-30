import { windowClose, windowMinimize } from '@/utils/app'
import { useEffect, useState } from 'react'
import {
  ArrowLeftCircleIcon,
  XMarkIcon,
  MinusIcon,
  MoonIcon,
  SunIcon
} from '@heroicons/react/24/outline'

import { invoke } from '@tauri-apps/api/core'

type HeaderProps = {
  isOpen: boolean
  onToggleDrawer: () => void
}

type SysConfig = {
  theme?: string
  current_app?: string
}

const Header = ({ isOpen, onToggleDrawer }: HeaderProps) => {
  const [sysConfig, setSysConfig] = useState<SysConfig>({})
  const handleThemeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSysConfig({
      ...sysConfig,
      theme: e.target.checked ? 'dark' : 'light'
    })
  }

  const getSysConfig = () => {
    invoke('get_sys_config')
      .then(res => {
        setSysConfig(res as SysConfig)
      })
      .catch(err => console.log(err))
  }

  useEffect(() => {
    getSysConfig()
  }, [])

  useEffect(() => {
    const { theme } = sysConfig
    if (theme) {
      document.documentElement.setAttribute('data-theme', theme)
    }
  }, [sysConfig])

  return (
    <header
      data-tauri-drag-region
      onContextMenu={e => e.preventDefault()}
      className="navbar px-4 py-2 w-full flex items-center justify-between bg-base-100 border-b-2 border-base-300"
    >
      <button
        className={`btn btn-circle btn-outline btn-primary border-none `}
        onClick={onToggleDrawer}
      >
        <ArrowLeftCircleIcon
          className={`w-8 h-8 transition-all duration-500 ease-in-out ${isOpen ? 'rotate-180' : ''}`}
        />
      </button>
      <div className="flex items-center justify-between">
        <div className="avatar mr-12">
          <div className="ring-neutral ring-offset-base-100 w-8 rounded-full ring-1 ring-offset-1 bg-primary-content"></div>
        </div>
        <div className="flex items-center justify-end">
          <label className="swap swap-rotate mr-2">
            <input
              type="checkbox"
              value="dark"
              onChange={handleThemeChange}
              className="theme-controller"
            />
            <MoonIcon className="w-5 h-5 swap-off" />
            <SunIcon className="w-5 h-5 swap-on" />
          </label>

          <button
            className="w-6 h-6 btn btn-ghost btn-circle mr-2"
            onClick={() => windowMinimize()}
          >
            <MinusIcon className="w-5 h-5" />
          </button>
          <button
            className="w-6 h-6 btn btn-ghost btn-circle "
            onClick={() => windowClose()}
          >
            <XMarkIcon className="w-5 h-5" />
          </button>
        </div>
      </div>
    </header>
  )
}

export default Header
