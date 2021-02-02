//! An tiny wrapper around `std::process::Command`
use std::io;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct Response {
    pub command: String,
    pub code: Option<i32>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl Response {
    fn from_output(command: String, output: Output) -> Self {
        let status = output.status;
        let success = status.success();
        let code = status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Response {
            command,
            code,
            success,
            stdout,
            stderr,
        }
    }
}

/// Calls a command and returns its result without checking for success
pub fn call(command: impl Into<String>) -> io::Result<Response> {
    let resp = get_response(command)?;
    log::debug!("{:#?}", &resp);
    Ok(resp)
}

/// Calls a command and produces an error on bad exit status
pub fn ensure_call(command: impl Into<String>) -> io::Result<Response> {
    let resp = get_response(command)?;
    if resp.success {
        log::debug!("{:#?}", &resp);
        Ok(resp)
    } else {
        log::error!("{:#?}", &resp);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Unsuccessful call: {:#?}", resp),
        ))
    }
}

pub fn get_response(command: impl Into<String>) -> io::Result<Response> {
    let command = command.into();
    log::info!("Calling: {}", command);
    let output = get_output(&command)?;
    Ok(Response::from_output(command, output))
}

#[cfg(target_os = "linux")]
fn get_output(command: &str) -> io::Result<Output> {
    Command::new("sh").arg("-c").arg(command).output()
}

#[cfg(target_os = "windows")]
fn get_output(command: &str) -> io::Result<Output> {
    Command::new("cmd").arg("/C").arg(&command).output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status() {
        assert!(ensure_call("unknown-command".to_string()).is_err());
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
