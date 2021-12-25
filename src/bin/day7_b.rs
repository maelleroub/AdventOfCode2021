fn main() {
    let input = include_str!("day7_input");
    let mut lines = input.lines();
    let mut positions : Vec<i64> = lines.next().unwrap().split(",").map(|word| word.parse().unwrap()).collect();
    positions.sort();
    let max_pos = *positions.last().unwrap();
    let mut min_fuel = std::i64::MAX;

    // Simply bruteforce all possible positions
    for i in 0..max_pos {
        let mut fuel = 0;
        for position in positions.iter() {
            let dist = i64::abs(position - i);
            fuel += dist * (dist + 1) / 2;
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("Fuel: {}", min_fuel);
}
