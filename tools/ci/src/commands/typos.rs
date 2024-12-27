use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "typo")]
#[argh(description = "run `typo` on all packages")]
pub struct TypoCommand {}

impl Run for TypoCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        vec![RunnableCommand::new::<Self>(
            cmd!(shell, "typo"),
            String::from("seems like there are some typos in your code , please fix errors above/below and try again."),
        )]
    }
}
