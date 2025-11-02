// ui/src/App.tsx
// Main React component for Health & Speed Checker

import { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import {
  Shield,
  Zap,
  AlertTriangle,
  CheckCircle,
  Info,
  Play,
  Download,
  Settings,
  X,
  ChevronRight,
  Activity,
  ScrollText
} from 'lucide-react';
import './App.css';
import { QuickActions } from './components/QuickActions';
import { ExportDialog } from './components/ExportDialog';
import { TrendsChart } from './components/TrendsChart';
import AutomationPage from './components/AutomationPage';
import BottleneckReport from './components/BottleneckReport';
import ChangelogPage from './components/ChangelogPage';
import LicenseDialog from './components/LicenseDialog';
import { useKeyboardShortcuts, useShortcutsModal, KeyboardShortcutsModal } from './hooks/useKeyboardShortcuts';

interface ScanResult {
  scan_id: string;
  timestamp: number;
  duration_ms: number;
  scores: {
    health: number;
    speed: number;
    health_delta?: number;
    speed_delta?: number;
  };
  issues: Issue[];
  details: any;
}

interface Issue {
  id: string;
  severity: 'Critical' | 'Warning' | 'Info';
  title: string;
  description: string;
  impact_category: string;
  fix?: {
    action_id: string;
    label: string;
    is_auto_fix: boolean;
    params?: Record<string, unknown>;
  };
}

interface ProgressEvent {
  type: string;
  payload: any;
}

function App() {
  const [scanning, setScanning] = useState(false);
  const [progress, setProgress] = useState(0);
  const [progressMessage, setProgressMessage] = useState('');
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [ignoredIssues, setIgnoredIssues] = useState<Set<string>>(new Set());
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [scanHistory, setScanHistory] = useState<Array<{scan_id?: string; timestamp: number; health: number; speed: number}>>([]);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [fixingIssueId, setFixingIssueId] = useState<string | null>(null);
  const [showLicenseDialog, setShowLicenseDialog] = useState(false);
  const [activePage, setActivePage] = useState<'dashboard' | 'automation' | 'changelog'>('dashboard');

  const navButtonClass = (page: 'dashboard' | 'automation' | 'changelog') =>
    `flex items-center space-x-2 px-3 py-2 rounded-lg transition-colors ${
      activePage === page ? 'bg-blue-600 text-white' : 'text-gray-300 hover:text-white hover:bg-gray-800'
    }`;

  const bottleneckIssues = useMemo(
    () => scanResult?.issues.filter((issue) => issue.id.startsWith('bottleneck_')) ?? [],
    [scanResult]
  );

  const visibleIssues = useMemo(
    () =>
      scanResult?.issues.filter(
        (issue) => !issue.id.startsWith('bottleneck_') && !ignoredIssues.has(issue.id)
      ) ?? [],
    [scanResult, ignoredIssues]
  );

  // Auto-dismiss messages after 5 seconds
  useEffect(() => {
    if (errorMessage) {
      const timer = setTimeout(() => setErrorMessage(null), 5000);
      return () => clearTimeout(timer);
    }
  }, [errorMessage]);

  useEffect(() => {
    if (successMessage) {
      const timer = setTimeout(() => setSuccessMessage(null), 5000);
      return () => clearTimeout(timer);
    }
  }, [successMessage]);

  // Load scan history on startup
  useEffect(() => {
    const loadHistory = async () => {
      try {
        const history = await invoke<Array<{ scan_id: string; timestamp: number; health_score: number; speed_score: number }>>('get_scan_history');
        setScanHistory(
          history.map(item => ({
            scan_id: item.scan_id,
            timestamp: item.timestamp,
            health: item.health_score,
            speed: item.speed_score,
          }))
        );
      } catch (error) {
        console.error('Failed to load scan history', error);
      }
    };

    loadHistory();
  }, []);

  // Listen for tray events
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setup = async () => {
      try {
        unlisten = await listen('tray-action', (event) => {
          console.log('Tray action:', event.payload);
        });
      } catch (error) {
        console.error('Failed to listen for tray actions', error);
      }
    };

    setup();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  // Initialize keyboard shortcuts
  const shortcuts = useKeyboardShortcuts({
    scan: () => !scanning && startScan(false),
    quickScan: () => !scanning && startScan(true),
    fix: () => {
      const topIssue = visibleIssues[0];
      if (topIssue?.fix) {
        fixIssue(topIssue.fix.action_id, {}, topIssue.id, topIssue.fix.is_auto_fix);
      }
    },
    export: () => {
      if (scanResult) {
        void openExportDialog();
      }
    },
    cancel: () => {
      setShowExportDialog(false);
    },
  });

  // Shortcuts modal
  const { isVisible: showShortcutsModal, close: closeShortcutsModal } = useShortcutsModal();

  const checkFeatureAccess = async (feature: string) => {
    try {
      return await invoke<boolean>('check_feature_access', { feature_name: feature });
    } catch (error) {
      console.error(`Failed to check feature access for ${feature}`, error);
      return false;
    }
  };

  const requireFeature = async (feature: string) => {
    const allowed = await checkFeatureAccess(feature);
    if (!allowed) {
      setShowLicenseDialog(true);
    }
    return allowed;
  };

  const requireAnyFeature = async (features: string[]) => {
    const results = await Promise.all(features.map(checkFeatureAccess));
    if (results.some(Boolean)) {
      return true;
    }
    setShowLicenseDialog(true);
    return false;
  };

  const openExportDialog = async () => {
    const allowed = await requireAnyFeature(['export_pdf', 'export_csv']);
    if (!allowed) {
      return;
    }
    setShowExportDialog(true);
  };

  // Start a scan
  const startScan = async (quick: boolean = false) => {
    setScanning(true);
    setProgress(0);
    setProgressMessage('Starting scan...');

    try {
      const scanId = await invoke<string>('scan_start', {
        options: {
          security: true,
          performance: true,
          quick,
          exclude_apps: quick,
          exclude_startup: quick,
        },
      });

      // Simulate progress (in real implementation, listen to events)
      const progressInterval = setInterval(() => {
        setProgress((prev) => {
          if (prev >= 95) {
            clearInterval(progressInterval);
            return prev;
          }
          return prev + 5;
        });
      }, 500);

      // Simulate different scan stages
      setTimeout(() => setProgressMessage('Checking security...'), 1000);
      setTimeout(() => setProgressMessage('Analyzing performance...'), 2000);
      setTimeout(() => setProgressMessage('Scanning processes...'), 3000);
      setTimeout(() => setProgressMessage('Calculating scores...'), 4000);

      // Get result after delay (simulated)
      setTimeout(async () => {
        const result = await invoke<ScanResult>('get_scan_result', { scanId });
        setScanResult(result);

        // Add to history
        setScanHistory(prev => [...prev, {
          scan_id: result.scan_id,
          timestamp: result.timestamp,
          health: result.scores.health,
          speed: result.scores.speed,
        }].slice(-30)); // Keep last 30 scans

        setScanning(false);
        setProgress(100);
        clearInterval(progressInterval);
        setProgressMessage('Scan complete!');
      }, 5000);
    } catch (error) {
      setScanning(false);
      setProgressMessage('Scan failed');
      setErrorMessage(error instanceof Error ? error.message : 'Failed to start scan. Please try again.');
    }
  };

  // Fix an issue
  const fixIssue = async (actionId: string, params: any, issueId?: string, isAutoFix: boolean = false) => {
    if (isAutoFix) {
      const allowed = await requireFeature('auto_fix');
      if (!allowed) {
        return;
      }
    }

    const confirmed = window.confirm('Are you sure you want to apply this fix?');
    if (!confirmed) {
      return;
    }

    if (issueId) setFixingIssueId(issueId);
    try {
      const payload = { ...params, confirm: true };
      const result = await invoke<{success: boolean, message: string}>('fix_action', { actionId, params: payload });
      if (result.success) {
        setSuccessMessage(result.message);
        // Refresh scan after fix
        startScan(true);
      } else {
        setErrorMessage(result.message);
      }
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to apply fix. Please try manually.');
    } finally {
      if (issueId) setFixingIssueId(null);
    }
  };

  // Ignore an issue
  const ignoreIssue = (issueId: string) => {
    setIgnoredIssues(new Set([...ignoredIssues, issueId]));
  };

  // Get score color
  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-500';
    if (score >= 60) return 'text-yellow-500';
    return 'text-red-500';
  };

  // Get severity icon
  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'Critical':
        return <AlertTriangle className="w-5 h-5 text-red-500" />;
      case 'Warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-500" />;
      case 'Info':
        return <Info className="w-5 h-5 text-blue-500" />;
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gray-950 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Activity className="w-8 h-8 text-blue-500" />
            <h1 className="text-2xl font-bold">Health & Speed Checker</h1>
          </div>
          <div className="flex items-center space-x-3">
            <div className="flex items-center bg-gray-900/60 border border-gray-800 rounded-lg p-1 gap-1">
              <button
                onClick={() => setActivePage('dashboard')}
                className={navButtonClass('dashboard')}
              >
                <Shield className="w-4 h-4" />
                <span className="text-sm font-medium">Dashboard</span>
              </button>
              <button
                onClick={() => setActivePage('automation')}
                className={navButtonClass('automation')}
              >
                <Settings className="w-4 h-4" />
                <span className="text-sm font-medium">Automation</span>
              </button>
              <button
                onClick={() => setActivePage('changelog')}
                className={navButtonClass('changelog')}
              >
                <ScrollText className="w-4 h-4" />
                <span className="text-sm font-medium">Changelog</span>
              </button>
            </div>
            <button
              onClick={() => setShowLicenseDialog(true)}
              className="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 rounded-lg transition-colors text-sm font-semibold"
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-6">
        {activePage === 'automation' ? (
          <AutomationPage onUpgrade={() => setShowLicenseDialog(true)} />
        ) : activePage === 'changelog' ? (
          <ChangelogPage />
        ) : (
          <>
            {!scanning && !scanResult && (
              <div className="max-w-4xl mx-auto">
                {/* Welcome Screen */}
                <div className="text-center py-16">
                  <div className="flex justify-center space-x-8 mb-12">
                    <div className="text-center">
                      <Shield className="w-16 h-16 mx-auto mb-4 text-blue-500" />
                      <h3 className="text-xl font-semibold">Security Check</h3>
                      <p className="text-gray-400 mt-2">Scan for vulnerabilities</p>
                    </div>
                    <div className="text-center">
                      <Zap className="w-16 h-16 mx-auto mb-4 text-yellow-500" />
                      <h3 className="text-xl font-semibold">Speed Analysis</h3>
                      <p className="text-gray-400 mt-2">Find performance issues</p>
                    </div>
                  </div>

                  <div className="flex justify-center space-x-4">
                    <button
                      onClick={() => startScan(false)}
                      className="flex items-center space-x-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
                    >
                      <Play className="w-5 h-5" />
                      <span>Full Scan</span>
                    </button>
                    <button
                      onClick={() => startScan(true)}
                      className="flex items-center space-x-2 px-6 py-3 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                      <Zap className="w-5 h-5" />
                      <span>Quick Scan (5s)</span>
                    </button>
                  </div>
                </div>
              </div>
            )}

            {scanning && (
              <div className="max-w-2xl mx-auto py-16">
                {/* Progress Screen */}
                <div className="text-center">
                  <h2 className="text-2xl font-bold mb-8">Scanning Your System...</h2>

                  <div className="mb-4">
                    <div className="bg-gray-800 rounded-full h-4 overflow-hidden">
                      <div
                        className="bg-gradient-to-r from-blue-500 to-blue-600 h-full transition-all duration-500"
                        style={{ width: `${progress}%` }}
                      />
                    </div>
                  </div>

                  <p className="text-gray-400">{progressMessage}</p>
                  <p className="text-sm text-gray-500 mt-2">{progress}%</p>
                </div>
              </div>
            )}

            {scanResult && !scanning && (
              <div className="max-w-6xl mx-auto">
                {bottleneckIssues.length > 0 && (
                  <div className="mb-10">
                    {bottleneckIssues.map((issue) => (
                      <BottleneckReport
                        key={issue.id}
                        issue={issue}
                        onFix={(fix) => fixIssue(fix.action_id, fix.params ?? {}, issue.id, fix.is_auto_fix)}
                      />
                    ))}
                  </div>
                )}

                {/* Results Screen */}
                <div className="mb-8 grid grid-cols-1 gap-6 lg:grid-cols-2">
                  {/* Health Score Card */}
                  <div className="rounded-xl border border-gray-800 bg-gray-900 p-6">
                    <div className="mb-4 flex items-center justify-between">
                      <h3 className="flex items-center text-lg font-semibold">
                        <Shield className="mr-2 h-5 w-5 text-blue-500" />
                        Health Score
                      </h3>
                      {scanResult.scores.health_delta && (
                        <span className={scanResult.scores.health_delta > 0 ? 'text-green-500' : 'text-red-500'}>
                          {scanResult.scores.health_delta > 0 ? '+' : '-'}
                          {Math.abs(scanResult.scores.health_delta)}
                        </span>
                      )}
                    </div>
                    <div className="text-center">
                      <div className={`text-5xl font-bold ${getScoreColor(scanResult.scores.health)}`}>
                        {scanResult.scores.health}
                      </div>
                      <div className="mt-2 text-gray-400">out of 100</div>
                    </div>
                  </div>

                  {/* Speed Score Card */}
                  <div className="rounded-xl border border-gray-800 bg-gray-900 p-6">
                    <div className="mb-4 flex items-center justify-between">
                      <h3 className="flex items-center text-lg font-semibold">
                        <Zap className="mr-2 h-5 w-5 text-yellow-500" />
                        Speed Score
                      </h3>
                      {scanResult.scores.speed_delta && (
                        <span className={scanResult.scores.speed_delta > 0 ? 'text-green-500' : 'text-red-500'}>
                          {scanResult.scores.speed_delta > 0 ? '+' : '-'}
                          {Math.abs(scanResult.scores.speed_delta)}
                        </span>
                      )}
                    </div>
                    <div className="text-center">
                      <div className={`text-5xl font-bold ${getScoreColor(scanResult.scores.speed)}`}>
                        {scanResult.scores.speed}
                      </div>
                      <div className="mt-2 text-gray-400">out of 100</div>
                    </div>
                  </div>
                </div>

                {/* Issues List */}
                <div className="space-y-4">
                  <h3 className="mb-4 text-xl font-semibold">
                    Top Issues ({visibleIssues.length})
                  </h3>

                  {visibleIssues.slice(0, 5).map((issue) => (
                    <div
                      key={issue.id}
                      className="rounded-lg border border-gray-800 bg-gray-900 p-4"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <div className="mb-2 flex items-center space-x-2">
                            {getSeverityIcon(issue.severity)}
                            <span className="font-semibold">{issue.title}</span>
                          </div>
                          <p className="mb-3 text-sm text-gray-400">
                            {issue.description}
                          </p>
                          <div className="flex items-center space-x-3">
                            {issue.fix && (
                              <button
                                onClick={() => fixIssue(issue.fix!.action_id, {}, issue.id, issue.fix.is_auto_fix)}
                                disabled={fixingIssueId === issue.id}
                                className="flex items-center space-x-1 rounded bg-blue-600 px-3 py-1 text-sm transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-600"
                              >
                                {fixingIssueId === issue.id ? (
                                  <>
                                    <div className="spinner h-4 w-4" />
                                    <span>Fixing...</span>
                                  </>
                                ) : (
                                  <>
                                    <CheckCircle className="h-4 w-4" />
                                    <span>{issue.fix.label}</span>
                                  </>
                                )}
                              </button>
                            )}
                            <button
                              onClick={() => ignoreIssue(issue.id)}
                              className="text-sm text-gray-400 transition-colors hover:text-white"
                            >
                              Ignore
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}

                  {visibleIssues.length > 5 && (
                    <p className="py-4 text-center text-gray-400">
                      And {visibleIssues.length - 5} more issues...
                    </p>
                  )}
                </div>

                {/* Action Buttons */}
                <div className="mt-8 flex justify-center space-x-4">
                  <button
                    onClick={() => startScan(false)}
                    className="flex items-center space-x-2 rounded-lg bg-gray-800 px-4 py-2 transition-colors hover:bg-gray-700"
                  >
                    <Play className="h-4 w-4" />
                    <span>Scan Again</span>
                  </button>
                  <button
                    onClick={() => {
                      if (scanResult) {
                        void openExportDialog();
                      }
                    }}
                    className="flex items-center space-x-2 rounded-lg bg-gray-800 px-4 py-2 transition-colors hover:bg-gray-700"
                  >
                    <Download className="h-4 w-4" />
                    <span>Export Report</span>
                  </button>
                </div>

                {/* Trends Chart */}
                {scanHistory.length > 0 && (
                  <div className="mt-8">
                    <h3 className="mb-4 text-xl font-semibold">Historical Trends</h3>
                    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                      <TrendsChart data={scanHistory} type="health" />
                      <TrendsChart data={scanHistory} type="speed" />
                    </div>
                  </div>
                )}
              </div>
            )}
          </>
        )}
      </main>

      {/* Quick Actions Widget */}
      {activePage === 'dashboard' && !scanning && (
        <QuickActions
          onScanQuick={() => startScan(true)}
          onScanFull={() => startScan(false)}
          onFixTop={() => {
            const topIssue = visibleIssues[0];
            if (topIssue?.fix) {
              fixIssue(topIssue.fix.action_id, topIssue.fix.params ?? {}, topIssue.id, topIssue.fix.is_auto_fix);
            }
          }}
          onExport={() => {
            if (scanResult) {
              void openExportDialog();
            }
          }}
          onHistory={() => setSuccessMessage('Showing latest scan history below.')}
          isScanning={scanning}
          healthScore={scanResult?.scores.health}
        />
      )}

      {/* Export Dialog */}
      {showExportDialog && scanResult && (
        <ExportDialog
          scanId={scanResult.scan_id}
          onClose={() => setShowExportDialog(false)}
          onSuccess={(message) => setSuccessMessage(message)}
          onError={(message) => setErrorMessage(message)}
        />
      )}

      {/* Keyboard Shortcuts Modal */}
      {showShortcutsModal && (
        <KeyboardShortcutsModal
          shortcuts={shortcuts}
          onClose={closeShortcutsModal}
        />
      )}

      {/* Toast Notifications */}
      {errorMessage && (
        <div className="fixed bottom-4 right-4 bg-red-600 text-white px-6 py-4 rounded-lg shadow-lg flex items-center space-x-3 max-w-md notification-slide z-50">
          <AlertTriangle className="w-5 h-5 flex-shrink-0" />
          <p className="flex-1">{errorMessage}</p>
          <button onClick={() => setErrorMessage(null)} className="text-white hover:text-gray-200">
            <X className="w-5 h-5" />
          </button>
        </div>
      )}

      {successMessage && (
        <div className="fixed bottom-4 right-4 bg-green-600 text-white px-6 py-4 rounded-lg shadow-lg flex items-center space-x-3 max-w-md notification-slide z-50">
          <CheckCircle className="w-5 h-5 flex-shrink-0" />
          <p className="flex-1">{successMessage}</p>
          <button onClick={() => setSuccessMessage(null)} className="text-white hover:text-gray-200">
            <X className="w-5 h-5" />
          </button>
        </div>
      )}

      <LicenseDialog
        isOpen={showLicenseDialog}
        onClose={() => setShowLicenseDialog(false)}
      />
    </div>
  );
}

export default App;
