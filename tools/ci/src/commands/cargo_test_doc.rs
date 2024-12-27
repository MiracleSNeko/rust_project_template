use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-test-doc")]
#[argh(description = "run `cargo test` on all documentation")]
pub struct CargoTestDocCommand {}

impl Run for CargoTestDocCommand {
    fn run<'a>(&self, shell: &'a Shell, flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        let continue_on_failure = flags
            .contains(RunFlags::SKIP_FAILURES)
            .then_some("--no-fail-fast")
            .unwrap_or_default();

        vec![RunnableCommand::new::<Self>(
            cmd!(shell, "cargo test --workspace --doc {continue_on_failure}"),
            String::from("cargo test failed, please fix errors above and try again."),
        )]
    }
}
