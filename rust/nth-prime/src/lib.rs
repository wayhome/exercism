pub fn nth(n: usize) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut primes: Vec<usize> = vec![2];
    let mut last = primes.last().unwrap().clone();
    while primes.len() < n {
        last = last + 1;
        if !primes.iter().any(|x| last % x == 0 ) {
            primes.push(last);
        }
    };
    return Some(primes[n-1]);
}
