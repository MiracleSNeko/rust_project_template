use crate::run::{Run, RunFlags, RunnableCommand};
use argh::FromArgs;
use walkdir::WalkDir;
use xshell::{cmd, Shell};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "cargo-mirai")]
#[argh(description = "run `cargo mirai` on all packages")]
pub struct CargoMiraiCommand {}

impl Run for CargoMiraiCommand {
    fn run<'a>(&self, shell: &'a Shell, _flags: RunFlags) -> Vec<RunnableCommand<'a>> {
        // NOTE: The `cargo mirai` command will fail with the error "$HOME/.cargo/bin/mirai: error while loading shared libraries: librustc_driver-xxxxxxxxxxxxxxxx.so: cannot open shared object file: No such file or directory" unless we add "$HOME/.rustup/toolchains/nightly-${specific version installed by miri}/lib/" to the `LD_LIBRARY_PATH` and execute the command as `LD_LIBRARY_PATH=$LD_LIBRARY_PATH cargo mirai`.
        let library_dir = WalkDir::new(std::env::var("HOME").unwrap() + "/.rustup/toolchains")
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .fold(String::from("$LD_LIBRARY_PATH"), |library_dir, e| {
                format!("{}:{}/lib/", library_dir, e.path().display())
            });

        vec![
            RunnableCommand::new::<Self>(
                cmd!(shell, "cargo mirai").env("LD_LIBRARY_PATH", &library_dir),
                String::from("cargo mirai failed, please fix errors above/below and try again."),
            ),
            RunnableCommand::new::<Self>(
                cmd!(shell, "cargo mirai --tests").env("LD_LIBRARY_PATH", &library_dir),
                String::from("cargo mirai failed, please fix errors above/below and try again."),
            ),
        ]
    }
}
