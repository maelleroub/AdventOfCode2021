fn main() {
    let input = include_str!("day1_input");
    let measurements : Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut previous = std::i32::MAX;
    let mut nb_increases = 0;

    for window in measurements.windows(3) {
        let sum = window.iter().sum::<i32>();
        if sum > previous {
            nb_increases += 1;
        }
        previous = sum;
    }
    println!("Number of increases: {}", nb_increases);
}
