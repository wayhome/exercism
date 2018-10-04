pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = vec![];
    let rows = input.len();
    for (rindex, row) in input.iter().enumerate() {
        let cols = row.len();
        for (cindex, col) in row.iter().enumerate() {
            if (0..cols).all(|c| row[c] <= *col) && (0..rows).all(|x| input[x][cindex] >= * col) {
                saddle_points.push((rindex, cindex))
            }
        }
    }
    saddle_points
}
