// cow1.rs
//
// This exercise explores the Clone-on-write (Cow) type, which can hold either
// borrowed or owned data. Your task is to fill in the two TODO sections to make
// the tests pass.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.



use std::borrow::Cow;

fn main() {
    // TODO: Create a Cow that borrows the string
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    
    // TODO: Create a Cow that owns the string
    let owned: Cow<str> = Cow::Owned(String::from("world"));
    
    // Test borrowed variant
    assert!(matches!(borrowed, Cow::Borrowed(_)), "Should be borrowed");
    assert_eq!(borrowed, "hello");
    
    // Test owned variant
    assert!(matches!(owned, Cow::Owned(_)), "Should be owned");
    assert_eq!(owned, "world");
}