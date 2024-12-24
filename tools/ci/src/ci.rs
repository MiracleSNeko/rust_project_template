//! This CI command line tool is copied from Bevy project.

use crate::{
    commands::CargoCheckCommand,
    run::{Run, RunFlags, RunnableCommand},
};
use anyhow::Result;
use argh::FromArgs;
use xshell::Shell;

#[derive(FromArgs)]
#[argh(description = "the CI command line tool.")]
pub struct CiTools {
    #[argh(subcommand)]
    pub command: Option<Command>,

    #[argh(switch)]
    #[argh(description = "skip failures and continue running CI commands.")]
    pub skip_failures: bool,
}

impl CiTools {
    pub fn run(&self) -> Result<()> {
        let shell = Shell::new()?;

        let mut failures = vec![];

        for command in self.get_commands(&shell) {
            // Will step into the subdirectory permanently if it is set
            let _ = command.subdir.map(|path| shell.push_dir(path));

            if command.command.envs(command.env).run().is_err() {
                failures.push(format!(
                    "- {name}: {message}",
                    name = command.name,
                    message = command.failure_message
                ));

                if !self.skip_failures {
                    break;
                }
            }
        }

        if !failures.is_empty() {
            panic!("The following commands failed:\n{}", failures.join("\n"));
        }

        Ok(())
    }

    pub fn get_commands<'a>(&self, sh: &'a Shell) -> Vec<RunnableCommand<'a>> {
        let mut flags = RunFlags::empty();
        if self.skip_failures {
            flags |= RunFlags::SKIP_FAILURES;
        }

        match &self.command {
            Some(command) => command.run(sh, flags),
            None => vec![],
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    CargoCheck(CargoCheckCommand),
}

impl Run for Command {
    fn run<'a>(&self, sh: &'a Shell, flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        match self {
            Command::CargoCheck(command) => command.run(sh, flags),
        }
    }
}
