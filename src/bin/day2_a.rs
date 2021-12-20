fn main() {
    let input = include_str!("day2_input");
    let commands = input.lines();

    let mut dep = 0;
    let mut hor = 0;

    for command in commands {
        let words : Vec<&str> = command.split_whitespace().collect();
        let action = words[0];
        let x : i32 = words[1].parse().unwrap();

        match action {
            "forward"   => hor += x,
            "up"        => dep -= x,
            _           => dep += x,
        }
    }
    println!("dep * hor = {} * {} = {}", dep, hor, dep * hor);
}
