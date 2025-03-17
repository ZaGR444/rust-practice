fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    
    let n = (n.rem_euclid(len as isize)) as usize;
    let rotated = [&s[len - n..], &s[..len - n]].concat();
    rotated
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];
        
        for (n, expected) in shifts.iter() {
            assert_eq!(rotate(s.clone(), *n), expected.to_string());
        }
    }
}
