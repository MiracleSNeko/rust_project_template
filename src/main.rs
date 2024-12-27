fn main() {
    let home = std::env::var("HOME").unwrap();
    for entry in walkdir::WalkDir::new(home + "/.rustup/toolchains")
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        println!("Found toolchain: {}", entry.path().display());
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test1() {
    assert_eq!(add(40, 2), 42);
}

#[test]
fn test2() {
    assert_eq!(String::from("40") + "2", String::from("402"));
}

#[test]
fn test3() {
    assert_eq!(40.0 + 2.0, 42.0);
}

#[test]
fn test4() {
    assert_eq!(42.0, 40 as f64 + 2.0);
}
