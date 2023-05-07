#[allow(unused_macros)]
macro_rules! greeter {
    ($( $n:expr ),+) => {
        vec![$(format!("Hello, {}!", $n) ),+].join(" ")
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_greeter() {
        assert_eq!(greeter!("world"), "Hello, world!");
    }

    #[test]
    fn test_multi_greeter() {
        assert_eq!(
            greeter!("a", "b", "c"),
            "Hello, a! Hello, b! Hello, c!"
        );
    }
}
