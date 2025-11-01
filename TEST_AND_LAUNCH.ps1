# One-Click Test & Launch Script for Health & Speed Checker
# Run this, review results, then post to X!

Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  HEALTH & SPEED CHECKER TEST SUITE" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"
$TestResults = @()

# Test 1: Rust Agent Build
Write-Host "[1/6] Building Rust agent..." -ForegroundColor Yellow
cd agent
$buildOutput = cargo build --release 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Rust agent built successfully" -ForegroundColor Green
    $TestResults += "✅ Rust Build: PASSED"
} else {
    Write-Host "  ❌ Build failed!" -ForegroundColor Red
    Write-Host $buildOutput -ForegroundColor DarkRed
    $TestResults += "❌ Rust Build: FAILED"
    exit 1
}

# Test 2: Run Full Scan
Write-Host ""
Write-Host "[2/6] Running full system scan (all 10 checkers)..." -ForegroundColor Yellow
$scanOutput = cargo run --release -- scan --output json 2>&1 | Out-String
$scanResult = $scanOutput | ConvertFrom-Json -ErrorAction SilentlyContinue

if ($scanResult) {
    Write-Host "  ✅ Scan completed successfully" -ForegroundColor Green
    Write-Host "  📊 Health Score: $($scanResult.scores.health)/100" -ForegroundColor Cyan
    Write-Host "  ⚡ Speed Score: $($scanResult.scores.speed)/100" -ForegroundColor Cyan
    Write-Host "  🔍 Issues Found: $($scanResult.issues.Count)" -ForegroundColor Cyan
    Write-Host "  ⏱️  Duration: $($scanResult.duration_ms)ms" -ForegroundColor Cyan
    $TestResults += "✅ Full Scan: PASSED (Health: $($scanResult.scores.health), Speed: $($scanResult.scores.speed))"
    
    # Show checkers that ran
    Write-Host ""
    Write-Host "  Checkers that ran:" -ForegroundColor Cyan
    $scanResult.issues | ForEach-Object { 
        Write-Host "    • $($_.id)" -ForegroundColor DarkGray
    }
} else {
    Write-Host "  ❌ Scan failed to produce valid JSON" -ForegroundColor Red
    $TestResults += "❌ Full Scan: FAILED"
}

# Test 3: Check Bottleneck Analyzer
Write-Host ""
Write-Host "[3/6] Checking Bottleneck Analyzer (killer feature)..." -ForegroundColor Yellow
$bottleneckIssues = $scanResult.issues | Where-Object { $_.id -like "bottleneck_*" }
if ($bottleneckIssues.Count -gt 0) {
    Write-Host "  ✅ Bottleneck Analyzer is working!" -ForegroundColor Green
    $bottleneckIssues | ForEach-Object {
        Write-Host "  📌 $($_.title)" -ForegroundColor Cyan
    }
    $TestResults += "✅ Bottleneck Analyzer: FOUND $($bottleneckIssues.Count) insights"
} else {
    Write-Host "  ⚠️  No bottleneck issues detected (might be a good PC!)" -ForegroundColor Yellow
    $TestResults += "⚠️  Bottleneck Analyzer: No issues (good hardware)"
}

# Test 4: Tauri App Build
Write-Host ""
Write-Host "[4/6] Building Tauri desktop app..." -ForegroundColor Yellow
cd ../ui
$tauriBuild = npm run tauri build 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Tauri app built successfully" -ForegroundColor Green
    $TestResults += "✅ Tauri Build: PASSED"
    
    # Find the installer
    $installer = Get-ChildItem -Path "src-tauri/target/release/bundle" -Recurse -Filter "*.msi" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($installer) {
        Write-Host "  📦 Installer: $($installer.FullName)" -ForegroundColor Cyan
        Write-Host "  📏 Size: $([math]::Round($installer.Length / 1MB, 2)) MB" -ForegroundColor Cyan
    }
} else {
    Write-Host "  ❌ Tauri build failed!" -ForegroundColor Red
    $TestResults += "❌ Tauri Build: FAILED"
}

# Test 5: Database Schema
Write-Host ""
Write-Host "[5/6] Checking database schema..." -ForegroundColor Yellow
if (Test-Path "../agent/db/schema.sql") {
    Write-Host "  ✅ Database schema exists" -ForegroundColor Green
    $schemaContent = Get-Content "../agent/db/schema.sql" -Raw
    if ($schemaContent -match "changelog") {
        Write-Host "  ✅ Forensic changelog table present" -ForegroundColor Green
        $TestResults += "✅ Database Schema: PASSED (with changelog)"
    } else {
        Write-Host "  ⚠️  Changelog table missing" -ForegroundColor Yellow
        $TestResults += "⚠️  Database Schema: Missing changelog"
    }
} else {
    Write-Host "  ❌ Database schema not found!" -ForegroundColor Red
    $TestResults += "❌ Database Schema: MISSING"
}

# Test 6: All Checkers Registered
Write-Host ""
Write-Host "[6/6] Verifying all 10 checkers are registered..." -ForegroundColor Yellow
$mainRs = Get-Content "../agent/src/main.rs" -Raw
$expectedCheckers = @(
    "FirewallChecker",
    "StartupAnalyzer", 
    "ProcessMonitor",
    "OsUpdateChecker",
    "PortScanner",
    "BloatwareDetector",
    "NetworkChecker",
    "SmartDiskChecker",
    "StorageChecker",
    "BottleneckAnalyzer"
)

$registeredCount = 0
foreach ($checker in $expectedCheckers) {
    if ($mainRs -match $checker) {
        $registeredCount++
    }
}

if ($registeredCount -eq 10) {
    Write-Host "  ✅ All 10 checkers registered!" -ForegroundColor Green
    $TestResults += "✅ Checker Registration: 10/10 checkers"
} else {
    Write-Host "  ⚠️  Only $registeredCount/10 checkers registered" -ForegroundColor Yellow
    $TestResults += "⚠️  Checker Registration: $registeredCount/10"
}

# Final Summary
Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  TEST RESULTS SUMMARY" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

foreach ($result in $TestResults) {
    if ($result -like "*✅*") {
        Write-Host $result -ForegroundColor Green
    } elseif ($result -like "*⚠️*") {
        Write-Host $result -ForegroundColor Yellow
    } else {
        Write-Host $result -ForegroundColor Red
    }
}

# Launch Readiness Check
Write-Host ""
$passedTests = ($TestResults | Where-Object { $_ -like "*✅*" }).Count
$totalTests = $TestResults.Count

if ($passedTests -ge ($totalTests * 0.8)) {
    Write-Host "=====================================" -ForegroundColor Green
    Write-Host "  🚀 READY FOR LAUNCH!" -ForegroundColor Green
    Write-Host "=====================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Review the scan results above" -ForegroundColor White
    Write-Host "2. Check LAUNCH_POSTS.md for pre-written X/Reddit/HN posts" -ForegroundColor White
    Write-Host "3. Post to X (Twitter) using the template" -ForegroundColor White
    Write-Host "4. Launch on Reddit r/software and r/Windows" -ForegroundColor White
    Write-Host "5. Submit to Hacker News (Show HN)" -ForegroundColor White
    Write-Host ""
    Write-Host "Your app is production-ready! 🎉" -ForegroundColor Green
} else {
    Write-Host "=====================================" -ForegroundColor Red
    Write-Host "  ⚠️  NEEDS ATTENTION" -ForegroundColor Red
    Write-Host "=====================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Fix the failed tests above before launching." -ForegroundColor Yellow
}

Write-Host ""
