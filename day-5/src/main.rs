use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

static INPUT: &str = include_str!("./input.txt");

type Rulebook = HashMap<usize, HashSet<usize>>;

struct Page<'a> {
    value: usize,
    rulebook: &'a Rulebook,
}

impl PartialEq for Page<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.rulebook
            .get(&self.value)
            .is_none_or(|rules| !rules.contains(&other.value))
            && self
                .rulebook
                .get(&other.value)
                .is_none_or(|rules| !rules.contains(&self.value))
    }
}

impl Eq for Page<'_> {}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self
            .rulebook
            .get(&self.value)
            .is_some_and(|rules| rules.contains(&other.value))
        {
            return Ordering::Less;
        }
        if self
            .rulebook
            .get(&other.value)
            .is_some_and(|rules| rules.contains(&self.value))
        {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1() {
    let mut rulebook: Rulebook = HashMap::new();
    let mut updates: Vec<Vec<Page>> = Vec::new();
    let mut iter_lines = INPUT.lines();
    for line in &mut iter_lines {
        if line.is_empty() {
            break;
        } else {
            let (left, right) = line.split_once('|').unwrap();
            rulebook
                .entry(left.parse().unwrap())
                .or_default()
                .insert(right.parse().unwrap());
        }
    }
    for line in &mut iter_lines {
        if line.is_empty() {
            break;
        } else {
            updates.push(
                line.split(',')
                    .map(|page| Page {
                        value: page.parse().unwrap(),
                        rulebook: &rulebook,
                    })
                    .collect(),
            );
        }
    }
    let sum: usize = updates
        .into_iter()
        .filter_map(|update| {
            if update.is_sorted() {
                Some(update.get(update.len() / 2).unwrap().value)
            } else {
                None
            }
        })
        .sum();
    println!(
        "The sum of middle pages for correctly-ordered updates is {}",
        sum
    );
}

fn part2() {
    let mut rulebook: Rulebook = HashMap::new();
    let mut updates: Vec<Vec<Page>> = Vec::new();
    let mut iter_lines = INPUT.lines();
    for line in &mut iter_lines {
        if line.is_empty() {
            break;
        } else {
            let (left, right) = line.split_once('|').unwrap();
            rulebook
                .entry(left.parse().unwrap())
                .or_default()
                .insert(right.parse().unwrap());
        }
    }
    for line in &mut iter_lines {
        if line.is_empty() {
            break;
        } else {
            updates.push(
                line.split(',')
                    .map(|page| Page {
                        value: page.parse().unwrap(),
                        rulebook: &rulebook,
                    })
                    .collect(),
            );
        }
    }
    let sum: usize = updates
        .into_iter()
        .filter(|update| !update.is_sorted())
        .map(|mut update| {
            update.sort();
            update.get(update.len() / 2).unwrap().value
        })
        .sum();
    println!(
        "The sum of middle pages only for the fixed updates is {}",
        sum
    );
}

fn main() {
    part1();
    part2();
}
