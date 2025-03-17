use rand::Rng;

#[allow(dead_code)]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(10..99)).collect()
}

#[allow(dead_code)]
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (pair[0] + pair[1], i, i + 1))
        .min_by_key(|&(sum, _, _)| sum)
        .unwrap()
}

#[allow(dead_code)]
fn print_result(data: &[i32]) {
    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    
    println!("\nindexes: {}",&(0..data.len()).map(|i| format!("{:2}.", i)).collect::<Vec<_>>().join(" "));
    println!("data:   [{}]", data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
    println!("indexes: {}\\__ __/{}", " ".repeat(idx1 * 3), " ".repeat((data.len() - idx2 - 1) * 3));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22];
        let (sum, idx1, idx2) = min_adjacent_sum(&data);
        assert_eq!(sum, 82);
        assert_eq!((idx1, idx2), (5, 6));
    }
}
