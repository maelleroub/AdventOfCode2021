fn main() {
    let input = include_str!("day3_input");
    let numbers : Vec<&str> = input.lines().collect();
    
    let nb_number = numbers.len();
    let size_number = numbers[0].len();

    let mut occurrences = vec![0; size_number];

    for number in numbers {
        for (i, c) in number.chars().enumerate() {
            if c == '1' {
                occurrences[i] += 1;
            }
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for i in 0..size_number {
        if occurrences[i] >= (nb_number / 2) {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
        else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma_int = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = u64::from_str_radix(&epsilon, 2).unwrap();

    println!("{} * {} = {}", gamma_int, epsilon_int, gamma_int * epsilon_int);
}
