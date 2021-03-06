// Copyright 2020 Square Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

use super::IoEnv;
use crate::errors::Result;

use std::io::Write;

#[warn(clippy::missing_inline_in_public_items)]

/// The trait that defines the implementation of a sudo I/O plugin.
pub trait IoPlugin: Sized {
    /// The name of the plugin. Used when printing the version of the
    /// plugin and error messages.
    const NAME: &'static str;

    /// The version of the plugin. Defaults to the the value of the
    /// `CARGO_PKG_VERSION` environment variable during build.
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// The `sudo_plugin` facility sets `iolog_{facility}` hints that I
    /// believe come from whether or not `LOG_INPUT` or `LOG_OUTPUT` are
    /// set. By default, plugins will not have their logging callbacks
    /// invoked if `sudo` has told us not to log.
    ///
    /// Setting this to `true` will ignore these hints and always call
    /// user-provided `log_*` callbacks.
    const IGNORE_IOLOG_HINTS: bool = false;

    /// Prints the name and version of the plugin. A default
    /// implementation of this function is provided, but may be
    /// overridden if desired.
    #[inline]
    fn show_version(env: &'static IoEnv, _verbose: bool) {
        let _ = writeln!(
            env.stdout(),
            "{} I/O plugin version {}",
            Self::NAME,
            Self::VERSION,
        );
    }

    /// The `open` function is run before the `log_ttyin`, `log_ttyout`,
    /// `log_stdin`, `log_stdout`, `log_stderr`, `log_suspend`, or
    /// `change_winsize` methods are called. It is only called if the
    /// policy plugin's `check_policy` function has returned
    /// successfully.
    ///
    /// # Errors
    ///
    /// If this method returns an error, the command will be terminated.
    fn open(env: &'static IoEnv) -> Result<Self>;

    /// The `close` method is called when the command being run by
    /// sudo finishes. A default no-op implementation is provided, but
    /// be overriden if desired.
    ///
    /// As suggested by its signature, once this method exits, the
    /// plugin will be dropped.
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn close(self, _exit_status: i32, _error: i32) {}

    /// The `log_ttyin` method is called whenever data can be read from the user
    /// but before it is passed to the running command. This allows the plugin
    /// to reject data if it chooses to (for instance if the input contains
    /// banned content).
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn log_ttyin(&mut self, _log: &[u8]) -> Result<()> {
        Ok(())
    }

    /// The `log_ttyout` function is called whenever data can be read from the
    /// command but before it is written to the user's terminal. This allows the
    /// plugin to reject data if it chooses to (for instance if the output
    /// contains banned content).
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn log_ttyout(&mut self, _log: &[u8]) -> Result<()> {
        Ok(())
    }

    /// The `log_stdin` function is only used if the standard input does not
    /// correspond to a tty device. It is called whenever data can be read from
    /// the standard input but before it is passed to the running command.
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn log_stdin(&mut self, _log: &[u8]) -> Result<()> {
        Ok(())
    }

    /// The `log_stdout` function is only used if the standard output does not
    /// correspond to a tty device. It is called whenever data can be read from
    /// the command but before it is written to the standard output. This allows
    /// the plugin to reject data if it chooses to (for instance if the output
    /// contains banned content).
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn log_stdout(&mut self, _log: &[u8]) -> Result<()> {
        Ok(())
    }

    /// The `log_stderr` function is only used if the standard error does not
    /// correspond to a tty device. It is called whenever data can be read from
    /// the command but before it is written to the standard error. This allows
    /// the plugin to reject data if it chooses to (for instance if the output
    /// contains banned content).
    ///
    /// # Errors
    ///
    /// If this method returns an `Err(ErrorKind::Unauthorized)`, the command
    /// will be terminated. Other errors are ignored by `sudo`.
    #[inline]
    fn log_stderr(&mut self, _log: &[u8]) -> Result<()> {
        Ok(())
    }

    // TODO: support for `change_winsize`
    // TODO: support for `log_suspend`
}
