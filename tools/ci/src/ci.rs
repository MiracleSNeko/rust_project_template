use argh::FromArgs;

#[derive(FromArgs)]
pub struct CiTools {
    #[argh(subcommand)]
    pub subcommand: Option<Subcommand>,
}

impl CiTools {
    pub fn run(&self) {}
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Subcommand {
    Build,
}
