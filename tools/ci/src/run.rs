use argh::SubCommand;
use bitflags::bitflags;
use xshell::{Cmd, Shell};

pub trait Run {
    fn run<'a>(&self, sh: &'a Shell, flags: RunFlags) -> Vec<RunnableCommand<'a>>;
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RunFlags: u32 {
        const SKIP_FAILURES = 1 << 1;
    }
}

#[derive(Debug)]
pub struct RunnableCommand<'a> {
    // Name of the runnable command
    pub name: String,

    // Command to execute
    pub command: Cmd<'a>,

    // Message to display when the command fails
    pub failure_message: String,

    // Subdirectory to run the command in
    pub subdir: Option<String>,

    // Environment variables to set before running the command
    pub env: Vec<(String, String)>,
}

impl<'a> RunnableCommand<'a> {
    pub fn new<T: SubCommand>(command: Cmd<'a>, failure_message: String) -> Self {
        Self {
            name: String::from(T::COMMAND.name),
            command,
            failure_message,
            subdir: None,
            env: Vec::new(),
        }
    }

    pub fn with_subdir(mut self, subdir: String) -> Self {
        self.subdir = Some(subdir);
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.push((key, value));
        self
    }
}
