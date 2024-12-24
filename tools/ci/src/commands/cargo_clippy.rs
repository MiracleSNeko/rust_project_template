use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-clippy")]
#[argh(description = "run `cargo clippy` on all packages")]
pub struct CargoClippyCommand {}

impl Run for CargoClippyCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(
                shell,
                "cargo check --workspace --all-targets --all-features --verbose -- -Dwarnings"
            ),
            String::from("cargo clippy failed, please fix errors above and try again."),
        )]
    }
}
