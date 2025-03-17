#[allow(dead_code)]
fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        for (n, expected) in data.iter() {
            assert_eq!(is_palindrome(*n), *expected);
        }
    }
}
