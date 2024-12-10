use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("./input.txt");

fn find_neighbors(map: &Vec<Vec<u8>>, position: (usize, usize), value: u8) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    // Up
    if position.1 > 0
        && map
            .get(position.1 - 1)
            .and_then(|line| line.get(position.0))
            == Some(&value)
    {
        neighbors.push((position.0, position.1 - 1));
    }
    // Left
    if position.0 > 0
        && map
            .get(position.1)
            .and_then(|line| line.get(position.0 - 1))
            == Some(&value)
    {
        neighbors.push((position.0 - 1, position.1));
    }
    // Down
    if map
        .get(position.1 + 1)
        .and_then(|line| line.get(position.0))
        == Some(&value)
    {
        neighbors.push((position.0, position.1 + 1));
    }
    // Right
    if map
        .get(position.1)
        .and_then(|line| line.get(position.0 + 1))
        == Some(&value)
    {
        neighbors.push((position.0 + 1, position.1));
    }
    neighbors
}

fn part1() {
    let map: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let trailheads = map
        .iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == 0)
                .map(move |(i, _)| (i, j))
        })
        .flatten();
    let mut total_score = 0usize;
    for trailhead in trailheads {
        let mut height = 0;
        let mut trails = HashSet::new();
        trails.insert(trailhead);
        while height < 9 {
            trails = trails
                .into_iter()
                .map(|tile| find_neighbors(&map, tile, height + 1))
                .flatten()
                .collect();
            height += 1;
        }
        total_score += trails.len();
    }
    println!("The total score of all trailheads is {}", total_score);
}

fn part2() {
    let map: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let trailheads = map
        .iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == 0)
                .map(move |(i, _)| (i, j))
        })
        .flatten();
    let mut total_rating = 0usize;
    for trailhead in trailheads {
        let mut height = 0;
        let mut trails: HashMap<(usize, usize), usize> = HashMap::new();
        trails.insert(trailhead, 1);
        while height < 9 {
            let mut new_trails = HashMap::new();
            for (tile, rating) in trails.into_iter() {
                let neighbors = find_neighbors(&map, tile, height + 1);
                for neighbor in neighbors {
                    *new_trails.entry(neighbor).or_default() += rating;
                }
            }
            trails = new_trails;
            height += 1;
        }
        total_rating += trails.values().sum::<usize>();
    }
    println!("The total rating of all trailheads is {}", total_rating);
}

fn main() {
    part1();
    part2();
}
