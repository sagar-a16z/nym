// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
  collections::HashMap,
  io::{BufRead, BufReader, Write},
  path::PathBuf,
  process::{Command as StdCommand, Stdio},
  sync::{Arc, Mutex, RwLock},
  thread::spawn,
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

use crate::async_runtime::{block_on as block_on_task, channel, Receiver};
use os_pipe::{pipe, PipeWriter};
use serde::Serialize;
use shared_child::SharedChild;
use tauri_utils::platform;

type ChildStore = Arc<Mutex<HashMap<u32, Arc<SharedChild>>>>;

fn commands() -> &'static ChildStore {
  use once_cell::sync::Lazy;
  static STORE: Lazy<ChildStore> = Lazy::new(Default::default);
  &STORE
}

/// Kills all child processes created with [`Command`].
/// By default it's called before the [`crate::App`] exits.
pub fn kill_children() {
  for child in commands().lock().unwrap().values() {
    let _ = child.kill();
  }
}

/// Payload for the [`CommandEvent::Terminated`] command event.
#[derive(Debug, Clone, Serialize)]
pub struct TerminatedPayload {
  /// Exit code of the process.
  pub code: Option<i32>,
  /// If the process was terminated by a signal, represents that signal.
  pub signal: Option<i32>,
}

/// A event sent to the command callback.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", content = "payload")]
#[non_exhaustive]
pub enum CommandEvent {
  /// Stderr line.
  Stderr(String),
  /// Stdout line.
  Stdout(String),
  /// An error happened.
  Error(String),
  /// Command process terminated.
  Terminated(TerminatedPayload),
}

macro_rules! get_std_command {
  ($self: ident) => {{
    let mut command = StdCommand::new($self.program);
    command.args(&$self.args);
    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());
    command.stderr(Stdio::piped());
    if $self.env_clear {
      command.env_clear();
    }
    command.envs($self.env);
    if let Some(current_dir) = $self.current_dir {
      command.current_dir(current_dir);
    }
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    command
  }};
}

/// The type to spawn commands.
#[derive(Debug)]
pub struct Command {
  program: String,
  args: Vec<String>,
  env_clear: bool,
  env: HashMap<String, String>,
  current_dir: Option<PathBuf>,
}

/// Spawned child process.
#[derive(Debug)]
pub struct CommandChild {
  inner: Arc<SharedChild>,
  stdin_writer: PipeWriter,
}

impl CommandChild {
  /// Writes to process stdin.
  pub fn write(&mut self, buf: &[u8]) -> crate::api::Result<()> {
    self.stdin_writer.write_all(buf)?;
    Ok(())
  }

  /// Sends a kill signal to the child.
  pub fn kill(self) -> crate::api::Result<()> {
    self.inner.kill()?;
    Ok(())
  }

  /// Returns the process pid.
  pub fn pid(&self) -> u32 {
    self.inner.id()
  }
}

/// Describes the result of a process after it has terminated.
#[derive(Debug)]
pub struct ExitStatus {
  code: Option<i32>,
}

impl ExitStatus {
  /// Returns the exit code of the process, if any.
  pub fn code(&self) -> Option<i32> {
    self.code
  }

  /// Returns true if exit status is zero. Signal termination is not considered a success, and success is defined as a zero exit status.
  pub fn success(&self) -> bool {
    self.code == Some(0)
  }
}

/// The output of a finished process.
#[derive(Debug)]
pub struct Output {
  /// The status (exit code) of the process.
  pub status: ExitStatus,
  /// The data that the process wrote to stdout.
  pub stdout: String,
  /// The data that the process wrote to stderr.
  pub stderr: String,
}

fn relative_command_path(command: String) -> crate::Result<String> {
  match platform::current_exe()?.parent() {
    #[cfg(windows)]
    Some(exe_dir) => Ok(format!("{}\\{}.exe", exe_dir.display(), command)),
    #[cfg(not(windows))]
    Some(exe_dir) => Ok(format!("{}/{}", exe_dir.display(), command)),
    None => Err(crate::api::Error::Command("Could not evaluate executable dir".to_string()).into()),
  }
}

impl Command {
  /// Creates a new Command for launching the given program.
  pub fn new<S: Into<String>>(program: S) -> Self {
    Self {
      program: program.into(),
      args: Default::default(),
      env_clear: false,
      env: Default::default(),
      current_dir: None,
    }
  }

  /// Creates a new Command for launching the given sidecar program.
  ///
  /// A sidecar program is a embedded external binary in order to make your application work
  /// or to prevent users having to install additional dependencies (e.g. Node.js, Python, etc).
  pub fn new_sidecar<S: Into<String>>(program: S) -> crate::Result<Self> {
    Ok(Self::new(relative_command_path(program.into())?))
  }

  /// Appends arguments to the command.
  #[must_use]
  pub fn args<I, S>(mut self, args: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    for arg in args {
      self.args.push(arg.as_ref().to_string());
    }
    self
  }

  /// Clears the entire environment map for the child process.
  #[must_use]
  pub fn env_clear(mut self) -> Self {
    self.env_clear = true;
    self
  }

  /// Adds or updates multiple environment variable mappings.
  #[must_use]
  pub fn envs(mut self, env: HashMap<String, String>) -> Self {
    self.env = env;
    self
  }

  /// Sets the working directory for the child process.
  #[must_use]
  pub fn current_dir(mut self, current_dir: PathBuf) -> Self {
    self.current_dir.replace(current_dir);
    self
  }

