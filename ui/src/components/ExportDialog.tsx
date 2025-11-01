import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { writeBinaryFile, writeTextFile } from '@tauri-apps/api/fs';
import './ExportDialog.css';

interface ExportDialogProps {
  scanId: string;
  onClose: () => void;
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}

type ExportFormat = 'json' | 'pdf' | 'html' | 'csv';

export function ExportDialog({ scanId, onClose, onSuccess, onError }: ExportDialogProps) {
  const [format, setFormat] = useState<ExportFormat>('json');
  const [isExporting, setIsExporting] = useState(false);
  const [includeCharts, setIncludeCharts] = useState(true);
  const [includeHistory, setIncludeHistory] = useState(false);

  const formatInfo = {
    json: {
      icon: '📋',
      title: 'JSON',
      description: 'Machine-readable format, great for automation',
      extension: 'json',
    },
    pdf: {
      icon: '📄',
      title: 'PDF',
      description: 'Professional report, ready to share',
      extension: 'pdf',
    },
    html: {
      icon: '🌐',
      title: 'HTML',
      description: 'Interactive web page, view in browser',
      extension: 'html',
    },
    csv: {
      icon: '📊',
      title: 'CSV',
      description: 'Spreadsheet format, open in Excel',
      extension: 'csv',
    },
  };

  const handleExport = async () => {
    setIsExporting(true);

    try {
      // Ask user where to save
      const filePath = await save({
        defaultPath: `health-report-${scanId}.${formatInfo[format].extension}`,
        filters: [{
          name: formatInfo[format].title,
          extensions: [formatInfo[format].extension],
        }],
      });

      if (!filePath) {
        setIsExporting(false);
        return;
      }

      // Generate report via Tauri backend
      const reportData = await invoke<string>('export_report', {
        scanId,
        format,
        options: {
          includeCharts,
          includeHistory,
        },
      });

      // Write to file
      if (format === 'pdf') {
        // PDF is binary
        const binaryData = new Uint8Array(
          atob(reportData)
            .split('')
            .map(char => char.charCodeAt(0))
        );
        await writeBinaryFile(filePath, binaryData);
      } else {
        // JSON, HTML, CSV are text
        await writeTextFile(filePath, reportData);
      }

      // Show success message
      onSuccess?.(`Report exported successfully to: ${filePath}`);
      onClose();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      onError?.(`Failed to export report: ${errorMessage}`);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="export-dialog-overlay" onClick={onClose}>
      <div className="export-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="export-dialog-header">
          <h2>💾 Export Report</h2>
          <button className="close-button" onClick={onClose}>×</button>
        </div>

        <div className="export-dialog-content">
          {/* Format Selection */}
          <div className="format-selection">
            <h3>Choose Format</h3>
            <div className="format-grid">
              {(Object.keys(formatInfo) as ExportFormat[]).map((fmt) => (
                <button
                  key={fmt}
                  className={`format-card ${format === fmt ? 'selected' : ''}`}
                  onClick={() => setFormat(fmt)}
                >
                  <div className="format-icon">{formatInfo[fmt].icon}</div>
                  <div className="format-title">{formatInfo[fmt].title}</div>
                  <div className="format-description">{formatInfo[fmt].description}</div>
                </button>
              ))}
            </div>
          </div>

          {/* Options */}
          <div className="export-options">
            <h3>Options</h3>

            <label className="checkbox-option">
              <input
                type="checkbox"
                checked={includeCharts}
                onChange={(e) => setIncludeCharts(e.target.checked)}
                disabled={format === 'csv' || format === 'json'}
              />
              <span>Include charts and visualizations</span>
            </label>

            <label className="checkbox-option">
              <input
                type="checkbox"
                checked={includeHistory}
                onChange={(e) => setIncludeHistory(e.target.checked)}
              />
              <span>Include historical data</span>
            </label>
          </div>

          {/* Preview */}
          <div className="export-preview">
            <h3>What's Included</h3>
            <ul>
              <li>✓ Current health and speed scores</li>
              <li>✓ All detected issues with severity</li>
              <li>✓ System information</li>
              <li>✓ Scan timestamp</li>
              {includeCharts && format !== 'json' && format !== 'csv' && (
                <li>✓ Charts and graphs</li>
              )}
              {includeHistory && (
                <li>✓ Historical trends (last 30 days)</li>
              )}
            </ul>
          </div>
        </div>

        <div className="export-dialog-footer">
          <button className="btn-secondary" onClick={onClose} disabled={isExporting}>
            Cancel
          </button>
          <button
            className="btn-primary"
            onClick={handleExport}
            disabled={isExporting}
          >
            {isExporting ? (
              <>
                <span className="spinner-small"></span>
                Exporting...
              </>
            ) : (
              <>Export {formatInfo[format].title}</>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
