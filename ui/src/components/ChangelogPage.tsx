import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { FileClock, Loader2, ShieldCheck } from 'lucide-react';

interface ChangelogEntry {
  timestamp: number;
  action: string;
  path: string;
  size_bytes: number;
  reason: string;
}

const formatTimestamp = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);
  if (Number.isNaN(date.getTime())) {
    return 'Unknown time';
  }
  return date.toLocaleString(undefined, {
    year: 'numeric',
    month: 'short',
    day: '2-digit',
    hour: 'numeric',
    minute: '2-digit',
  });
};

const formatSize = (bytes: number): string => {
  if (!Number.isFinite(bytes) || bytes <= 0) {
    return '—';
  }

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex += 1;
  }
  return `${size.toFixed(size >= 10 || unitIndex === 0 ? 0 : 1)}${units[unitIndex]}`;
};

export function ChangelogPage() {
  const [entries, setEntries] = useState<ChangelogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const fetchChangelog = async () => {
      try {
        const result = await invoke<ChangelogEntry[]>('get_changelog');
        if (mounted) {
          setEntries(result);
        }
      } catch (err) {
        console.error('Failed to load changelog', err);
        if (mounted) {
          setError('Unable to load transparency log. Please try again after running a scan.');
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void fetchChangelog();
    return () => {
      mounted = false;
    };
  }, []);

  const emptyState = useMemo(
    () => (
      <div className="flex flex-col items-center justify-center rounded-2xl border border-gray-800 bg-gray-900/60 p-12 text-center">
        <ShieldCheck className="mb-4 h-10 w-10 text-blue-400" />
        <h3 className="text-xl font-semibold text-white">Nothing to report yet</h3>
        <p className="mt-2 max-w-lg text-sm text-gray-400">
          We will list every file, folder, or registry change we ever make. Run a scan or apply a fix to populate this
          transparency log.
        </p>
      </div>
    ),
    []
  );

  return (
    <div className="mx-auto flex max-w-5xl flex-col gap-6">
      <header className="flex flex-col gap-2">
        <div className="inline-flex items-center gap-3 rounded-full border border-blue-500/30 bg-blue-500/10 px-4 py-2 text-sm font-medium text-blue-200">
          <FileClock className="h-4 w-4" />
          Transparency Log (What We&apos;ve Done)
        </div>
        <p className="text-sm text-gray-400">
          Every file operation is recorded locally. Review this forensic changelog to see the exact changes we have made
          on your system.
        </p>
      </header>

      {loading ? (
        <div className="flex items-center justify-center rounded-2xl border border-gray-800 bg-gray-900/60 p-12 text-gray-400">
          <Loader2 className="mr-3 h-5 w-5 animate-spin" />
          Loading transparency log...
        </div>
      ) : error ? (
        <div className="rounded-2xl border border-red-500/40 bg-red-500/10 p-6 text-red-100">
          {error}
        </div>
      ) : entries.length === 0 ? (
        emptyState
      ) : (
        <div className="space-y-4">
          {entries.map((entry) => (
            <article
              key={`${entry.timestamp}-${entry.path}-${entry.action}`}
              className="rounded-2xl border border-gray-800 bg-gray-900/60 p-5 shadow-lg shadow-black/10"
            >
              <div className="flex flex-wrap items-center justify-between gap-3">
                <div className="flex items-center gap-3">
                  <span className="rounded-full bg-gray-800 px-3 py-1 text-xs font-semibold uppercase tracking-wide text-gray-200">
                    {entry.action}
                  </span>
                  <time className="text-sm text-gray-400">{formatTimestamp(entry.timestamp)}</time>
                </div>
                <span className="text-sm text-gray-500">{formatSize(entry.size_bytes)}</span>
              </div>
              <p className="mt-3 break-words font-mono text-sm text-white">{entry.path}</p>
              <p className="mt-2 text-sm text-gray-300">{entry.reason}</p>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}

export default ChangelogPage;
