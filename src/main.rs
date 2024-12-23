fn main() {
    println!("Hello, world!");
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
