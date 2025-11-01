import { useState } from 'react';
import './QuickActions.css';

interface QuickActionsProps {
  onScanQuick: () => void;
  onScanFull: () => void;
  onFixTop: () => void;
  onExport: () => void;
  onHistory: () => void;
  isScanning?: boolean;
  healthScore?: number;
}

export function QuickActions({
  onScanQuick,
  onScanFull,
  onFixTop,
  onExport,
  onHistory,
  isScanning = false,
  healthScore,
}: QuickActionsProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [position, setPosition] = useState({ x: window.innerWidth - 80, y: window.innerHeight - 80 });
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });

  const handleMouseDown = (e: React.MouseEvent) => {
    if ((e.target as HTMLElement).closest('.quick-action-button')) {
      return; // Don't drag when clicking buttons
    }
    setIsDragging(true);
    setDragOffset({
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    });
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging) {
      setPosition({
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  // Add/remove mouse move listeners
  React.useEffect(() => {
    if (isDragging) {
      window.addEventListener('mousemove', handleMouseMove);
      window.addEventListener('mouseup', handleMouseUp);
    } else {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    }

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging]);

  const getHealthColor = () => {
    if (!healthScore) return '#666';
    if (healthScore >= 80) return '#22c55e';
    if (healthScore >= 60) return '#eab308';
    return '#ef4444';
  };

  return (
    <div
      className={`quick-actions-widget ${isExpanded ? 'expanded' : ''} ${isDragging ? 'dragging' : ''}`}
      style={{
        left: `${position.x}px`,
        top: `${position.y}px`,
      }}
      onMouseDown={handleMouseDown}
    >
      {/* Main Action Button */}
      <button
        className="quick-actions-main"
        onClick={() => setIsExpanded(!isExpanded)}
        style={{
          background: `conic-gradient(${getHealthColor()} ${healthScore || 0}%, #2a2a2a 0%)`,
        }}
        title="Quick Actions"
      >
        <div className="main-icon">
          {isScanning ? (
            <div className="spinner-icon">⟳</div>
          ) : (
            <span className="health-score">{healthScore || '?'}</span>
          )}
        </div>
      </button>

      {/* Expanded Action Buttons */}
      {isExpanded && (
        <div className="quick-actions-menu">
          <button
            className="quick-action-button scan-quick"
            onClick={(e) => {
              e.stopPropagation();
              onScanQuick();
            }}
            disabled={isScanning}
            title="Quick Scan (5 seconds)"
          >
            <span className="icon">⚡</span>
            <span className="label">Quick</span>
          </button>

          <button
            className="quick-action-button scan-full"
            onClick={(e) => {
              e.stopPropagation();
              onScanFull();
            }}
            disabled={isScanning}
            title="Full Scan"
          >
            <span className="icon">🔍</span>
            <span className="label">Full</span>
          </button>

          <button
            className="quick-action-button fix"
            onClick={(e) => {
              e.stopPropagation();
              onFixTop();
            }}
            disabled={isScanning}
            title="Fix Top Issue"
          >
            <span className="icon">🔧</span>
            <span className="label">Fix</span>
          </button>

          <button
            className="quick-action-button history"
            onClick={(e) => {
              e.stopPropagation();
              onHistory();
            }}
            title="View History"
          >
            <span className="icon">📊</span>
            <span className="label">History</span>
          </button>

          <button
            className="quick-action-button export"
            onClick={(e) => {
              e.stopPropagation();
              onExport();
            }}
            title="Export Report"
          >
            <span className="icon">💾</span>
            <span className="label">Export</span>
          </button>
        </div>
      )}
    </div>
  );
}
