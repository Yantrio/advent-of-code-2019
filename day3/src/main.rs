use std::fs::read_to_string;
use std::str;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    dist_to_start: i64,
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");

    run(input);
}

fn run(input: String) {
    let wireset1 = parse_wires(input.lines().nth(0).unwrap());
    let wireset2 = parse_wires(input.lines().nth(1).unwrap());

    let all_intersections: Vec<Point> = wireset1
        .iter()
        .flat_map(|w| {
            wireset2
                .iter()
                .filter_map(move |w2| get_intersection_point(&w.start, &w.end, &w2.start, &w2.end))
        })
        .collect();

    println!(
        "Part 1 Solution: {:?}",
        all_intersections
            .iter()
            .map(|i| manhattan_distance(&Point { x: 0, y: 0 }, &i))
            .min()
            .unwrap()
    );

    let min = &all_intersections
        .iter()
        .map(|p| {
            //get the distance to p
            let w1 = wireset1
                .iter()
                .find(|w| is_on_line(&p, &w.start, &w.end))
                .unwrap();

            let w2 = wireset2
                .iter()
                .find(|w| is_on_line(&p, &w.start, &w.end))
                .unwrap();

            let mhd1 = manhattan_distance(&w1.start, &p);
            let mhd2 = manhattan_distance(&w2.start, &p);

            mhd1 + mhd2 + w1.dist_to_start + w2.dist_to_start
        })
        .min();

    println!("Part 2 Solution: {:?}", min.unwrap());
}

fn get_intersection_point(a: &Point, b: &Point, c: &Point, d: &Point) -> Option<Point> {
    let a1 = b.y - a.y;
    let b1 = a.x - b.x;
    let c1 = a1 * (a.x) + b1 * (a.y);

    let a2 = d.y - c.y;
    let b2 = c.x - d.x;
    let c2 = a2 * (c.x) + b2 * (c.y);

    let det = (a1 * b2) - (a2 * b1);

    match det {
        0 => None,
        _ => {
            let p = Point {
                x: (b2 * c1 - b1 * c2) / det,
                y: (a1 * c2 - a2 * c1) / det,
            };
            match is_on_line(&p, a, b) && is_on_line(&p, c, d) && (p.x != 0 && p.y != 0) {
                true => Some(p),
                false => None,
            }
        }
    }
}

fn is_on_line(point: &Point, line_start: &Point, line_end: &Point) -> bool {
    let on_x = (point.x >= line_start.x && point.x <= line_end.x)
        || (point.x >= line_end.x && point.x <= line_start.x);
    let on_y = (point.y >= line_start.y && point.y <= line_end.y)
        || (point.y >= line_end.y && point.y <= line_start.y);
    on_x && on_y
}

fn manhattan_distance(a: &Point, b: &Point) -> i64 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn parse_wires(inputline: &str) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;
    let mut dist_so_far = 0;
    let directions = inputline
        .split(",")
        .map(|d| {
            let start = Point {
                x: x.clone(),
                y: y.clone(),
            };
            let dist: i64 = (&d[1..]).parse().unwrap();

            let dir = d.chars().nth(0).unwrap();
            match dir {
                'U' => y = y - dist,
                'D' => y = y + dist,
                'L' => x = x - dist,
                'R' => x = x + dist,
                _ => panic!(),
            }
            let result = Line {
                start,
                end: Point { x, y },
                dist_to_start: dist_so_far,
            };
            dist_so_far = dist_so_far + dist;
            result
        })
        .collect::<Vec<Line>>();

    return directions;
}
