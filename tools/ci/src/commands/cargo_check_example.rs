use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-check-example")]
#[argh(description = "run `cargo check` on all examples")]
pub struct CargoCheckExampleCommand {}

impl Run for CargoCheckExampleCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(
                shell,
                "cargo check --workspace --examples --all-features --verbose"
            ),
            String::from("cargo check failed, please fix errors above and try again."),
        )]
    }
}
