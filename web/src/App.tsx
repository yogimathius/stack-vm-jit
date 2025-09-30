import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { VMProvider } from './contexts/VMContext'
import Layout from './components/Layout'
import Playground from './pages/Playground'
import Tutorial from './pages/Tutorial'
import Performance from './pages/Performance'
import Documentation from './pages/Documentation'

function App() {
  return (
    <VMProvider>
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<Playground />} />
            <Route path="/tutorial" element={<Tutorial />} />
            <Route path="/performance" element={<Performance />} />
            <Route path="/docs" element={<Documentation />} />
          </Routes>
        </Layout>
      </Router>
    </VMProvider>
  )
}

export default App