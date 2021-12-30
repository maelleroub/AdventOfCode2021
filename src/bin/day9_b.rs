fn compute_basin(i: usize, j: usize, grid: &mut Vec<Vec<u32>>) -> i64 {
    let cur = grid[i][j];
    let mut res = 1;
    grid[i][j] = 9;
    if (i >= 1) && (grid[i - 1][j] != 9) {
        res += compute_basin(i - 1, j, grid);
    }
    if (i + 1 < grid.len()) && (grid[i + 1][j] != 9) {
        res += compute_basin(i + 1, j, grid);
    }
    if (j >= 1) && (grid[i][j - 1] != 9) {
        res += compute_basin(i, j - 1, grid);
    }
    if (j + 1 < grid[0].len()) && (grid[i][j + 1] != 9) {
        res += compute_basin(i, j + 1, grid);
    }
    res
}

fn main() {
    let input = include_str!("day9_input");
    let mut grid : Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let row : Vec<u32> = line.chars().map(|x| char::to_digit(x, 10).unwrap()).collect();
        grid.push(row.clone());
    }
    let mut res : Vec<i64> = vec![0,0,0];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 9 {
                let basin = compute_basin(i, j, &mut grid);
                if basin > res[0] {
                    res[0] = basin;
                    res.sort();
                }
            }
        }
    }
    println!("{:?} {}", res, res[0] * res[1] * res[2]);
}
