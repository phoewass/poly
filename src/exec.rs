use crate::constants;
use std::process;

#[allow(unused)]
#[cfg(unix)]
mod os {
    pub const SHELL: [&str; 2] = ["sh", "-c"];
}

#[allow(unused)]
#[cfg(windows)]
mod os {
    pub const SHELL: [&str; 2] = ["cmd.exe", "/c"];
}

// /// Return a `subprocess::Exec` for a command.
// pub(crate) fn exec_command<S>(command: &[S]) -> Exec
// where
//     S: AsRef<std::ffi::OsStr>,
// {
//     if command.len() > 1 {
//         Exec::cmd(&command[0]).args(&command[1..])
//     } else {
//         Exec::cmd(&command[0])
//     }
// }

pub fn exec_command_in_shell<I, K, V>(
    cmd_str: &str,
    workdir: &std::path::PathBuf,
    env_vars: I,
) -> process::Output
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    // Executes a command as a child process, waiting for it to finish and collecting its status.
    // By default, stdin, stdout and stderr are inherited from the parent.

    let mut cmd = process::Command::new(os::SHELL[0]);
    cmd.args(&os::SHELL[1..])
        .arg(cmd_str)
        .envs(env_vars)
        .current_dir(workdir)
        .env(constants::ENV_PWD, workdir.as_os_str());

    cmd.spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on child");

    cmd.output().expect("output")
    // let output = child
    //     .wait_with_output()
    //     .expect("failed to wait on child");
    // return output;
    // let mut proc: subprocess::Exec = subprocess::Exec::shell(cmd);
    // Exec::cmd(os::SHELL[0]).args(&os::SHELL[1..]).arg(cmdstr);

    // proc = proc.cwd(workdir.clone());
    // // Set $PWD to ensure that commands that are sensitive to it see the right value.
    // proc = proc.env(constants::ENV_PWD, workdir.as_os_str());

    // cmd::stdout_to_string(proc).unwrap_or_default()
}

// /// Return a CaptureData result for a subprocess's stdout.
// pub(crate) fn capture_stdout(
//     exec: subprocess::Exec,
// ) -> Result<subprocess::CaptureData, errors::CommandError> {
//     let command = exec.to_cmdline_lossy();
//     let capture = exec
//         .stdout(subprocess::Redirection::Pipe)
//         .stderr(subprocess::NullFile {}) // Redirect stderr to /dev/null
//         .capture();

//     match capture {
//         Ok(result) => {
//             let status = exit_status(result.exit_status);
//             if status == 0 {
//                 Ok(result)
//             } else {
//                 Err(errors::CommandError { command, status })
//             }
//         }
//         Err(err) => Err(command_error_from_popen_error(command, err)),
//     }
// }

// /// Convert subprocess::ExitStatus into a CommandError
// pub(crate) fn exit_status(status: subprocess::ExitStatus) -> i32 {
//     match status {
//         subprocess::ExitStatus::Exited(status) => status as i32,
//         subprocess::ExitStatus::Signaled(status) => status as i32,
//         subprocess::ExitStatus::Other(status) => status,
//         subprocess::ExitStatus::Undetermined => errors::EX_ERROR,
//     }
// }

// /// Convert a PopenError into a errors::CommandError.
// fn command_error_from_popen_error(
//     command: String,
//     popen_err: subprocess::PopenError,
// ) -> errors::CommandError {
//     let status = match popen_err {
//         subprocess::PopenError::IoError(err) => err.raw_os_error().unwrap_or(1),
//         _ => 1,
//     };
//     errors::CommandError { command, status }
// }
