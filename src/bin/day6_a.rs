fn main() {
    let input = include_str!("day6_input");
    let mut lines = input.lines();
    let initial_states : Vec<usize> = lines.next().unwrap().split(",").map(|word| word.parse().unwrap()).collect();

    let mut occurences = [0u32; 9];
    for state in initial_states {
        occurences[state] += 1;
    }

    for _ in 0..80 {
        let mut next_occurences = [0u32; 9];
        for i in 0..8 {
            next_occurences[i] = occurences[i + 1];
        }
        next_occurences[8] = occurences[0];
        next_occurences[6] += occurences[0];
        occurences = next_occurences;
    }
    let sum : u32 = occurences.iter().sum();
    println!("{}", sum);
}
