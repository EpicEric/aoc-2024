use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

static INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(isize, isize);

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn part1() {
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();
    let mut width = 0usize;
    let mut height = 0usize;
    INPUT.lines().enumerate().for_each(|(j, line)| {
        if j >= height {
            height = j + 1;
        }
        line.char_indices().for_each(|(i, char)| {
            if i >= width {
                width = i + 1;
            }
            if char != '.' {
                nodes
                    .entry(char)
                    .or_default()
                    .push(Position(i as isize, j as isize));
            }
        })
    });
    for antenna_set in nodes.values() {
        for a1 in 0..(antenna_set.len() - 1) {
            let antenna_1 = antenna_set[a1];
            for a2 in (a1 + 1)..antenna_set.len() {
                let antenna_2 = antenna_set[a2];
                let diff = antenna_1 - antenna_2;
                let antinode = antenna_1 + diff;
                if antinode.0 >= 0
                    && antinode.0 < width as isize
                    && antinode.1 >= 0
                    && antinode.1 < height as isize
                {
                    antinodes.insert(antinode);
                }
                let antinode = antenna_2 - diff;
                if antinode.0 >= 0
                    && antinode.0 < width as isize
                    && antinode.1 >= 0
                    && antinode.1 < height as isize
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    println!(
        "There are {} unique locations with an antinode.",
        antinodes.len()
    );
}

fn part2() {
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();
    let mut width = 0usize;
    let mut height = 0usize;
    INPUT.lines().enumerate().for_each(|(j, line)| {
        if j >= height {
            height = j + 1;
        }
        line.char_indices().for_each(|(i, char)| {
            if i >= width {
                width = i + 1;
            }
            if char != '.' {
                nodes
                    .entry(char)
                    .or_default()
                    .push(Position(i as isize, j as isize));
            }
        })
    });
    for antenna_set in nodes.values() {
        for a1 in 0..(antenna_set.len() - 1) {
            let antenna_1 = antenna_set[a1];
            for a2 in (a1 + 1)..antenna_set.len() {
                let antenna_2 = antenna_set[a2];
                let diff = antenna_1 - antenna_2;
                // let gcd = num::integer::gcd(diff.0, diff.1);
                // let diff = Position(diff.0 / gcd, diff.0 / gcd);
                let mut antinode = antenna_1;
                while antinode.0 >= 0
                    && antinode.0 < width as isize
                    && antinode.1 >= 0
                    && antinode.1 < height as isize
                {
                    antinodes.insert(antinode);
                    antinode = antinode + diff;
                }
                let mut antinode = antenna_2;
                while antinode.0 >= 0
                    && antinode.0 < width as isize
                    && antinode.1 >= 0
                    && antinode.1 < height as isize
                {
                    antinodes.insert(antinode);
                    antinode = antinode - diff;
                }
                // let mut antinode = antenna_1 - diff;
                // while antinode != antenna_2 {
                //     antinodes.insert(antinode);
                //     antinode = antinode - diff;
                // }
            }
        }
    }
    println!(
        "There are {} resonant locations with an antinode.",
        antinodes.len()
    );
}

fn main() {
    part1();
    part2();
}
