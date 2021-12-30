fn is_low(i: usize, j:usize, grid: &Vec<Vec<u32>>) -> bool {
    let cur = grid[i][j];
    if (i >= 1) && (grid[i - 1][j] <= cur) {
        return false;
    }
    if (i + 1 < grid.len()) && (grid[i + 1][j] <= cur) {
        return false;
    }
    if (j >= 1) && (grid[i][j - 1] <= cur) {
        return false;
    }
    if (j + 1 < grid[0].len()) && (grid[i][j + 1] <= cur) {
        return false;
    }
    true
}

fn main() {
    let input = include_str!("day9_input");
    let mut grid : Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let row : Vec<u32> = line.chars().map(|x| char::to_digit(x, 10).unwrap()).collect();
        grid.push(row.clone());
    }
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if is_low(i, j, &grid) {
                res += grid[i][j] + 1;
            }
        }
    }
    println!("{}", res);
}
