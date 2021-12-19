fn main() {
    let input = include_str!("day1_input");
    let measurements : Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut previous = std::i32::MAX;
    let mut nb_increases = 0;

    for measurement in measurements {
        if measurement > previous {
            nb_increases += 1;
        }
        previous = measurement;
    }
    println!("Number of increases: {}", nb_increases);
}
