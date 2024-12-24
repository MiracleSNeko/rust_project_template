mod ci;
mod commands;
mod run;

pub use self::ci::CiTools;
use anyhow::Result;

pub fn main() -> Result<()> {
    argh::from_env::<CiTools>().run()
}
