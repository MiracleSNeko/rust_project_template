use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-kani")]
#[argh(description = "run `cargo kani` on all targets")]
pub struct CargoKaniCommand {}

impl Run for CargoKaniCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(shell, "cargo kani --workspace --all-features --verbose"),
            String::from("cargo check failed, please fix errors above/below and try again."),
        )]
    }
}
