//! An tiny wrapper on `std::process::Command`
use std::io;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct Response {
    pub command: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Calls a command and returns its result, without checking for success
pub fn call(command: impl AsRef<str>) -> io::Result<Response> {
    let command = command.as_ref().to_owned();
    log::info!("$ {}", command);
    let output = run_command(&command)?;
    let status = output.status;
    let success = status.success();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let resp = Response {
        command,
        success,
        stdout,
        stderr,
    };
    log::debug!("{:#?}", &resp);
    Ok(resp)
}

/// Calls a command and returns an error on bad exit status
pub fn ensure_call(command: impl AsRef<str>) -> io::Result<Response> {
    let resp = call(command)?;
    match resp.success {
        true => Ok(resp),
        false => Err({
            log::error!("{:#?}", &resp);
            io::Error::new(
                io::ErrorKind::Other,
                format!("Error exit status of the command: {:#?}", resp),
            )
        }),
    }
}

#[cfg(target_os = "linux")]
fn run_command(command: &str) -> io::Result<Output> {
    Command::new("sh").arg("-c").arg(&command).output()
}

#[cfg(target_os = "windows")]
fn run_command(command: &str) -> io::Result<Output> {
    Command::new("cmd").arg("/C").arg(&command).output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status() {
        assert!(ensure_call("unknown-command").is_err());
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests_nix {
    use super::*;

    #[test]
    fn test_success_status() {
        assert!(ensure_call("ls").is_ok());
    }

    #[test]
    fn test_error() {
        let r = call("unknown-command").unwrap();
        assert!(!r.success);
        assert!(r.stderr.contains("not found"));
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests_win {
    use super::*;

    #[test]
    fn test_success_status() {
        assert_eq!(call_successfuly("dir").is_ok());
    }

    #[test]
    fn test_error() {
        let r = call("unknown-command").unwrap();
        assert!(!r.success);
        assert!(r.stderr.contains("not recognized"));
    }
}
