use std::cmp::Ordering;

use regex::Regex;

static INPUT: &str = include_str!("./input.txt");

fn part1() {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut top_left_quadrant = 0usize;
    let mut bottom_left_quadrant = 0usize;
    let mut top_right_quadrant = 0usize;
    let mut bottom_right_quadrant = 0usize;
    for (position, velocity) in INPUT
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|captures| {
            let position = (
                captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            );
            let velocity = (
                captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            );
            (position, velocity)
        })
    {
        let final_position = (
            ((position.0 + velocity.0 * 100).rem_euclid(101)),
            ((position.1 + velocity.1 * 100).rem_euclid(103)),
        );
        match (final_position.0.cmp(&50), final_position.1.cmp(&51)) {
            (Ordering::Less, Ordering::Less) => top_left_quadrant += 1,
            (Ordering::Less, Ordering::Greater) => bottom_left_quadrant += 1,
            (Ordering::Greater, Ordering::Less) => top_right_quadrant += 1,
            (Ordering::Greater, Ordering::Greater) => bottom_right_quadrant += 1,
            _ => (),
        }
    }
    println!(
        "The safety factor after 100 seconds is {}",
        top_left_quadrant * bottom_left_quadrant * top_right_quadrant * bottom_right_quadrant
    );
}

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn get_variance(robots: &[Robot]) -> f64 {
    let (total_x, total_y) = robots.iter().fold((0f64, 0f64), |acc, robot| {
        (
            acc.0 + (robot.position.0 as f64),
            acc.1 + (robot.position.1 as f64),
        )
    });
    let (avg_x, avg_y) = (total_x / robots.len() as f64, total_y / robots.len() as f64);
    let (var_x, var_y) = robots.iter().fold((0f64, 0f64), |acc, robot| {
        (
            acc.0 + ((robot.position.0 as f64) - avg_x).powi(2),
            acc.1 + ((robot.position.1 as f64) - avg_y).powi(2),
        )
    });
    var_x + var_y
}

fn part2() {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = INPUT
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|captures| {
            let position = (
                captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            );
            let velocity = (
                captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            );
            Robot { position, velocity }
        })
        .collect();
    let mut easter_egg_time = 0usize;
    let mut lowest_variance = f64::INFINITY;
    for time in 0..(101 * 103) {
        let variance = get_variance(&robots);
        if variance < lowest_variance {
            lowest_variance = variance;
            easter_egg_time = time;
        }
        for robot in robots.iter_mut() {
            robot.position = (
                ((robot.position.0 + robot.velocity.0).rem_euclid(101)),
                ((robot.position.1 + robot.velocity.1).rem_euclid(103)),
            );
        }
    }
    println!(
        "The robots display the Easter egg after {} seconds",
        easter_egg_time
    );
}

fn main() {
    part1();
    part2();
}
