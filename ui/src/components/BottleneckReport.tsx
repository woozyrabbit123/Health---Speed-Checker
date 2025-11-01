import { AlertTriangle, ArrowUpRight } from 'lucide-react';

interface FixAction {
  action_id: string;
  label: string;
  is_auto_fix: boolean;
  params?: Record<string, unknown>;
}

interface BottleneckIssue {
  id: string;
  title: string;
  description: string;
  fix?: FixAction | null;
}

interface BottleneckReportProps {
  issue: BottleneckIssue;
  onFix?: (fix: FixAction) => void;
}

interface BulletSection {
  title: string;
  items: string[];
}

export function BottleneckReport({ issue, onFix }: BottleneckReportProps) {
  const lines = issue.description
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0);

  const paragraphs: string[] = [];
  const honestStatements: string[] = [];
  const bulletSections: BulletSection[] = [];
  const footerFacts: string[] = [];

  let currentBulletSection: BulletSection | null = null;

  for (const line of lines) {
    if (line.toUpperCase().startsWith('HONEST')) {
      honestStatements.push(line);
      currentBulletSection = null;
      continue;
    }

    if (line.endsWith(':') && !line.startsWith('-')) {
      currentBulletSection = { title: line, items: [] };
      bulletSections.push(currentBulletSection);
      continue;
    }

    if (line.startsWith('- ')) {
      if (!currentBulletSection) {
        currentBulletSection = { title: '', items: [] };
        bulletSections.push(currentBulletSection);
      }
      currentBulletSection.items.push(line.slice(2));
      continue;
    }

    if (/^(Cost|Difficulty)/i.test(line)) {
      footerFacts.push(line);
      currentBulletSection = null;
      continue;
    }

    paragraphs.push(line);
    currentBulletSection = null;
  }

  return (
    <section
      className="mb-8 rounded-2xl border border-amber-500/40 bg-amber-500/10 p-6 text-amber-100 shadow-lg shadow-amber-900/20"
      role="alert"
    >
      <div className="mb-4 flex items-start gap-3">
        <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-amber-500/20 text-amber-200">
          <AlertTriangle className="h-6 w-6" />
        </div>
        <div>
          <p className="text-sm uppercase tracking-wide text-amber-300">
            ⚠️ Honest Bottleneck Report
          </p>
          <h2 className="mt-1 text-xl font-semibold text-white">{issue.title}</h2>
        </div>
      </div>

      <div className="space-y-4 text-amber-100">
        {paragraphs.map((paragraph, index) => (
          <p key={`paragraph-${index}`} className="leading-relaxed text-amber-100/90">
            {paragraph}
          </p>
        ))}

        {honestStatements.map((statement, index) => {
          const [label, value] = statement.split(':', 2);
          return (
            <div
              key={`honest-${index}`}
              className="rounded-xl border border-amber-500/30 bg-amber-500/10 px-4 py-3"
            >
              <p className="text-sm font-semibold uppercase tracking-wide text-amber-200">
                {label || 'HONEST RECOMMENDATION'}
              </p>
              {value && (
                <p className="mt-2 text-base font-medium text-white">
                  {value.trim()}
                </p>
              )}
            </div>
          );
        })}

        {bulletSections.map((section, index) => (
          <div key={`section-${index}`}>
            {section.title && (
              <p className="text-sm font-semibold uppercase tracking-wide text-amber-200">
                {section.title.replace(/:$/, '')}
              </p>
            )}
            <ul className="mt-2 list-disc space-y-1 pl-6 text-amber-100/90">
              {section.items.map((item, itemIndex) => (
                <li key={`section-${index}-item-${itemIndex}`}>{item}</li>
              ))}
            </ul>
          </div>
        ))}

        {footerFacts.length > 0 && (
          <div className="grid gap-2 md:grid-cols-2">
            {footerFacts.map((fact, index) => (
              <div
                key={`fact-${index}`}
                className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-4 py-2 text-sm text-amber-100/90"
              >
                {fact}
              </div>
            ))}
          </div>
        )}
      </div>

      {issue.fix && (
        <div className="mt-6">
          <button
            onClick={() => onFix?.(issue.fix!)}
            className="inline-flex items-center gap-2 rounded-xl bg-amber-400 px-4 py-2 text-sm font-semibold text-amber-950 transition-colors hover:bg-amber-300 focus:outline-none focus:ring-2 focus:ring-amber-200 focus:ring-offset-2 focus:ring-offset-amber-950"
          >
            {issue.fix.label}
            <ArrowUpRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </section>
  );
}

export default BottleneckReport;
