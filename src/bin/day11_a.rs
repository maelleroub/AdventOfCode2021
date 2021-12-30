fn handle(i: usize, j: usize, grid: &mut Vec<Vec<u32>>) -> u64 {
    //println!("{} {}", i, j);
    if grid[i][j] <= 9 || grid[i][j] >= 9999 {
        return 0;
    }
    grid[i][j] = 9999; // Can't flash again during this step
    let mut res = 1;
    if i >= 1 {
        grid[i - 1][j] += 1;
        res += handle(i - 1, j, grid);
        if j >= 1 {
            grid[i - 1][j - 1] += 1;
            res += handle(i - 1, j - 1, grid);
        }
        if j + 1 < grid[0].len() {
            grid[i - 1][j + 1] += 1;
            res += handle(i - 1, j + 1, grid);
        }
    }
    if i + 1 < grid.len() {
        grid[i + 1][j] += 1;
        res += handle(i + 1, j, grid);
        if j >= 1 {
            grid[i + 1][j - 1] += 1;
            res += handle(i + 1, j - 1, grid);
        }
        if j + 1 < grid[0].len() {
            grid[i + 1][j + 1] += 1;
            res += handle(i + 1, j + 1, grid);
        }
    }
    if j >= 1 {
        grid[i][j - 1] += 1;
        res += handle(i, j - 1, grid);
    }
    if j + 1 < grid[0].len() {
        grid[i][j + 1] += 1;
        res += handle(i, j + 1, grid);
    }
    res
}

fn main() {
    let input = include_str!("day11_input");
    let mut grid : Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let row : Vec<u32> = line.chars().map(|x| char::to_digit(x, 10).unwrap()).collect();
        grid.push(row.clone());
    }
    let mut res = 0;
    for _ in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                res += handle(i, j, &mut grid);
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] >= 9999 {
                    grid[i][j] = 0;
                }
            }
        }
    }
    println!("{}", res);
}
