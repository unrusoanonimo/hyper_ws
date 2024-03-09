use std::process::{Command, Output};

use serde::Serialize;

pub fn execute(cmd: impl AsRef<str>) -> CommandOutput {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd.as_ref()])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd.as_ref())
            .output()
            .expect("failed to execute process")
    };
    CommandOutput::new(output)
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct CommandOutput {
    status: String,
    stderr: String,
    stdout: String,
}
impl CommandOutput {
    pub fn new(o: Output) -> Self {
        Self {
            status: o.status.to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
        }
    }
}
