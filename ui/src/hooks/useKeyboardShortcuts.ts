import React, { useEffect, useCallback } from 'react';

export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean; // Command key on Mac
  description: string;
  action: () => void;
}

export interface KeyboardShortcutsConfig {
  scan?: () => void;
  quickScan?: () => void;
  fix?: () => void;
  history?: () => void;
  export?: () => void;
  settings?: () => void;
  help?: () => void;
  cancel?: () => void;
}

export function useKeyboardShortcuts(config: KeyboardShortcutsConfig) {
  const shortcuts: KeyboardShortcut[] = [
    {
      key: 's',
      ctrl: true,
      description: 'Start full scan',
      action: config.scan || (() => {}),
    },
    {
      key: 'q',
      ctrl: true,
      description: 'Start quick scan',
      action: config.quickScan || (() => {}),
    },
    {
      key: 'f',
      ctrl: true,
      description: 'Fix top issue',
      action: config.fix || (() => {}),
    },
    {
      key: 'h',
      ctrl: true,
      description: 'View history',
      action: config.history || (() => {}),
    },
    {
      key: 'e',
      ctrl: true,
      description: 'Export report',
      action: config.export || (() => {}),
    },
    {
      key: ',',
      ctrl: true,
      description: 'Open settings',
      action: config.settings || (() => {}),
    },
    {
      key: '/',
      ctrl: true,
      description: 'Show help',
      action: config.help || (() => {}),
    },
    {
      key: 'Escape',
      description: 'Cancel current action',
      action: config.cancel || (() => {}),
    },
    {
      key: 'F1',
      description: 'Show keyboard shortcuts',
      action: () => {
        // Show shortcuts modal
        const event = new CustomEvent('show-shortcuts-modal');
        window.dispatchEvent(event);
      },
    },
  ];

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      for (const shortcut of shortcuts) {
        const ctrlMatch = shortcut.ctrl ? event.ctrlKey || event.metaKey : !event.ctrlKey && !event.metaKey;
        const shiftMatch = shortcut.shift ? event.shiftKey : !event.shiftKey;
        const altMatch = shortcut.alt ? event.altKey : !event.altKey;

        if (
          event.key.toLowerCase() === shortcut.key.toLowerCase() &&
          ctrlMatch &&
          shiftMatch &&
          altMatch
        ) {
          event.preventDefault();
          event.stopPropagation();
          shortcut.action();
          break;
        }
      }
    },
    [shortcuts]
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleKeyDown]);

  return shortcuts;
}

// Hook to show keyboard shortcuts modal
export function useShortcutsModal() {
  const [isVisible, setIsVisible] = React.useState(false);

  useEffect(() => {
    const handleShow = () => setIsVisible(true);
    window.addEventListener('show-shortcuts-modal', handleShow);

    return () => {
      window.removeEventListener('show-shortcuts-modal', handleShow);
    };
  }, []);

  const close = useCallback(() => setIsVisible(false), []);

  return { isVisible, close };
}

// Keyboard shortcuts modal component
export function KeyboardShortcutsModal({ shortcuts, onClose }: {
  shortcuts: KeyboardShortcut[];
  onClose: () => void;
}) {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleEscape);
    return () => window.removeEventListener('keydown', handleEscape);
  }, [onClose]);

  const formatShortcut = (shortcut: KeyboardShortcut) => {
    const parts: string[] = [];

    if (shortcut.ctrl) parts.push('Ctrl');
    if (shortcut.shift) parts.push('Shift');
    if (shortcut.alt) parts.push('Alt');
    if (shortcut.meta) parts.push('Cmd');

    parts.push(shortcut.key.toUpperCase());

    return parts.join(' + ');
  };

  return (
    <div className="shortcuts-modal-overlay" onClick={onClose}>
      <div className="shortcuts-modal" onClick={(e) => e.stopPropagation()}>
        <div className="shortcuts-modal-header">
          <h2>⌨️ Keyboard Shortcuts</h2>
          <button className="close-button" onClick={onClose}>×</button>
        </div>

        <div className="shortcuts-list">
          {shortcuts.map((shortcut, index) => (
            <div key={index} className="shortcut-item">
              <span className="shortcut-keys">
                {formatShortcut(shortcut)}
              </span>
              <span className="shortcut-description">
                {shortcut.description}
              </span>
            </div>
          ))}
        </div>

        <div className="shortcuts-modal-footer">
          <p>Press <kbd>Esc</kbd> to close</p>
        </div>
      </div>

      <style>{`
        .shortcuts-modal-overlay {
          position: fixed;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: rgba(0, 0, 0, 0.8);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 10000;
          animation: fadeIn 0.2s ease-out;
        }

        .shortcuts-modal {
          background: #1a1a1a;
          border: 1px solid #333;
          border-radius: 12px;
          max-width: 600px;
          width: 90%;
          max-height: 80vh;
          overflow: hidden;
          box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
          animation: slideUp 0.3s ease-out;
        }

        .shortcuts-modal-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 20px;
          border-bottom: 1px solid #333;
        }

        .shortcuts-modal-header h2 {
          margin: 0;
          font-size: 24px;
          color: #fff;
        }

        .close-button {
          background: none;
          border: none;
          font-size: 32px;
          color: #999;
          cursor: pointer;
          padding: 0;
          width: 32px;
          height: 32px;
          display: flex;
          align-items: center;
          justify-content: center;
          border-radius: 4px;
          transition: all 0.2s ease;
        }

        .close-button:hover {
          background: #333;
          color: #fff;
        }

        .shortcuts-list {
          padding: 20px;
          max-height: 60vh;
          overflow-y: auto;
        }

        .shortcut-item {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 12px;
          border-bottom: 1px solid #2a2a2a;
          transition: background 0.2s ease;
        }

        .shortcut-item:last-child {
          border-bottom: none;
        }

        .shortcut-item:hover {
          background: #2a2a2a;
        }

        .shortcut-keys {
          font-family: 'Courier New', monospace;
          font-weight: 600;
          color: #3b82f6;
          background: #2a2a2a;
          padding: 6px 12px;
          border-radius: 6px;
          border: 1px solid #333;
        }

        .shortcut-description {
          color: #ccc;
          flex: 1;
          margin-left: 20px;
        }

        .shortcuts-modal-footer {
          padding: 15px 20px;
          border-top: 1px solid #333;
          text-align: center;
          color: #666;
          font-size: 14px;
        }

        .shortcuts-modal-footer kbd {
          background: #2a2a2a;
          padding: 2px 6px;
          border-radius: 4px;
          border: 1px solid #333;
          font-family: 'Courier New', monospace;
        }

        @keyframes fadeIn {
          from {
            opacity: 0;
          }
          to {
            opacity: 1;
          }
        }

        @keyframes slideUp {
          from {
            opacity: 0;
            transform: translateY(20px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }
      `}</style>
    </div>
  );
}
