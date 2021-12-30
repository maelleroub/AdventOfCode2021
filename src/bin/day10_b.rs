use std::collections::HashMap;

fn main() {
    let input = include_str!("day10_input");
    let mut res : Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut score = 0;
        let mut chunks : Vec<char> = Vec::new();
        let associations = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>'),
        ]);
        let mut incomplete = true;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => chunks.push(c),
                _ => {
                    let last_chunk = chunks.last().unwrap();
                    if associations[last_chunk] != c {
                        incomplete = false;
                        break;
                    }
                    else {
                        chunks.pop();
                    }
                }
            }
        }
        if ! incomplete {
            continue;
        }
        while chunks.len() != 0 {
            let chunk = chunks.pop().unwrap();
            let close = associations[&chunk];
            score *= 5;
            match close {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                _ => score += 4,
            }
        }
        res.push(score);
    }
    res.sort();
    println!("{:?}", res);
    println!("{}", res[res.len() / 2]);
}
