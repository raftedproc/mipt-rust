use super::{command::Command, context::CommandContext};
use anyhow::{bail, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

const FORBID_UNSAFE_LINE: &str = "#![forbid(unsafe_code)]";

macro_rules! launch {
    ($toolchain: expr, $command: expr, $context: expr) => {{
        let toolchain_shell_line = $toolchain.get_shell_line()?;
        let command_shell_line = $command.get_shell_line()?;
        let toolchain_shell_iter = toolchain_shell_line.split(' ');
        let command_shell_iter = command_shell_line.split(' ');
        let mut iter = toolchain_shell_iter.chain(command_shell_iter);
        let mut cmd = if let Some(program) = iter.next() {
            let mut cmd = process::Command::new(program);
            cmd.current_dir($context.get_workdir());
            cmd
        } else {
            bail!("toolchain and command are empty")
        };
        while let Some(arg) = iter.next() {
            cmd.arg(arg);
        }
        if cmd.status().context("command failed")?.success() {
            Ok(())
        } else {
            bail!("command failed")
        }
    }};
}

#[derive(Clone, Copy, Debug)]
pub enum Toolchain {
    Empty,
    Stable,
    Nightly,
}

impl Toolchain {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "empty" => Self::Empty,
            "stable" => Self::Stable,
            "nightly" => Self::Nightly,
            name => bail!("toolchain \"{name}\" is not supported"),
        })
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::Empty => "".to_string(),
            Self::Stable => "rustup run stable".to_string(),
            Self::Nightly => "rustup run nightly".to_string(),
        })
    }

    pub fn run_command(&self, command: &Command, context: &CommandContext) -> Result<()> {
        match command {
            Command::ForbidUnsafe => {
                for file in context.get_user_files() {
                    if let Some(line) = BufReader::new(File::open(file)?)
                        .lines()
                        .next()
                        .transpose()?
                    {
                        if line != FORBID_UNSAFE_LINE {
                            bail!(format!(
                                "file {file:?} doesn't contain line '{FORBID_UNSAFE_LINE}'"
                            ))
                        }
                    } else {
                        // TODO: ForbidUnsafe shouldn't check whether file is empty
                        bail!(format!(
                            "file {file:?} is empty"
                        ))
                    }
                }
                Ok(())
            }
            Command::CargoFmt | Command::CargoClippy | Command::CargoTest | Command::PythonTest => {
                launch!(self, command, context)
            }
        }
    }
}
