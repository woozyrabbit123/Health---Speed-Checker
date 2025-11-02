@echo off
echo =====================================
echo  QUICK TEST - Health Speed Checker
echo =====================================
echo.

cd agent
echo [1/3] Testing Rust build...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo FAILED: Rust build error
    pause
    exit /b 1
)
echo SUCCESS: Rust build passed
echo.

echo [2/3] Running scan test...
cargo run --release -- scan --output json > scan_output.json 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo FAILED: Scan error
    type scan_output.json
    pause
    exit /b 1
)
echo SUCCESS: Scan completed
echo.

echo [3/3] Checking results...
findstr /C:"health" scan_output.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo FAILED: No scan results found
    pause
    exit /b 1
)

echo.
echo =====================================
echo  ALL TESTS PASSED - READY TO LAUNCH
echo =====================================
echo.
echo Next steps:
echo 1. Check LAUNCH_POSTS.md for X/Reddit posts
echo 2. Post to X (Twitter)
echo 3. Post to Reddit r/software
echo 4. Submit to Hacker News
echo.
echo Scan results saved to: agent\scan_output.json
echo.
pause
