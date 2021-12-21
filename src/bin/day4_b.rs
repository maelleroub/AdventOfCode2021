fn create_grid(lines : &mut std::str::Lines) -> Vec<Vec<i32>> {
    let mut grid : Vec<Vec<i32>> = Vec::new();
    for _ in 0..5 {
        let row : Vec<i32> = lines.next().unwrap().split_whitespace().map(|word| word.parse().unwrap()).collect();
        grid.push(row);
    }
    grid
}

fn mark_grid(grid: &mut Vec<Vec<i32>>, nb : i32) {
    for i in 0..5 {
        for j in 0..5 {
            if grid[i][j] == nb {
                grid[i][j] = 9999;
            }
        }
    }
}

fn is_grid_over(grid: &Vec<Vec<i32>>) -> bool {
    for i in 0..5 {
        let mut horizontal_over = true;
        let mut vertical_over = true;
        for j in 0..5 {
            if grid[i][j] != 9999 {
                horizontal_over = false;
            }
            if grid[j][i] != 9999 {
                vertical_over = false;
            }
        }
        if horizontal_over || vertical_over {
            return true;
        }
    }
    false
}

fn compute_score(grid: &Vec<Vec<i32>>, nb : i32) -> i64 {
    let mut sum : i64 = 0;
    for row in grid {
        for number in row {
            if *number != 9999 {
                sum += *number as i64;
            }
        }
    }
    sum * nb as i64
}

fn main() {
    let input = include_str!("day4_input");
    let mut lines = input.lines();

    let draws : Vec<i32> = lines.next().unwrap().split(",").map(|word| word.parse().unwrap()).collect();
    let mut grids : Vec<Vec<Vec<i32>>> = Vec::new();
    let mut grids_over : Vec<usize> = Vec::new();

    while lines.next() != None {
        grids.push(create_grid(&mut lines));
    }

   for draw in draws {
       let len = grids.len();
       for i in 0..len {
           if grids_over.contains(&i) {
               continue;
           }
           let mut grid = &mut grids[i];
           mark_grid(&mut grid, draw);
           if is_grid_over(&grid) {
               grids_over.push(i);
               if grids_over.len() == len {
                   let res = compute_score(&grid, draw);
                   println!("{}", res);
                   return;
               }
           }
       }
   }
}
