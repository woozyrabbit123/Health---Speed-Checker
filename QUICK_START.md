# Quick Start Guide - All New Features

## Keyboard Shortcuts (Just Press and Go!)

| Shortcut | Action |
|----------|--------|
| **Ctrl+S** | Start full scan |
| **Ctrl+Q** | Start quick scan (5s) |
| **Ctrl+F** | Fix the top issue |
| **Ctrl+E** | Export report |
| **Ctrl+H** | View history |
| **Ctrl+,** | Open settings |
| **Ctrl+/** | Show help |
| **Esc** | Cancel/Close dialogs |
| **F1** | Show all shortcuts |

## Quick Actions Widget (Bottom Right)

The floating circle shows your health score:
- **Click** to expand menu
- **Drag** to move anywhere
- **5 Quick Buttons**:
  - ⚡ Quick Scan
  - 🔍 Full Scan
  - 🔧 Fix Top Issue
  - 💾 Export
  - 📊 History

## Export Your Report

1. Press **Ctrl+E** or click "Export Report"
2. Choose format:
   - **📋 JSON** - For automation/scripts
   - **📄 PDF** - Professional report (coming soon)
   - **🌐 HTML** - Beautiful web page
   - **📊 CSV** - Open in Excel
3. Click **Export**
4. Choose save location

## System Tray

Look for the tray icon (system notification area):
- **Left-click** - Show/hide window
- **Right-click** - Quick menu:
  - Quick Scan
  - Full Scan
  - Fix Top Issue
  - Quit

## What Gets Checked (9 Checkers)

### Security (4 Checkers)
- ✅ Firewall Status
- ✅ OS Updates
- ✅ Open Ports
- ✅ Bloatware

### Performance (5 Checkers)
- ✅ Network Speed & Latency
- ✅ Disk Health (S.M.A.R.T.)
- ✅ Storage Space & Fragmentation
- ✅ Startup Programs
- ✅ CPU/Memory Usage

## Example Workflow

### First Time User
1. Launch the app
2. Press **Ctrl+S** for full scan
3. Wait 5-10 seconds
4. Review issues
5. Click "Fix" on any auto-fixable issues
6. Press **Ctrl+E** to export results

### Daily User
1. Press **Ctrl+Q** for quick scan (5s)
2. Check the quick actions widget score
3. If score drops, press **Ctrl+F** to fix top issue
4. Done!

### Power User
1. System tray monitoring (always running)
2. Right-click tray → "Quick Scan" daily
3. Use keyboard shortcuts for everything
4. Export weekly reports via **Ctrl+E**

## New Issues You'll See

### Network Issues
- **High Latency** - If ping > 150ms
- **Slow DNS** - If DNS > 100ms
- **Slow Download** - If speed < 5 Mbps
- **Proxy Detected** - If VPN/proxy active

**Auto-fix**: Can switch to Cloudflare DNS (1.1.1.1)

### Storage Issues
- **Low Disk Space** - If < 10% free
- **High Fragmentation** - If > 15% (Windows)
- **Inefficient File System** - FAT32 on large drives

**Auto-fix**: Launches Disk Cleanup and Defragmenter

### Disk Health Issues
- **⚠️ CRITICAL: Drive Failure Predicted** - BACK UP NOW
- **Drive Degraded** - Performance issues
- **Bad Sectors Detected** - Data loss risk

**No auto-fix** - Manual replacement needed

### Bloatware Issues
- Detects 20+ known bloatware patterns
- Discord, Spotify, McAfee, Norton, etc.
- Shows startup impact

**Auto-fix**: Can disable startup entries

## Tips & Tricks

### Keep Score High
- Run quick scan daily (**Ctrl+Q**)
- Fix issues immediately (**Ctrl+F**)
- Export weekly reports for tracking

### Monitor Trends
- After 2+ scans, scroll down to see charts
- Green trend = improving
- Red trend = getting worse

### Use Tray Icon
- Minimize to tray (keeps monitoring)
- Right-click for quick actions
- Get notifications for critical issues

### Export for IT Support
1. Run full scan (**Ctrl+S**)
2. Export as HTML (**Ctrl+E** → HTML)
3. Send beautiful report to tech support
4. Includes all details they need

### Keyboard-Only Operation
```
Ctrl+S → Wait → Ctrl+F → Ctrl+E → Enter → Done!
```
Never touch the mouse!

## Troubleshooting

### "No results showing"
- Make sure scan completed (100%)
- Check for errors in console
- Try quick scan first (**Ctrl+Q**)

### "Export failed"
- Ensure you have write permissions
- Choose a different save location
- Try JSON format first (simplest)

### "Tray icon not showing"
- Check system tray overflow area
- Restart the application
- Check platform support (Windows/Mac/Linux)

### "Keyboard shortcuts not working"
- Make sure dialog isn't open
- Try **F1** to see all shortcuts
- Check for conflicts with other apps

## File Locations

### Reports (when exported)
- **JSON**: Machine-readable data
- **HTML**: Standalone web page
- **CSV**: Open in Excel/Sheets

### Scan Data
- Stored in memory (for now)
- Database integration coming soon
- Last 30 scans kept for trends

## What's Next?

After integration testing:
- [ ] PDF export implementation
- [ ] Database persistence (Jules working on this)
- [ ] Real-time progress events
- [ ] Admin elevation for auto-fixes
- [ ] Extended historical data

---

**Have fun with your new features!**

Press **F1** in the app to see the shortcuts modal.
Press **Ctrl+Q** for your first quick scan.
Check the bottom-right for the quick actions widget!
