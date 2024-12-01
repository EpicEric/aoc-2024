use std::collections::HashMap;

static INPUT: &str = include_str!("./input.txt");

fn part_1() {
    let mut list_1: Vec<usize> = Vec::new();
    let mut list_2: Vec<usize> = Vec::new();
    INPUT
        .lines()
        .filter_map(|line| line.split_once("   "))
        .for_each(|(first, second)| {
            list_1.push(first.parse().unwrap());
            list_2.push(second.parse().unwrap());
        });
    list_1.sort();
    list_2.sort();
    let result: usize = list_1
        .into_iter()
        .zip(list_2.into_iter())
        .map(|(first, second)| first.abs_diff(second))
        .sum();
    println!("The total distance is {result}");
}

fn part_2() {
    let mut list_1: Vec<usize> = Vec::new();
    let mut list_2: HashMap<usize, usize> = HashMap::new();
    INPUT
        .lines()
        .filter_map(|line| line.split_once("   "))
        .for_each(|(first, second)| {
            list_1.push(first.parse().unwrap());
            *list_2.entry(second.parse().unwrap()).or_default() += 1;
        });
    let result: usize = list_1
        .into_iter()
        .map(|number| number * list_2.get(&number).unwrap_or(&0))
        .sum();
    println!("The similarity score is {result}");
}

fn main() {
    part_1();
    part_2();
}
