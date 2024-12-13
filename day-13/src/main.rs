use regex::Regex;

static INPUT: &str = include_str!("./input.txt");

fn part1() {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut tokens = 0usize;
    for c in re.captures_iter(INPUT) {
        let button_a = (
            c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        );
        let button_b = (
            c.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            c.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        );
        let prize = (
            c.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            c.get(6).unwrap().as_str().parse::<usize>().unwrap(),
        );
        let mut a_presses = 0usize;
        let mut b_presses = (prize.0 / button_b.0).max(prize.1 / button_b.1).min(99) + 1;
        let mut position = (b_presses * button_b.0, b_presses * button_b.1);
        'a: loop {
            while position.0 > prize.0 || position.1 > prize.1 {
                if b_presses == 0 {
                    break 'a;
                }
                b_presses -= 1;
                position.0 -= button_b.0;
                position.1 -= button_b.1;
            }
            while position.0 < prize.0 || position.1 < prize.1 {
                a_presses += 1;
                position.0 += button_a.0;
                position.1 += button_a.1;
                if a_presses > 100 {
                    break;
                }
            }
            if position == prize {
                tokens += 3 * a_presses + b_presses;
                break;
            }
        }
    }
    println!("The fewest tokens to win all prizes is {}", tokens);
}

fn part2() {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut tokens = 0usize;
    for c in re.captures_iter(INPUT) {
        let button_a = (
            c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let button_b = (
            c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let prize = (
            c.get(5).unwrap().as_str().parse::<isize>().unwrap() + 10000000000000,
            c.get(6).unwrap().as_str().parse::<isize>().unwrap() + 10000000000000,
        );
        let Some(b_presses) = (prize.0 * button_a.1 - prize.1 * button_a.0)
            .checked_div(button_b.0 * button_a.1 - button_a.0 * button_b.1)
        else {
            continue;
        };
        let a_presses = prize.0.saturating_sub(b_presses * button_b.0) / button_a.0;
        let position = (
            a_presses * button_a.0 + b_presses * button_b.0,
            a_presses * button_a.1 + b_presses * button_b.1,
        );
        if position == prize {
            tokens += 3 * a_presses as usize + b_presses as usize;
        }
    }
    println!(
        "The fewest tokens to win all corrected prizes is {}",
        tokens
    );
}

fn main() {
    part1();
    part2();
}
