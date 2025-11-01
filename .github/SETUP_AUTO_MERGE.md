# 🤖 Auto-Merge Setup Guide

This guide will help you enable the **fully autonomous AI review and auto-merge system**.

---

## 🎯 What This Does

When Jules (or any approved bot) creates a PR:

1. **7 AI Specialists** automatically review the code:
   - 🏗️ **Architecture Specialist** (OpenAI) - Analyzes design patterns
   - 🔒 **Security Specialist** (OpenAI) - Scans for vulnerabilities
   - 🦀 **Rust Specialist** - Checks Rust best practices
   - ⚛️ **Frontend Specialist** - Reviews React/TypeScript code
   - 🧪 **Test Coverage Specialist** - Ensures adequate testing
   - 🧠 **Gemini Strategic Specialist** (Google) - Big-picture architecture review
   - 💡 **Gemini Innovation Specialist** (Google) - Suggests improvements

2. **All Tests Run** - Windows, macOS, Linux builds

3. **Smart Auto-Merge Decision**:
   - ✅ If ALL checks pass → **Auto-merges to main**
   - ❌ If ANY check fails → **Blocks merge, adds comment**

4. **Bonus Automation**:
   - 🤖 Dependabot auto-merges safe dependency updates
   - 🧹 Stale issues automatically closed after 30 days
   - ⚡ Performance benchmarks on every PR
   - 📊 Weekly code quality dashboard

---

## 📋 Step-by-Step Setup

### **Step 1: Add GitHub Secrets**

Go to: `Settings` → `Secrets and variables` → `Actions` → `New repository secret`

Add these 3 secrets:

#### 1. `GH_PAT` (Personal Access Token)
- Go to: https://github.com/settings/tokens
- Click **"Generate new token (classic)"**
- Select scopes:
  - ✅ `repo` (full control)
  - ✅ `workflow`
  - ✅ `write:discussion`
- Copy the token
- Add as secret: `GH_PAT`

#### 2. `OPENAI_API_KEY`
- Get your key from: https://platform.openai.com/api-keys
- Click **"Create new secret key"**
- Copy the key
- Add as secret: `OPENAI_API_KEY`

**⚠️ Important:** This must be an OpenAI key, NOT a Google API key!

#### 3. `GOOGLE_API_KEY`
- Get your key from: https://aistudio.google.com/app/apikey
- Click **"Create API key"**
- Copy the key
- Add as secret: `GOOGLE_API_KEY`

**Why both?** We use 5 OpenAI specialists + 2 Google Gemini specialists for the most comprehensive code review possible!

---

### **Step 2: Enable Auto-Merge Feature**

1. Go to: `Settings` → `General`
2. Scroll to **"Pull Requests"**
3. Check: ✅ **"Allow auto-merge"**
4. Check: ✅ **"Automatically delete head branches"**
5. Click **"Save changes"**

---

### **Step 3: Set Up Branch Protection**

1. Go to: `Settings` → `Branches`
2. Click **"Add rule"** or edit existing `main` rule
3. Branch name pattern: `main`

**Configure these settings:**

✅ **"Require status checks to pass before merging"**
   - Add these required checks:
     - `Test on ubuntu-latest`
     - `Test on windows-latest`
     - `Test on macos-latest`
     - `🏗️ Architecture Specialist`
     - `🔒 Security Specialist`
     - `🦀 Rust Specialist`
     - `🧠 Gemini Strategic Specialist`
     - `✅ AI Review Summary`

✅ **"Require branches to be up to date before merging"**

❌ **DO NOT enable "Require approvals"** (this would block auto-merge)

❌ **DO NOT enable "Require conversation resolution"**

✅ **"Allow specified actors to bypass required pull requests"** (optional)
   - Add: `dependabot[bot]` if you want Dependabot to auto-merge

4. Click **"Create"** or **"Save changes"**

---

### **Step 4: Test the System**

Create a test PR to verify everything works:

```bash
# Create a test branch
git checkout -b test-auto-merge

# Make a tiny change
echo "# Test" >> README.md

# Commit and push
git add README.md
git commit -m "Test: Auto-merge system"
git push origin test-auto-merge

# Create PR on GitHub
gh pr create --title "Test Auto-Merge" --body "Testing autonomous merge system"
```

**Expected Behavior:**
1. PR opens
2. 7 AI specialists start reviewing (OpenAI + Google Gemini)
3. All tests run on 3 platforms
4. Performance benchmarks run
5. If everything passes → Auto-merge triggers
6. PR merges automatically
7. Branch deletes automatically

---

## 🚀 Using with Jules

When assigning tasks to Jules:

```
Create a PR to implement [feature name].

Once your PR is created, the autonomous review system will:
1. Run 7 AI specialist reviews (5 OpenAI + 2 Google Gemini)
2. Execute all tests on 3 platforms
3. Run performance benchmarks
4. Auto-merge if everything passes

No manual review needed unless checks fail.
```

Jules will create PRs that trigger the full pipeline automatically!

### 🤖 Dependabot Integration

