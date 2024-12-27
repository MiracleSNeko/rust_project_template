use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-doc")]
#[argh(description = "run `cargo doc` on all packages")]
pub struct CargoDocCommand {}

impl Run for CargoDocCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(
                shell,
                "cargo doc --workspace --all-features --no-deps --document-private-items --verbose --keep-going"
            ),
            String::from("cargo doc failed, please fix errors above/below and try again."),
        )]
    }
}
