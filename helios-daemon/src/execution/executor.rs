use super::job_object::Sandboxjob;
use std::process::Command;

pub fn execute_compiler_task(compiler_path: &str, args: &[String]) -> Result<(i32, String, String), String> {
    let sandbox = SandboxJob::new()?;
    let mut child = Command::new(compiler_path)
        .args(args)
        .spawn()
        .map_err(|e| format!("Failed to start compiler executable ({}): {}", compiler_path, e))?;
    sandbox.assign_process(&child)?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to await compiler execution: {}", e))?;
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((exit_code, stdout, stderr))
}
