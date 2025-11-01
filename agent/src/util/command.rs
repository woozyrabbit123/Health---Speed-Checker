use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;

/// Run a command with a timeout, returning Ok(Output) if the process completes
/// within the duration or an Err string on timeout or spawn error.
pub fn run_with_timeout(mut cmd: Command, timeout: Duration) -> Result<Output, String> {
    let child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn: {}", e))?;

    // Use a thread to wait on the child with a timeout
    let pid = child.id();
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let out = child.wait_with_output();
        let _ = tx.send(out);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(format!("failed to wait: {}", e)),
        Err(_) => {
            // Timeout: best effort to terminate
            #[cfg(unix)]
            {
                let _ = nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::SIGKILL);
            }
            #[cfg(windows)]
            {
                // On Windows, use taskkill as a best effort fallback
                let _ = Command::new("taskkill").args(["/PID", &pid.to_string(), "/F"]).output();
            }
            Err("process timeout".to_string())
        }
    }
}
