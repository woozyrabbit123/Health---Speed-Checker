import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CheckCircle2, Clock, Loader2, Lock, RefreshCw, ShieldCheck } from 'lucide-react';

interface AutomationSettings {
  automation_enabled: boolean;
  run_schedule: 'daily' | 'weekly' | 'monthly';
  auto_fix_enabled: boolean;
}

interface AutomationPageProps {
  onUpgrade: () => void;
}

const defaultSettings: AutomationSettings = {
  automation_enabled: false,
  run_schedule: 'weekly',
  auto_fix_enabled: false,
};

const scheduleLabels: Record<AutomationSettings['run_schedule'], string> = {
  daily: 'Daily',
  weekly: 'Weekly',
  monthly: 'Monthly',
};

export function AutomationPage({ onUpgrade }: AutomationPageProps) {
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [hasAccess, setHasAccess] = useState<boolean | null>(null);
  const [settings, setSettings] = useState<AutomationSettings>(defaultSettings);
  const [draft, setDraft] = useState<AutomationSettings>(defaultSettings);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const hasChanges = useMemo(
    () =>
      settings.automation_enabled !== draft.automation_enabled ||
      settings.run_schedule !== draft.run_schedule ||
      settings.auto_fix_enabled !== draft.auto_fix_enabled,
    [settings, draft]
  );

  useEffect(() => {
    let mounted = true;

    const loadSettings = async () => {
      try {
        const access = await invoke<boolean>('check_feature_access', {
          feature_name: 'automation',
        });

        if (!mounted) {
          return;
        }

        setHasAccess(access);

        if (access) {
          const current = await invoke<AutomationSettings>('get_automation_settings');
          if (!mounted) {
            return;
          }
          setSettings(current);
          setDraft(current);
        }
      } catch (err) {
        console.error('Failed to load automation settings', err);
        if (mounted) {
          setError('Unable to load automation settings. Please try again.');
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadSettings();
    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    if (!success) {
      return;
    }

    const timer = setTimeout(() => setSuccess(null), 3500);
    return () => clearTimeout(timer);
  }, [success]);

  useEffect(() => {
    if (!error) {
      return;
    }

    const timer = setTimeout(() => setError(null), 5000);
    return () => clearTimeout(timer);
  }, [error]);

  const toggleAutomation = () => {
    setDraft((prev) => ({ ...prev, automation_enabled: !prev.automation_enabled }));
  };

  const toggleAutoFix = () => {
    setDraft((prev) => ({ ...prev, auto_fix_enabled: !prev.auto_fix_enabled }));
  };

  const updateSchedule = (run_schedule: AutomationSettings['run_schedule']) => {
    setDraft((prev) => ({ ...prev, run_schedule }));
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      await invoke('set_automation_settings', { settings: draft });
      setSettings(draft);
      setSuccess('Automation preferences saved');
    } catch (err) {
      console.error('Failed to save automation settings', err);
      setError('Could not save automation settings. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const handleReset = () => {
    setDraft(settings);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-[60vh] text-gray-400">
        <Loader2 className="w-6 h-6 mr-3 animate-spin" />
        Loading automation suite...
      </div>
    );
  }

  if (hasAccess === false) {
    return (
      <div className="max-w-3xl mx-auto bg-gray-900/70 border border-gray-800 rounded-2xl p-10 text-center">
        <Lock className="w-12 h-12 mx-auto text-blue-400 mb-6" />
        <h2 className="text-3xl font-semibold mb-4">Unlock Automation Superpowers</h2>
        <p className="text-gray-400 mb-8 leading-relaxed">
          Schedule scans, auto-fix issues, and keep your PC fast without lifting a finger.
          Upgrade to Pro to enable the automation suite and run Health &amp; Speed Checker on autopilot.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8 text-left">
          <div className="bg-gray-900 border border-gray-800 rounded-xl p-4">
            <Clock className="w-6 h-6 text-blue-400 mb-3" />
            <h3 className="font-semibold mb-1">Scheduled Scans</h3>
            <p className="text-gray-400 text-sm">Run daily, weekly, or monthly scans automatically.</p>
          </div>
          <div className="bg-gray-900 border border-gray-800 rounded-xl p-4">
            <ShieldCheck className="w-6 h-6 text-purple-400 mb-3" />
            <h3 className="font-semibold mb-1">Hands-Free Auto-Fix</h3>
            <p className="text-gray-400 text-sm">Resolve fixable issues instantly after each scan.</p>
          </div>
          <div className="bg-gray-900 border border-gray-800 rounded-xl p-4">
            <RefreshCw className="w-6 h-6 text-teal-400 mb-3" />
            <h3 className="font-semibold mb-1">Always Optimized</h3>
            <p className="text-gray-400 text-sm">Keep performance high without manual check-ins.</p>
          </div>
        </div>
        <button
          onClick={onUpgrade}
          className="px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 rounded-lg font-semibold transition-colors"
        >
          Upgrade to Pro
        </button>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="bg-gray-900/70 border border-gray-800 rounded-2xl p-8">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-semibold">Automation Suite</h2>
            <p className="text-gray-400 mt-2">
              Schedule full scans and auto-fix issues so your system stays fast and secure on autopilot.
            </p>
          </div>
          {success && (
            <span className="flex items-center text-sm text-green-400">
              <CheckCircle2 className="w-4 h-4 mr-2" />
              {success}
            </span>
          )}
        </div>

        {error && (
          <div className="mt-6 bg-red-500/10 border border-red-500/40 text-red-200 px-4 py-3 rounded-lg text-sm">
            {error}
          </div>
        )}

        <div className="mt-8 space-y-6">
          <section className="bg-gray-950/60 border border-gray-800 rounded-xl p-6">
            <div className="flex items-start justify-between">
              <div>
                <h3 className="text-lg font-semibold">Enable Scheduled Automation</h3>
                <p className="text-gray-400 mt-1 text-sm">
                  When enabled, Health &amp; Speed Checker will run scans automatically based on your selected frequency.
                </p>
              </div>
              <button
                onClick={toggleAutomation}
                className={`w-16 h-8 rounded-full transition-colors flex items-center px-1 ${
                  draft.automation_enabled ? 'bg-blue-600' : 'bg-gray-700'
                }`}
              >
                <span
                  className={`bg-white w-6 h-6 rounded-full transform transition-transform ${
                    draft.automation_enabled ? 'translate-x-8' : ''
                  }`}
                />
              </button>
            </div>
          </section>

          <section className="bg-gray-950/60 border border-gray-800 rounded-xl p-6 space-y-4">
            <h3 className="text-lg font-semibold flex items-center">
              <Clock className="w-5 h-5 mr-2 text-blue-400" />
              Scan Frequency
            </h3>
            <p className="text-gray-400 text-sm">
              Choose how often to run an automated full scan. We recommend weekly for most systems.
            </p>
            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
              {(['daily', 'weekly', 'monthly'] as AutomationSettings['run_schedule'][]).map((option) => (
                <button
                  key={option}
                  onClick={() => updateSchedule(option)}
                  className={`border rounded-lg p-4 text-left transition-colors ${
                    draft.run_schedule === option ? 'border-blue-500 bg-blue-500/10' : 'border-gray-800 hover:border-blue-500/50'
                  }`}
                >
                  <div className="font-semibold capitalize">{scheduleLabels[option]}</div>
                  <p className="text-gray-400 text-sm mt-1">
                    {option === 'daily' && 'Best for mission-critical machines that need constant monitoring.'}
                    {option === 'weekly' && 'Balanced protection and performance (recommended).'}
                    {option === 'monthly' && 'Light-touch maintenance for rarely used machines.'}
                  </p>
                </button>
              ))}
            </div>
          </section>

          <section className="bg-gray-950/60 border border-gray-800 rounded-xl p-6">
            <div className="flex items-start justify-between">
              <div>
                <h3 className="text-lg font-semibold">Enable Hands-Free Auto-Fix</h3>
                <p className="text-gray-400 mt-1 text-sm">
                  Automatically run available safe fixes immediately after each automated scan.
                </p>
              </div>
              <button
                onClick={toggleAutoFix}
                className={`w-16 h-8 rounded-full transition-colors flex items-center px-1 ${
                  draft.auto_fix_enabled ? 'bg-purple-600' : 'bg-gray-700'
                }`}
              >
                <span
                  className={`bg-white w-6 h-6 rounded-full transform transition-transform ${
                    draft.auto_fix_enabled ? 'translate-x-8' : ''
                  }`}
                />
              </button>
            </div>
          </section>
        </div>

        <div className="mt-8 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div className="text-sm text-gray-400">
            ⏱️ The automation engine checks every hour and runs when the next scheduled window opens.
          </div>
          <div className="flex items-center space-x-3">
            <button
              onClick={handleReset}
              disabled={!hasChanges || saving}
              className="px-4 py-2 rounded-lg border border-gray-700 text-gray-300 hover:border-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Reset
            </button>
            <button
              onClick={handleSave}
              disabled={!hasChanges || saving}
              className="px-5 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
            >
              {saving && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
              Save Settings
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AutomationPage;
