use num::integer::lcm;
use std::cmp::Ordering;
use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}
impl Vec3 {
    fn add(&mut self, vel: Vec3) {
        self.x += vel.x;
        self.y += vel.y;
        self.z += vel.z;
    }

    fn sum_abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn axis_by_number(&self, axis: i64) -> i64 {
        match axis {
            1 => self.x,
            2 => self.y,
            3 => self.z,
            _ => panic!("unknown axis"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Body {
    pos: Vec3,
    vel: Vec3,
}

impl Body {
    fn from_string(input: &str) -> Body {
        let reg = Regex::new(r"<x=([-\d]+),\sy=([-\d]+),\sz=([-\d]+)>").unwrap();
        let cap = reg.captures(input).unwrap();
        Body::new(
            cap[1].parse::<i64>().expect("failed to parse X"),
            cap[2].parse::<i64>().expect("failed to parse Y"),
            cap[3].parse::<i64>().expect("failed to parse Z"),
        )
    }

    fn new(x: i64, y: i64, z: i64) -> Body {
        Body {
            pos: Vec3 { x, y, z },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    fn get_energy(&self) -> i64 {
        self.pos.sum_abs() * self.vel.sum_abs()
    }
}

fn get_period(bodies: &Vec<Body>, axis: i64) -> i64 {
    let mut i = 0;
    let mut found = 0;
    let all_zero = |b: &Vec<Body>| b.iter().all(|x| x.vel.axis_by_number(axis) == 0);

    let mut b = bodies.clone();
    while found != 2 {
        i += 1;
        step(&mut b);
        found += all_zero(&b) as i64;
    }
    return i;
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    part1(&input);

    let bodies: Vec<Body> = input.lines().map(Body::from_string).collect();

    let period_x = get_period(&bodies, 1);
    let period_y = get_period(&bodies, 2);
    let period_z = get_period(&bodies, 3);

    println!(
        "Solution Part 2: {:?}",
        lcm(period_x, lcm(period_y, period_z))
    );
}

fn part1(input: &str) {
    let mut bodies: Vec<Body> = input.lines().map(Body::from_string).collect();

    for _ in 0..1000 {
        step(&mut bodies)
    }

    let result: i64 = bodies.iter().map(Body::get_energy).sum();

    println!("Solution Part 1: {}", result);
}

fn step(bodies: &mut Vec<Body>) {
    let bods = &mut bodies.clone();
    for b in bodies.iter_mut() {
        apply_gravity(b, bods);
        b.pos.add(b.vel);
    }
}

fn apply_gravity(body: &mut Body, bodies: &mut Vec<Body>) {
    let o_to_i64 = |o| match o {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };
    bodies.iter().for_each(|b| {
        body.vel.add(Vec3 {
            x: o_to_i64(body.pos.x.cmp(&b.pos.x)),
            y: o_to_i64(body.pos.y.cmp(&b.pos.y)),
            z: o_to_i64(body.pos.z.cmp(&b.pos.z)),
        })
    })
}
