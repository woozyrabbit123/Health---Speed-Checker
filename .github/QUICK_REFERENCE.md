# ⚡ Quick Reference Card

## 🔐 Required Secrets (3 Total)

| Secret Name | Get From | Used By |
|-------------|----------|---------|
| `GH_PAT` | [github.com/settings/tokens](https://github.com/settings/tokens) | Auto-merge, comments |
| `OPENAI_API_KEY` | [platform.openai.com/api-keys](https://platform.openai.com/api-keys) | 5 OpenAI specialists |
| `GOOGLE_API_KEY` | [aistudio.google.com/app/apikey](https://aistudio.google.com/app/apikey) | 2 Gemini specialists |

---

## 🤖 The 7 AI Specialists

| Specialist | AI Model | Focus |
|------------|----------|-------|
| 🏗️ Architecture | OpenAI GPT-4 | Design patterns, structure |
| 🔒 Security | OpenAI GPT-4 | Vulnerabilities, secrets |
| 🦀 Rust | Clippy | Rust idioms, performance |
| ⚛️ Frontend | ESLint | React, TypeScript |
| 🧪 Test Coverage | Tarpaulin | Test completeness |
| 🧠 Gemini Strategic | Google Gemini | Big-picture strategy |
| 💡 Gemini Innovation | Google Gemini | Modernization ideas |

---

## 🔄 Active Workflows (7 Total)

| Workflow | Trigger | What It Does |
|----------|---------|--------------|
| **AI Review Team** | Every PR | 7 specialists review code |
| **Auto-Merge** | After reviews | Merges if all checks pass |
| **Dependabot Merge** | Dependabot PRs | Auto-merges safe updates |
| **Stale Manager** | Daily 2 AM | Closes old issues (30d) |
| **Benchmarks** | Every PR | Tracks performance |
| **Quality Dashboard** | Weekly Mon 9 AM | Posts health metrics |
| **Build & Test** | Every PR | Tests 3 platforms |

---

## ✅ What Auto-Merges

- ✅ Jules/bot PRs that pass all checks
- ✅ Dependabot patch updates (1.2.3 → 1.2.4)
- ✅ Dependabot minor updates (1.2.0 → 1.3.0)

## ❌ What Requires Human Review

- ❌ Major version updates (1.x.x → 2.x.x)
- ❌ Any failing checks
- ❌ PRs with `🚫 do-not-merge` label
- ❌ PRs with `👀 needs-human-review` label
- ❌ Draft PRs

---

## 🏷️ Important Labels

| Label | Effect |
|-------|--------|
| `🚫 do-not-merge` | Blocks all merging |
| `👀 needs-human-review` | Requires manual approval |
| `🔥 critical` | Never marked stale |
| `🐛 bug` | Never marked stale |
| `🕸️ stale` | Added after 30 days inactive |
| `✅ auto-merged` | Successfully auto-merged |
| `🤖 ai-approved` | All AI specialists passed |

---

## 🎯 Quick Commands

### Check Workflow Status
```bash
gh run list --workflow=ai-review.yml
gh run list --workflow=auto-merge.yml
```

### View PR Status
```bash
gh pr view <NUMBER> --json statusCheckRollup
```

### Verify Secrets
```bash
gh secret list
```

### Manually Trigger Workflow
```bash
gh workflow run code-quality-dashboard.yml
gh workflow run performance-benchmark.yml
```

### Check API Usage
```bash
# OpenAI usage
open https://platform.openai.com/account/usage

# Google AI Studio quota
open https://aistudio.google.com/app/apikey
```

---

## 📊 Cost Estimates

| Service | Cost Per PR | Monthly (20 PRs) |
|---------|-------------|------------------|
| OpenAI | $0.10-0.30 | $2-6 |
| Google Gemini | FREE | FREE |
| **Total** | **~$0.15** | **~$5** |

---

## 🆘 Emergency: Disable Auto-Merge

**Option 1: Add Label**
- Add `🚫 do-not-merge` to any PR

**Option 2: Branch Protection**
1. Settings → Branches → Edit `main`
2. Check: "Require approvals (1)"
3. This blocks ALL auto-merges

**Option 3: Pause Workflow**
1. Actions → Auto-Merge workflow
2. Click "..." → Disable workflow

---

## 🎮 Task Jules

```
Jules, create a PR to [task description].

The autonomous system will handle:
- 7 AI specialist reviews
- 3 platform tests
- Performance benchmarks
- Auto-merge when ready

No manual work needed!
```

---

## 📈 Where to Monitor

| What | Where |
|------|-------|
| Workflow runs | Actions tab |
| Auto-merged PRs | PRs filtered by `✅ auto-merged` |
| Blocked PRs | PRs filtered by `🚫 auto-merge-blocked` |
| Quality metrics | Issues with `📊 dashboard` label |
| API costs | OpenAI dashboard |

---

## 🔗 Documentation Links

| Doc | Purpose |
|-----|---------|
| [SETUP_AUTO_MERGE.md](SETUP_AUTO_MERGE.md) | Full setup instructions |
| [AUTOMATION_GUIDE.md](AUTOMATION_GUIDE.md) | Complete system explanation |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | This file (quick lookup) |

---

## ⚠️ Common Issues

### Auto-merge not working?
1. Check all required checks passed
2. Verify PR is from bot account
3. Ensure `GH_PAT` secret is valid
4. Check for blocking labels

### AI reviews not posting?
1. Verify both API keys exist
2. Check API credits/quota
3. Review workflow logs for errors

### Performance benchmarks failing?
- First run? Normal (no baseline)
- Not a blocker for merge

---

**Print this page and keep it handy!**
