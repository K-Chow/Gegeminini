import { useState } from 'react'
import { Outlet } from 'react-router-dom'

const Home = () => {
  const [isDrawerOpen, setDrawerOpen] = useState(false)
  return (
    <section className="drawer drawer-open">
      <input
        id="nav-drawer"
        type="checkbox"
        className="drawer-toggle"
        checked={isDrawerOpen}
        onChange={() => setDrawerOpen(!isDrawerOpen)}
      />
      <div className="drawer-content">
        <div className="w-full h-(--spacing-main-content) bg-base-100 text-base-content">
          <Outlet />
        </div>
      </div>
    </section>
  )
}

export default Home
