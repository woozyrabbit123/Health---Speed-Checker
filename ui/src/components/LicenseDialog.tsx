import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './LicenseDialog.css';

interface License {
  key: string | null;
  tier: 'Free' | 'Trial' | 'Pro';
  activated_at: number;
  expires_at: number | null;
}

interface LicenseDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

const LicenseDialog: React.FC<LicenseDialogProps> = ({ isOpen, onClose }) => {
  const [license, setLicense] = useState<License | null>(null);
  const [licenseKey, setLicenseKey] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  useEffect(() => {
    if (isOpen) {
      loadLicenseStatus();
    }
  }, [isOpen]);

  const loadLicenseStatus = async () => {
    try {
      const status = await invoke<License>('get_license_status');
      setLicense(status);
    } catch (err) {
      setError('Failed to load license status');
      console.error(err);
    }
  };

  const handleStartTrial = async () => {
    setLoading(true);
    setError('');
    setSuccess('');

    try {
      const newLicense = await invoke<License>('start_trial');
      setLicense(newLicense);
      setSuccess('14-day trial started successfully!');
    } catch (err) {
      setError((err as string) || 'Failed to start trial');
    } finally {
      setLoading(false);
    }
  };

  const handleActivateLicense = async () => {
    if (!licenseKey.trim()) {
      setError('Please enter a license key');
      return;
    }

    setLoading(true);
    setError('');
    setSuccess('');

    try {
      const newLicense = await invoke<License>('activate_license', { key: licenseKey.trim() });
      setLicense(newLicense);
      setSuccess('Pro license activated successfully!');
      setLicenseKey('');
    } catch (err) {
      setError((err as string) || 'Invalid license key');
    } finally {
      setLoading(false);
    }
  };

  const getDaysRemaining = (): number => {
    if (!license || license.tier !== 'Trial' || !license.expires_at) {
      return 0;
    }

    const now = Math.floor(Date.now() / 1000);
    const remainingSeconds = license.expires_at - now;
    return Math.max(0, Math.floor(remainingSeconds / 86400));
  };

  const getFeaturesList = (): { name: string; available: boolean }[] => {
    const tier = license?.tier || 'Free';

    if (tier === 'Pro' || tier === 'Trial') {
      return [
        { name: 'All 9 system checkers', available: true },
        { name: 'Export to CSV, HTML, PDF, JSON', available: true },
        { name: 'Automatic fixes', available: true },
        { name: 'Scan history', available: true },
      ];
    } else {
      return [
        { name: 'Basic checkers (Firewall, Startup, Process Monitor)', available: true },
        { name: 'HTML export only', available: true },
        { name: 'Advanced checkers (Network, Storage, etc.)', available: false },
        { name: 'PDF/CSV/JSON export', available: false },
        { name: 'Automatic fixes', available: false },
      ];
    }
  };

  if (!isOpen) return null;

  return (
    <div className="license-dialog-overlay" onClick={onClose}>
      <div className="license-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="license-dialog-header">
          <h2>License Management</h2>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <div className="license-dialog-content">
          {/* Current License Status */}
          <div className="license-status">
            <h3>Current License</h3>
            <div className={`license-badge ${license?.tier.toLowerCase()}`}>
              {license?.tier || 'Free'}
            </div>

            {license?.tier === 'Trial' && (
              <p className="trial-info">
                {getDaysRemaining()} days remaining in your trial
              </p>
            )}

            {license?.tier === 'Pro' && license.key && (
              <p className="license-key-info">
                License Key: {license.key}
              </p>
            )}
          </div>

          {/* Features List */}
          <div className="features-section">
            <h4>Features</h4>
            <ul className="features-list">
              {getFeaturesList().map((feature, index) => (
                <li key={index} className={feature.available ? 'available' : 'unavailable'}>
                  <span className="feature-icon">{feature.available ? '✓' : '✗'}</span>
                  {feature.name}
                </li>
              ))}
            </ul>
          </div>

          {/* Error/Success Messages */}
          {error && <div className="message error">{error}</div>}
          {success && <div className="message success">{success}</div>}

          {/* Trial Section */}
          {license?.tier === 'Free' && (
            <div className="trial-section">
              <h4>Start Free Trial</h4>
              <p>Try all Pro features free for 14 days!</p>
              <button
                className="btn-trial"
                onClick={handleStartTrial}
                disabled={loading}
              >
                {loading ? 'Starting...' : 'Start 14-Day Trial'}
              </button>
            </div>
          )}

          {/* Activation Section */}
          {license?.tier !== 'Pro' && (
            <div className="activation-section">
              <h4>Activate Pro License</h4>
              <p>Enter your license key to unlock all features permanently</p>
              <div className="activation-form">
                <input
                  type="text"
                  className="license-input"
                  placeholder="HSPC-XXXX-XXXX-XXXX-XXXX"
                  value={licenseKey}
                  onChange={(e) => setLicenseKey(e.target.value.toUpperCase())}
                  disabled={loading}
                />
                <button
                  className="btn-activate"
                  onClick={handleActivateLicense}
                  disabled={loading || !licenseKey.trim()}
                >
                  {loading ? 'Activating...' : 'Activate License'}
                </button>
              </div>
            </div>
          )}

          {/* Purchase Link */}
          {license?.tier !== 'Pro' && (
            <div className="purchase-section">
              <p className="purchase-text">
                Don't have a license yet?{' '}
                <a
                  href="https://yourwebsite.com/purchase"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="purchase-link"
                >
                  Purchase Pro License ($9.99)
                </a>
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default LicenseDialog;
