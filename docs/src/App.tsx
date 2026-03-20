import { HashRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import HomePage from './pages/HomePage';
import InstallationPage from './pages/InstallationPage';
import QuickStartPage from './pages/QuickStartPage';
import ConfigurationPage from './pages/ConfigurationPage';
import ArchitecturePage from './pages/ArchitecturePage';
import SecurityPage from './pages/SecurityPage';
import MemoryPage from './pages/MemoryPage';
import CliCommandsPage from './pages/CliCommandsPage';
import InteractiveCommandsPage from './pages/InteractiveCommandsPage';
import ProfilesPage from './pages/ProfilesPage';
import McpServersPage from './pages/McpServersPage';
import BuiltinToolsPage from './pages/BuiltinToolsPage';
import ToolBridgesPage from './pages/ToolBridgesPage';
import CustomMcpPage from './pages/CustomMcpPage';
import ScriptsPage from './pages/ScriptsPage';
import TechStackPage from './pages/TechStackPage';
import VitalsPage from './pages/VitalsPage';
import SubdomainFetchPage from './pages/SubdomainFetchPage';
import VersionsPage from './pages/VersionsPage';

function App() {
  return (
    <HashRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/installation" element={<InstallationPage />} />
          <Route path="/versions" element={<VersionsPage />} />
          <Route path="/quickstart" element={<QuickStartPage />} />
          <Route path="/configuration" element={<ConfigurationPage />} />
          <Route path="/vitals" element={<VitalsPage />} />
          <Route path="/architecture" element={<ArchitecturePage />} />
          <Route path="/security" element={<SecurityPage />} />
          <Route path="/memory" element={<MemoryPage />} />
          <Route path="/cli-commands" element={<CliCommandsPage />} />
          <Route path="/interactive-commands" element={<InteractiveCommandsPage />} />
          <Route path="/profiles" element={<ProfilesPage />} />
          <Route path="/mcp-servers" element={<McpServersPage />} />
          <Route path="/builtin-tools" element={<BuiltinToolsPage />} />
          <Route path="/tool-bridges" element={<ToolBridgesPage />} />
          <Route path="/custom-mcp" element={<CustomMcpPage />} />
          <Route path="/scripts" element={<ScriptsPage />} />
          <Route path="/tech-stack" element={<TechStackPage />} />
          <Route path="/subdomain-fetch" element={<SubdomainFetchPage />} />
        </Routes>
      </Layout>
    </HashRouter>
  );
}

export default App;
