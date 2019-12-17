use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum TileType {
    None,
    Asteroid,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

type Board = Vec<Vec<TileType>>;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");

    let asteroid_points = parse_input(&input);
    let los_asteroids = asteroid_points
        .iter()
        .map(|a| (a, count_los(&a, &asteroid_points)))
        .collect::<Vec<(&Point, usize)>>();
    println!(
        "Solution Part 1 {:?}",
        los_asteroids.iter().max_by(|a, b| a.1.cmp(&b.1))
    );
}

fn parse_input(input: &str) -> Vec<Point> {
    let parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => TileType::None,
                    '#' => TileType::Asteroid,
                    _ => panic!("Unknown tiletype {}", c),
                })
                .collect()
        })
        .collect::<Board>();

    let mut asteroid_points = Vec::new();

    for y in 0..parsed.len() {
        for x in 0..parsed[0].len() {
            if parsed[y][x] == TileType::Asteroid {
                asteroid_points.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    asteroid_points
}

fn count_los(origin: &Point, asteroids: &Vec<Point>) -> usize {
    asteroids
        .iter()
        .map(|dest| {
            asteroids
                .iter()
                .filter(|a| is_blocking(origin, dest, a))
                .count()
        })
        .filter(|c| *c <= 2)
        .count()
        - 1
}

fn dist(a: &Point, b: &Point) -> f32 {
    return (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32).sqrt();
}

fn is_blocking(src: &Point, dest: &Point, point: &Point) -> bool {
    let dist_a = dist(src, point) + dist(point, dest);
    let dist_b = dist(src, dest);
    return (dist_a - dist_b).abs() < 0.00001;
}
