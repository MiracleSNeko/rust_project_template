use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-check")]
#[argh(description = "run `cargo check` on all packages")]
pub struct CargoCheckCommand {}

impl Run for CargoCheckCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(shell, "cargo check --workspace --all-features --verbose"),
            String::from("cargo check failed"),
        )]
    }
}
