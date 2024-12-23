mod ci;

pub use self::ci::CiTools;

pub fn main() {
    argh::from_env::<CiTools>().run();
}
