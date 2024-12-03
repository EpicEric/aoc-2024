use regex::Regex;

static INPUT: &str = include_str!("./input.txt");

fn part1() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total: usize = 0;
    for c in re.captures_iter(INPUT) {
        total += c.get(1).unwrap().as_str().parse::<usize>().unwrap()
            * c.get(2).unwrap().as_str().parse::<usize>().unwrap()
    }
    println!("The result of all multiplications is {}", total);
}

fn part2() {
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let mut iter_do = re_do.find_iter(INPUT).map(|m| m.start()).peekable();
    let mut iter_dont = re_dont.find_iter(INPUT).map(|m| m.start()).peekable();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total: usize = 0;
    let mut prev_do: isize = -1;
    let mut prev_dont: isize = -1;
    for capture in re.captures_iter(INPUT) {
        let capture_start = capture.get(0).unwrap().start();
        while let Some(val_do) = iter_do.peek() {
            if *val_do < capture_start {
                prev_do = iter_do.next().unwrap() as isize;
            } else {
                break;
            }
        }
        while let Some(val_dont) = iter_dont.peek() {
            if *val_dont < capture_start {
                prev_dont = iter_dont.next().unwrap() as isize;
            } else {
                break;
            }
        }
        if prev_dont <= prev_do {
            total += capture.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<usize>().unwrap()
        }
    }
    println!("The result of all multiplications is {}", total);
}

fn main() {
    part1();
    part2();
}
