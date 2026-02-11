import { RouterProvider } from 'react-router-dom'
import router from './router'
import { GlobalProvider } from './context/GlobalContext'

import './tailwind.css'
import './App.scss'

const App = () => {
  return (
    <GlobalProvider>
      <RouterProvider router={router} />
    </GlobalProvider>
  )
}

export default App