  /// Spawns the command.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use tauri::api::process::{Command, CommandEvent};
  /// tauri::async_runtime::spawn(async move {
  ///   let (mut rx, mut child) = Command::new("cargo")
  ///     .args(["tauri", "dev"])
  ///     .spawn()
  ///     .expect("Failed to spawn cargo");
  ///
  ///   let mut i = 0;
  ///   while let Some(event) = rx.recv().await {
  ///     if let CommandEvent::Stdout(line) = event {
  ///       println!("got: {}", line);
  ///       i += 1;
  ///       if i == 4 {
  ///         child.write("message from Rust\n".as_bytes()).unwrap();
  ///         i = 0;
  ///       }
  ///     }
  ///   }
  /// });
  /// ```
  pub fn spawn(self) -> crate::api::Result<(Receiver<CommandEvent>, CommandChild)> {
    let mut command = get_std_command!(self);
    let (stdout_reader, stdout_writer) = pipe()?;
    let (stderr_reader, stderr_writer) = pipe()?;
    let (stdin_reader, stdin_writer) = pipe()?;
    command.stdout(stdout_writer);
    command.stderr(stderr_writer);
    command.stdin(stdin_reader);

    let shared_child = SharedChild::spawn(&mut command)?;
    let child = Arc::new(shared_child);
    let child_ = child.clone();
    let guard = Arc::new(RwLock::new(()));

    commands().lock().unwrap().insert(child.id(), child.clone());

    let (tx, rx) = channel(1);

    let tx_ = tx.clone();
    let guard_ = guard.clone();
    spawn(move || {
      let _lock = guard_.read().unwrap();
      let reader = BufReader::new(stdout_reader);
      for line in reader.lines() {
        let tx_ = tx_.clone();
        block_on_task(async move {
          let _ = match line {
            Ok(line) => tx_.send(CommandEvent::Stdout(line)).await,
            Err(e) => tx_.send(CommandEvent::Error(e.to_string())).await,
          };
        });
      }
    });

    let tx_ = tx.clone();
    let guard_ = guard.clone();
    spawn(move || {
      let _lock = guard_.read().unwrap();
      let reader = BufReader::new(stderr_reader);
      for line in reader.lines() {
        let tx_ = tx_.clone();
        block_on_task(async move {
          let _ = match line {
            Ok(line) => tx_.send(CommandEvent::Stderr(line)).await,
            Err(e) => tx_.send(CommandEvent::Error(e.to_string())).await,
          };
        });
      }
    });

    spawn(move || {
      let _ = match child_.wait() {
        Ok(status) => {
          let _l = guard.write().unwrap();
          commands().lock().unwrap().remove(&child_.id());
          let _ = block_on_task(async move {
            tx.send(CommandEvent::Terminated(TerminatedPayload {
              code: status.code(),
              #[cfg(windows)]
              signal: None,
              #[cfg(unix)]
              signal: status.signal(),
            }))
            .await
          });
        }
        Err(e) => {
          let _l = guard.write().unwrap();
          let _ = block_on_task(async move { tx.send(CommandEvent::Error(e.to_string())).await });
        }
      };
    });

    Ok((
      rx,
      CommandChild {
        inner: child,
        stdin_writer,
      },
    ))
  }

  /// Executes a command as a child process, waiting for it to finish and collecting its exit status.
  /// Stdin, stdout and stderr are ignored.
  ///
  /// # Examples
  /// ```rust,no_run
  /// use tauri::api::process::Command;
  /// let status = Command::new("which").args(["ls"]).status().unwrap();
  /// println!("`which` finished with status: {:?}", status.code());
  /// ```
  pub fn status(self) -> crate::api::Result<ExitStatus> {
    let (mut rx, _child) = self.spawn()?;
    let code = crate::async_runtime::safe_block_on(async move {
      let mut code = None;
      #[allow(clippy::collapsible_match)]
      while let Some(event) = rx.recv().await {
        if let CommandEvent::Terminated(payload) = event {
          code = payload.code;
        }
      }
      code
    });
    Ok(ExitStatus { code })
  }

  /// Executes the command as a child process, waiting for it to finish and collecting all of its output.
  /// Stdin is ignored.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use tauri::api::process::Command;
  /// let output = Command::new("echo").args(["TAURI"]).output().unwrap();
  /// assert!(output.status.success());
  /// assert_eq!(output.stdout, "TAURI");
  /// ```
  pub fn output(self) -> crate::api::Result<Output> {
    let (mut rx, _child) = self.spawn()?;

    let output = crate::async_runtime::safe_block_on(async move {
      let mut code = None;
      let mut stdout = String::new();
      let mut stderr = String::new();
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            code = payload.code;
          }
          CommandEvent::Stdout(line) => {
            stdout.push_str(line.as_str());
            stdout.push('\n');
          }
          CommandEvent::Stderr(line) => {
            stderr.push_str(line.as_str());
            stderr.push('\n');
          }
          CommandEvent::Error(_) => {}
        }
      }
      Output {
        status: ExitStatus { code },
        stdout,
        stderr,
      }
    });

    Ok(output)
  }
}

// tests for the commands functions.
#[cfg(test)]
mod test {
  #[cfg(not(windows))]
  use super::*;

  #[cfg(not(windows))]
  #[test]
  fn test_cmd_output() {
    // create a command to run cat.
    let cmd = Command::new("cat").args(&["test/api/test.txt"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    crate::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(0));
          }
          CommandEvent::Stdout(line) => {
            assert_eq!(line, "This is a test doc!".to_string());
          }
          _ => {}
        }
      }
    });
  }

  #[cfg(not(windows))]
  #[test]
  // test the failure case
  fn test_cmd_fail() {
    let cmd = Command::new("cat").args(&["test/api/"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    crate::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(1));
          }
          CommandEvent::Stderr(line) => {
            assert_eq!(line, "cat: test/api/: Is a directory".to_string());
          }
          _ => {}
        }
      }
    });
  }
}
