use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

static INPUT: &str = include_str!("./input.txt");

fn part1() {
    let mut stones: Vec<usize> = INPUT
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len() * 3 / 2);
        for stone in stones.into_iter() {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_string = stone.to_string();
                if stone_string.len() % 2 == 0 {
                    let (stone_1, stone_2) = stone_string.split_at(stone_string.len() / 2);
                    new_stones.push(stone_1.parse().unwrap());
                    new_stones.push(stone_2.parse().unwrap());
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones;
    }
    println!("After blinking 25 times, we have {} stones", stones.len());
}

fn part2() {
    let stones: Vec<usize> = INPUT
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    let mut stone_map: HashMap<usize, usize> = HashMap::new();
    for stone in stones.into_iter() {
        *stone_map.entry(stone).or_default() += 1;
    }
    for _ in 0..75 {
        let mut new_stone_map: HashMap<usize, usize> = HashMap::new();
        let stones: Vec<(usize, usize)> = stone_map
            .into_par_iter()
            .flat_map_iter(|(stone, count)| {
                if stone == 0 {
                    vec![(1, count)]
                } else {
                    let stone_string = stone.to_string();
                    if stone_string.len() % 2 == 0 {
                        let (stone_1, stone_2) = stone_string.split_at(stone_string.len() / 2);
                        vec![
                            (stone_1.parse().unwrap(), count),
                            (stone_2.parse().unwrap(), count),
                        ]
                    } else {
                        vec![(stone * 2024, count)]
                    }
                }
            })
            .collect();
        for (stone, count) in stones.into_iter() {
            *new_stone_map.entry(stone).or_default() += count;
        }
        stone_map = new_stone_map;
    }
    println!(
        "After blinking 25 times, we have {} stones",
        stone_map.values().sum::<usize>()
    );
}

fn main() {
    part1();
    part2();
}