Dependabot will automatically:
- Create PRs for security updates
- Auto-merge patch and minor updates (safe changes)
- Request human review for major version updates

No action needed from you!

---

## ⚙️ Configuration

### Customize AI Reviews

Edit `.github/workflows/ai-review.yml`:

- Add more specialists (performance, docs, etc.)
- Adjust severity thresholds
- Modify review criteria

### Change Merge Strategy

Edit `.github/workflows/auto-merge.yml`:

```yaml
merge_method: 'squash'  # Options: squash, merge, rebase
```

### Block Auto-Merge Temporarily

Add one of these labels to any PR:
- `🚫 do-not-merge` - Blocks all merging
- `👀 needs-human-review` - Requires manual approval

---

## 🛡️ Safety Features

### Built-in Protections:

✅ **Multi-Specialist Review** - 5 different AI perspectives
✅ **Cross-Platform Testing** - Windows, macOS, Linux
✅ **Security Scanning** - Cargo audit + AI security review
✅ **Test Coverage** - Minimum coverage enforced
✅ **Merge Conflict Detection** - Blocks if conflicts exist
✅ **Draft PR Protection** - Won't merge draft PRs

### Emergency Stop:

If you need to disable auto-merge immediately:

1. Go to: `Settings` → `Branches`
2. Edit `main` branch rule
3. Check: ✅ **"Require approvals (1)"**
4. This blocks ALL auto-merges until you uncheck it

---

## 📊 Monitoring

### View Auto-Merge Activity:

- **Actions Tab** - See all workflow runs
- **Pull Requests** - Filter by `auto-merged` label
- **Insights → Pulse** - See merge velocity

### Check Why Auto-Merge Failed:

1. Open the blocked PR
2. Check the **"Checks"** tab
3. Look for ❌ failed checks
4. Read AI specialist comments for details

---

## 🎉 You're Done!

Your repository now has:

✅ **7 AI Specialists** (OpenAI + Google Gemini) reviewing every PR
✅ **Autonomous testing** on 3 platforms
✅ **Smart auto-merge** for bot PRs
✅ **Zero-touch deployment** when checks pass
✅ **Dependabot auto-merge** for safe updates
✅ **Stale issue cleanup** (runs daily)
✅ **Performance tracking** on every PR
✅ **Weekly quality dashboard** with metrics

**Jules can now work overnight and merge its own PRs! 🚀**

---

## 🌟 What's New in This Setup

### Dual AI Review System
We use BOTH OpenAI and Google Gemini because:
- **OpenAI specialists** focus on technical details (security, architecture, linting)
- **Google Gemini specialists** provide strategic insights and innovation ideas
- Together they give you the most comprehensive code review possible

### Automatic Maintenance
The system maintains itself:
- Old issues get closed automatically
- Dependencies stay up to date
- Performance regressions are caught early
- Code quality trends are tracked weekly

---

## 🆘 Troubleshooting

### "Auto-merge failed to enable"

**Cause:** Missing `GH_PAT` secret or wrong permissions

**Fix:**
1. Verify `GH_PAT` secret exists
2. Ensure PAT has `repo` and `workflow` scopes
3. Regenerate token if needed

---

### "Required checks not found"

**Cause:** Check names don't match branch protection

**Fix:**
1. Run a PR to see actual check names
2. Update branch protection to match exact names
3. Check names are case-sensitive!

---

### "Merge conflicts prevent auto-merge"

**Cause:** Branch is out of sync with main

**Fix:**
- Jules should handle this automatically
- Or manually: `git merge origin/main`

---

### "AI reviews not posting comments"

**Cause:** Missing API keys or rate limit hit

**Fix:**
1. Verify both `OPENAI_API_KEY` and `GOOGLE_API_KEY` secrets exist
2. Check OpenAI account has credits (https://platform.openai.com/account/usage)
3. Check Google AI Studio quota (https://aistudio.google.com/app/apikey)
4. Review GitHub Actions logs for errors

---

### "Gemini reviews failing"

**Cause:** Wrong API key or model access issue

**Fix:**
1. Ensure you used Google API key, NOT OpenAI key
2. Get key from: https://aistudio.google.com/app/apikey
3. Make sure Gemini 2.0 Flash is enabled for your account
4. Check the workflow logs for specific error messages

---

## 📋 All Active Workflows

Your repository now runs these workflows automatically:

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| **🤖 AI Specialist Review Team** | Every PR | 7 AI specialists review code |
| **🚀 Smart Auto-Merge** | After reviews complete | Auto-merges when all checks pass |
| **🤖 Dependabot Auto-Merge** | Dependabot PRs | Auto-merges safe dependency updates |
| **🧹 Stale Issue Manager** | Daily at 2 AM UTC | Closes inactive issues/PRs |
| **⚡ Performance Benchmark** | Every PR | Tracks performance metrics |
| **📊 Code Quality Dashboard** | Weekly on Mondays | Generates quality report |
| **🧪 Build and Test** | Every PR & push | Tests on 3 platforms |

---

**Questions? Check the Actions logs or create an issue!**
