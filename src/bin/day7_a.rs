fn main() {
    let input = include_str!("day7_input");
    let mut lines = input.lines();
    let mut positions : Vec<i64> = lines.next().unwrap().split(",").map(|word| word.parse().unwrap()).collect();
    positions.sort();
    let total : usize = positions.len();
    let median = positions[total / 2];

    let mut fuel = 0;
    for position in positions {
        fuel += i64::abs(position - median);
    }
    println!("Fuel: {}", fuel);
}
