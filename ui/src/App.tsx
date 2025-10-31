// ui/src/App.tsx
// Main React component for Health & Speed Checker

import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
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
  Activity
} from 'lucide-react';
import './App.css';

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
  const [currentTab, setCurrentTab] = useState<'overview' | 'security' | 'performance'>('overview');
  const [ignoredIssues, setIgnoredIssues] = useState<Set<string>>(new Set());

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
        setScanning(false);
        setProgress(100);
        clearInterval(progressInterval);
        setProgressMessage('Scan complete!');
      }, 5000);
    } catch (error) {
      console.error('Scan failed:', error);
      setScanning(false);
      setProgressMessage('Scan failed');
    }
  };

  // Fix an issue
  const fixIssue = async (actionId: string, params: any) => {
    try {
      const result = await invoke('fix_action', { actionId, params });
      console.log('Fix result:', result);
      // Refresh scan after fix
      startScan(true);
    } catch (error) {
      console.error('Fix failed:', error);
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

  // Filter visible issues
  const visibleIssues = scanResult?.issues.filter(
    (issue) => !ignoredIssues.has(issue.id)
  ) || [];

  return (
    <div className="min-h-screen bg-gray-950 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <Activity className="w-8 h-8 text-blue-500" />
            <h1 className="text-2xl font-bold">Health & Speed Checker</h1>
          </div>
          <button className="p-2 hover:bg-gray-800 rounded-lg transition-colors">
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-6">
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
            {/* Results Screen */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
              {/* Health Score Card */}
              <div className="bg-gray-900 rounded-xl p-6 border border-gray-800">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-semibold flex items-center">
                    <Shield className="w-5 h-5 mr-2 text-blue-500" />
                    Health Score
                  </h3>
                  {scanResult.scores.health_delta && (
                    <span className={scanResult.scores.health_delta > 0 ? 'text-green-500' : 'text-red-500'}>
                      {scanResult.scores.health_delta > 0 ? '↑' : '↓'}
                      {Math.abs(scanResult.scores.health_delta)}
                    </span>
                  )}
                </div>
                <div className="text-center">
                  <div className={`text-5xl font-bold ${getScoreColor(scanResult.scores.health)}`}>
                    {scanResult.scores.health}
                  </div>
                  <div className="text-gray-400 mt-2">out of 100</div>
                </div>
              </div>

              {/* Speed Score Card */}
              <div className="bg-gray-900 rounded-xl p-6 border border-gray-800">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-semibold flex items-center">
                    <Zap className="w-5 h-5 mr-2 text-yellow-500" />
                    Speed Score
                  </h3>
                  {scanResult.scores.speed_delta && (
                    <span className={scanResult.scores.speed_delta > 0 ? 'text-green-500' : 'text-red-500'}>
                      {scanResult.scores.speed_delta > 0 ? '↑' : '↓'}
                      {Math.abs(scanResult.scores.speed_delta)}
                    </span>
                  )}
                </div>
                <div className="text-center">
                  <div className={`text-5xl font-bold ${getScoreColor(scanResult.scores.speed)}`}>
                    {scanResult.scores.speed}
                  </div>
                  <div className="text-gray-400 mt-2">out of 100</div>
                </div>
              </div>
            </div>

            {/* Tabs */}
            <div className="border-b border-gray-800 mb-6">
              <div className="flex space-x-6">
                {['overview', 'security', 'performance'].map((tab) => (
                  <button
                    key={tab}
                    onClick={() => setCurrentTab(tab as any)}
                    className={`pb-3 px-1 border-b-2 transition-colors capitalize ${
                      currentTab === tab
                        ? 'border-blue-500 text-blue-500'
                        : 'border-transparent text-gray-400 hover:text-white'
                    }`}
                  >
                    {tab}
                  </button>
                ))}
              </div>
            </div>

            {/* Issues List */}
            <div className="space-y-4">
              {currentTab === 'overview' && (
                <>
                  <h3 className="text-xl font-semibold mb-4">
                    Top Issues ({visibleIssues.length})
                  </h3>

                  {visibleIssues.slice(0, 5).map((issue) => (
                    <div
                      key={issue.id}
                      className="bg-gray-900 rounded-lg p-4 border border-gray-800"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <div className="flex items-center space-x-2 mb-2">
                            {getSeverityIcon(issue.severity)}
                            <span className="font-semibold">{issue.title}</span>
                          </div>
                          <p className="text-gray-400 text-sm mb-3">
                            {issue.description}
                          </p>
                          <div className="flex items-center space-x-3">
                            {issue.fix && (
                              <button
                                onClick={() => fixIssue(issue.fix!.action_id, {})}
                                className="flex items-center space-x-1 px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm transition-colors"
                              >
                                <CheckCircle className="w-4 h-4" />
                                <span>{issue.fix.label}</span>
                              </button>
                            )}
                            <button
                              onClick={() => ignoreIssue(issue.id)}
                              className="text-gray-400 hover:text-white text-sm"
                            >
                              Ignore
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}

                  {visibleIssues.length > 5 && (
                    <p className="text-center text-gray-400 py-4">
                      And {visibleIssues.length - 5} more issues...
                    </p>
                  )}
                </>
              )}
            </div>

            {/* Action Buttons */}
            <div className="flex justify-center space-x-4 mt-8">
              <button
                onClick={() => startScan(false)}
                className="flex items-center space-x-2 px-4 py-2 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
              >
                <Play className="w-4 h-4" />
                <span>Scan Again</span>
              </button>
              <button className="flex items-center space-x-2 px-4 py-2 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors">
                <Download className="w-4 h-4" />
                <span>Export Report</span>
              </button>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
