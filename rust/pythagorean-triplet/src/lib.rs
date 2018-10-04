pub fn find() -> Option<u32> {
    let target = 1000;

    for a in 3..target {
        for b in (a + 1)..(target - a) {
            let c = target - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }

    None
}
