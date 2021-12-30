use std::collections::HashMap;

fn main() {
    let input = include_str!("day10_input");
    let mut res = 0;
    for line in input.lines() {
        let mut chunks : Vec<char> = Vec::new();
        let associations = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>'),
        ]);
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => chunks.push(c),
                _ => {
                    let last_chunk = chunks.last().unwrap();
                    if associations[last_chunk] != c {
                        match c {
                            ')' => res += 3,
                            ']' => res += 57,
                            '}' => res += 1197,
                            _ => res += 25137
                        }
                        break;
                    }
                    else {
                        chunks.pop();
                    }
                }
            }
        }
    }
    println!("{}", res);
}
