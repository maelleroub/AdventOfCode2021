struct Point {
    x : usize,
    y : usize
}

impl Point {
    fn from_coords(coords : &str) -> Point {
        let pair_coords = coords.split(",").collect::<Vec<&str>>();
        let x : usize = pair_coords[0].parse().unwrap();
        let y : usize = pair_coords[1].parse().unwrap();
        Point { x : x, y : y }
    }

    fn get_zone(a : Point, b : Point) -> Vec<Point> {
        let mut res : Vec<Point> = Vec::new();
        if a.x == b.x {
            let start = usize::min(a.y, b.y);
            let end = usize::max(a.y, b.y);
            for y in start..=end {
                res.push(Point { x : a.x, y : y });
            }
        }
        else {
            let start = usize::min(a.x, b.x);
            let end = usize::max(a.x, b.x);
            for x in start..=end {
                res.push(Point { x : x, y : a.y });
            }
        }
        res
    }
}

fn main() {
    let input = include_str!("day5_input");
    let lines = input.lines();
    let mut diagram = [[0i32; 1000]; 1000];

    for line in lines {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        let start = Point::from_coords(points[0]);
        let end = Point::from_coords(points[1]);
        // Ignore diagonals
        if (start.x != end.x) && (start.y != end.y) {
            continue;
        }

        let zone = Point::get_zone(start, end);
        for p in zone {
            diagram[p.x as usize][p.y as usize] += 1;
        }
    }

    let mut res : i32 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if diagram[i][j] >= 2 {
                res += 1;
            }
        }
    }
    println!("{}", res);        
}
