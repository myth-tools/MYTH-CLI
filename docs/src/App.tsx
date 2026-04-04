import { AlertCircle, RefreshCw } from "lucide-react";
import { Component, lazy, type ReactNode, Suspense } from "react";
import { HashRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./components/Layout";
import { PageSkeleton } from "./components/Skeleton";

// Technical Error Boundary
class ErrorBoundary extends Component<{ children: ReactNode }, { hasError: boolean }> {
	constructor(props: { children: ReactNode }) {
		super(props);
		this.state = { hasError: false };
	}

	static getDerivedStateFromError() {
		return { hasError: true };
	}

	render() {
		if (this.state.hasError) {
			return (
				<div className="min-h-screen bg-cyber-bg flex items-center justify-center p-6 scanline">
					<div className="glass-panel max-w-md w-full p-8 rounded-2xl border border-red-500/30 text-center">
						<div className="w-16 h-16 bg-red-500/10 text-red-500 rounded-full flex items-center justify-center mx-auto mb-6 border border-red-500/20">
							<AlertCircle className="w-8 h-8" />
						</div>
						<h1 className="text-2xl font-bold text-white mb-2 tracking-tight">
							TACTICAL_SYSTEM_FAILURE
						</h1>
						<p className="text-cyber-dim text-sm mb-8">
							A critical exception has occurred in the neural core. Operational parameters have been
							compromised.
						</p>
						<button
							type="button"
							onClick={() => window.location.reload()}
							className="w-full py-3 bg-red-500/20 hover:bg-red-500/30 text-red-500 border border-red-500/30 rounded-xl font-bold transition-all flex items-center justify-center gap-2"
						>
							<RefreshCw className="w-4 h-4" />
							RE-SYNCHRONIZE CORE
						</button>
					</div>
				</div>
			);
		}
		return this.props.children;
	}
}

// Industry-Grade Loading Shimmer
const RouteLoader = () => (
	<>
		<div className="route-loading-bar" />
		<PageSkeleton />
	</>
);

// Lazy load all pages
const ArchitecturePage = lazy(() => import("./pages/ArchitecturePage"));
const BuiltinToolsPage = lazy(() => import("./pages/BuiltinToolsPage"));
const CliCommandsPage = lazy(() => import("./pages/CliCommandsPage"));
const CommandRunnersPage = lazy(() => import("./pages/CommandRunnersPage"));
const ConfigurationPage = lazy(() => import("./pages/ConfigurationPage"));
const CreatorPage = lazy(() => import("./pages/CreatorPage"));
const CustomMcpPage = lazy(() => import("./pages/CustomMcpPage"));
const HomePage = lazy(() => import("./pages/HomePage"));
const InstallationPage = lazy(() => import("./pages/InstallationPage"));
const InteractiveCommandsPage = lazy(() => import("./pages/InteractiveCommandsPage"));
const McpServersPage = lazy(() => import("./pages/McpServersPage"));
const MemoryPage = lazy(() => import("./pages/MemoryPage"));
const ProfilesPage = lazy(() => import("./pages/ProfilesPage"));
const QuickStartPage = lazy(() => import("./pages/QuickStartPage"));
const ScriptsPage = lazy(() => import("./pages/ScriptsPage"));
const SecurityPage = lazy(() => import("./pages/SecurityPage"));
const SubdomainFetchPage = lazy(() => import("./pages/SubdomainFetchPage"));
const TechStackPage = lazy(() => import("./pages/TechStackPage"));
const ToolBridgesPage = lazy(() => import("./pages/ToolBridgesPage"));
const VersionsPage = lazy(() => import("./pages/VersionsPage"));
const VitalsPage = lazy(() => import("./pages/VitalsPage"));
const TypographyPage = lazy(() => import("./pages/TypographyPage"));

function App() {
	return (
		<ErrorBoundary>
			<HashRouter>
				<Layout>
					<Suspense fallback={<RouteLoader />}>
						<Routes>
							<Route path="/" element={<HomePage />} />
							<Route path="/installation" element={<InstallationPage />} />
							<Route path="/versions" element={<VersionsPage />} />
							<Route path="/quickstart" element={<QuickStartPage />} />
							<Route path="/command-runners" element={<CommandRunnersPage />} />
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
							<Route path="/creator" element={<CreatorPage />} />
							<Route path="/subdomain-fetch" element={<SubdomainFetchPage />} />
							<Route path="/typography" element={<TypographyPage />} />
						</Routes>
					</Suspense>
				</Layout>
			</HashRouter>
		</ErrorBoundary>
	);
}

export default App;
