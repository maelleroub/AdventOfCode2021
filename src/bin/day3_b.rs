fn main() {
    let input = include_str!("day3_input");
    let numbers : Vec<&str> = input.lines().collect();
    
    let size_number = numbers[0].len();

    let mut oxygen_candidates = numbers.clone();

    for i in 0..size_number {
        if oxygen_candidates.len() <= 1 {
            break;
        }

        let mut occurrences = 0;
        for candidate in oxygen_candidates.iter() {
            if candidate.chars().nth(i).unwrap() == '1' {
                occurrences += 1;
            }
        }
        let most_frequent_bit = if occurrences as f64 >= (oxygen_candidates.len() as f64 / 2.0) { '1' } else { '0' };
        oxygen_candidates = oxygen_candidates.into_iter().filter(|candidate| candidate.chars().nth(i).unwrap() == most_frequent_bit).collect();
    }
    let oxygen = oxygen_candidates.pop().unwrap();
    println!("{}", oxygen);
    let oxygen_int = u64::from_str_radix(&oxygen, 2).unwrap();

    let mut co2_candidates = numbers.clone();

    for i in 0..size_number {
        if co2_candidates.len() <= 1 {
            break;
        }

        let mut occurrences = 0;
        for candidate in co2_candidates.iter() {
            if candidate.chars().nth(i).unwrap() == '1' {
                occurrences += 1;
            }
        }
        let least_frequent_bit = if occurrences as f64 >= (co2_candidates.len() as f64 / 2.0) { '0' } else { '1' };
        co2_candidates = co2_candidates.into_iter().filter(|candidate| candidate.chars().nth(i).unwrap() == least_frequent_bit).collect();
    }
    let co2 = co2_candidates.pop().unwrap();
    println!("{}", co2);
    let co2_int = u64::from_str_radix(&co2, 2).unwrap();

    println!("Oxygen generator * CO2 scrubber = {} * {} = {}", oxygen, co2, oxygen_int * co2_int);
}
