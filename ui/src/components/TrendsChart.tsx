import { useEffect, useRef, useState } from 'react';
import './TrendsChart.css';

interface ScanHistory {
  timestamp: number;
  health_score: number;
  speed_score: number;
  issues_found: number;
}

interface TrendsChartProps {
  data: ScanHistory[];
  type: 'health' | 'speed';
}

// Helper function to get chart colors based on type
const getChartColor = (type: 'health' | 'speed'): { solid: string; transparent: string } => {
  return type === 'health'
    ? { solid: '#22c55e', transparent: 'rgba(34, 197, 94, ' }
    : { solid: '#3b82f6', transparent: 'rgba(59, 130, 246, ' };
};

export function TrendsChart({ data, type }: TrendsChartProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [hoveredPoint, setHoveredPoint] = useState<number | null>(null);

  useEffect(() => {
    if (!canvasRef.current || data.length === 0) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.scale(dpr, dpr);

    // Clear canvas
    ctx.clearRect(0, 0, rect.width, rect.height);

    // Chart dimensions
    const padding = 40;
    const chartWidth = rect.width - padding * 2;
    const chartHeight = rect.height - padding * 2;

    // Get data points
    const values = data.map(d =>
      type === 'health' ? d.health_score : d.speed_score
    );
    const maxValue = Math.max(...values, 100);
    const minValue = Math.min(...values, 0);

    // Draw grid lines
    ctx.strokeStyle = '#2a2a2a';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
      const y = padding + (chartHeight / 5) * i;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(rect.width - padding, y);
      ctx.stroke();

      // Y-axis labels
      const value = Math.round(maxValue - (maxValue - minValue) * (i / 5));
      ctx.fillStyle = '#888';
      ctx.font = '12px sans-serif';
      ctx.textAlign = 'right';
      ctx.fillText(value.toString(), padding - 10, y + 4);
    }

    // Calculate points
    const points = data.map((d, i) => {
      const value = type === 'health' ? d.health_score : d.speed_score;
      const x = padding + (chartWidth / (data.length - 1)) * i;
      const y = padding + chartHeight - ((value - minValue) / (maxValue - minValue)) * chartHeight;
      return { x, y, value, data: d };
    });

    // Draw gradient fill
    const colors = getChartColor(type);
    const gradient = ctx.createLinearGradient(0, padding, 0, rect.height - padding);
    gradient.addColorStop(0, colors.transparent + '0.3)');
    gradient.addColorStop(1, colors.transparent + '0.0)');

    ctx.fillStyle = gradient;
    ctx.beginPath();
    ctx.moveTo(points[0].x, rect.height - padding);
    points.forEach(p => ctx.lineTo(p.x, p.y));
    ctx.lineTo(points[points.length - 1].x, rect.height - padding);
    ctx.closePath();
    ctx.fill();

    // Draw line
    ctx.strokeStyle = colors.solid;
    ctx.lineWidth = 3;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
    ctx.beginPath();
    points.forEach((p, i) => {
      if (i === 0) ctx.moveTo(p.x, p.y);
      else ctx.lineTo(p.x, p.y);
    });
    ctx.stroke();

    // Draw points
    points.forEach((p, i) => {
      ctx.beginPath();
      ctx.arc(p.x, p.y, i === hoveredPoint ? 6 : 4, 0, Math.PI * 2);
      ctx.fillStyle = type === 'health' ? '#22c55e' : '#3b82f6';
      ctx.fill();
      ctx.strokeStyle = '#1a1a1a';
      ctx.lineWidth = 2;
      ctx.stroke();
    });

    // Draw X-axis labels (dates)
    ctx.fillStyle = '#888';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'center';
    points.forEach((p, i) => {
      if (i % Math.ceil(points.length / 5) === 0) {
        const date = new Date(p.data.timestamp * 1000);
        const label = date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
        ctx.fillText(label, p.x, rect.height - padding + 20);
      }
    });

  }, [data, type, hoveredPoint]);

  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;

    // Find closest point
    const padding = 40;
    const chartWidth = rect.width - padding * 2;
    const pointSpacing = chartWidth / (data.length - 1);
    const index = Math.round((x - padding) / pointSpacing);

    if (index >= 0 && index < data.length) {
      setHoveredPoint(index);
    }
  };

  const handleMouseLeave = () => {
    setHoveredPoint(null);
  };

  return (
    <div className="trends-chart">
      <div className="chart-header">
        <h3>{type === 'health' ? '🩺 Health Score Trends' : '⚡ Speed Score Trends'}</h3>
        {data.length > 0 && (
          <div className="chart-stats">
            <span className="stat">
              Last: <strong>{type === 'health' ? data[data.length - 1].health_score : data[data.length - 1].speed_score}</strong>
            </span>
            <span className="stat">
              Avg: <strong>
                {Math.round(data.reduce((sum, d) => sum + (type === 'health' ? d.health_score : d.speed_score), 0) / data.length)}
              </strong>
            </span>
          </div>
        )}
      </div>

      <div className="chart-container">
        <canvas
          ref={canvasRef}
          className="chart-canvas"
          onMouseMove={handleMouseMove}
          onMouseLeave={handleMouseLeave}
        />

        {hoveredPoint !== null && data[hoveredPoint] && (
          <div className="chart-tooltip" style={{
            left: `${40 + (hoveredPoint / (data.length - 1)) * (100 - 80)}%`
          }}>
            <div className="tooltip-date">
              {new Date(data[hoveredPoint].timestamp * 1000).toLocaleDateString('en-US', {
                month: 'short',
                day: 'numeric',
                year: 'numeric'
              })}
            </div>
            <div className="tooltip-value">
              {type === 'health' ? '🩺 Health' : '⚡ Speed'}: <strong>{type === 'health' ? data[hoveredPoint].health_score : data[hoveredPoint].speed_score}</strong>
            </div>
            <div className="tooltip-issues">
              Issues: {data[hoveredPoint].issues_found}
            </div>
          </div>
        )}
      </div>

      {data.length === 0 && (
        <div className="chart-empty">
          <div className="empty-icon">📊</div>
          <p>No historical data yet</p>
          <span>Run a few scans to see trends</span>
        </div>
      )}
    </div>
  );
}
