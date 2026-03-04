import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />

      {/*<BrowserRouter>*/}
      {/*    <Routes>*/}
      {/*        <Route path="/" element={<App />}>*/}
      {/*            <Route index element={<Home />} />*/}
      {/*            <Route path="about" element={<About />} />*/}
      {/*        </Route>*/}
      {/*    </Routes>*/}
      {/*</BrowserRouter>*/}

  </StrictMode>,
)
