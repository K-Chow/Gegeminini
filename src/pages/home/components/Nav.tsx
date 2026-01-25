import { Link, useLocation } from 'react-router-dom'

type NavProps = {
  isOpen: boolean
  onClose: () => void
}

const Nav = ({ onClose }: NavProps) => {
  const { pathname } = useLocation()
  const urlPath = pathname.split('/').filter(Boolean)
  const currentPage = urlPath && urlPath[urlPath.length - 1]
  return (
    <div className="drawer-side is-drawer-close:overflow-visible shadow-2xl border-r-1 border-base-300">
      <label className="drawer-overlay" onClick={() => onClose()} />
      <div className="flex min-h-full flex-col items-start bg-base-100 text-base-content is-drawer-close:w-16 is-drawer-open:w-48">
        <div className="w-full h-18" />
        <ul className="menu w-full grow font-bold"></ul>
      </div>
    </div>
  )
}

export default Nav
