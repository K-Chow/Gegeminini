import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
export const windowMinimize = () => appWindow.minimize()

export const windowClose = () => appWindow.close()
