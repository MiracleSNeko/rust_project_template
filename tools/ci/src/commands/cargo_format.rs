use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-format")]
#[argh(description = "run `cargo fmt` on all packages")]
pub struct CargoFormatCommand {}

impl Run for CargoFormatCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(shell, "cargo fmt --all -- --check"),
            String::from("cargo fmt failed, please fix errors above and try again."),
        )]
    }
}