fn main() {
    let input = include_str!("day8_input");
    let lines = input.lines();
    let mut nb_unique = 0;
    let unique_sizes = [2, 3, 4, 7];

    for line in lines {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let patterns = parts[0];
        let output = parts[1];
        let digits = output.split_whitespace().collect::<Vec<&str>>();
        for digit in digits {
            if unique_sizes.contains(&digit.len()) {
                nb_unique += 1;
            }
        }
    }

    println!("Unique number of segments: {}", nb_unique);
}
