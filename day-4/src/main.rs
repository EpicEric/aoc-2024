use itertools::izip;

static INPUT: &str = include_str!("./input.txt");

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_REVERSED: [char; 4] = ['S', 'A', 'M', 'X'];

fn part1() {
    let data = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let horizontal: usize = data
        .iter()
        .map(|line| {
            line.as_slice()
                .windows(4)
                .filter(|&window| window == XMAS || window == XMAS_REVERSED)
                .count()
        })
        .sum();
    let vertical: usize = data
        .as_slice()
        .windows(4)
        .filter_map(|lines| {
            let [a, b, c, d] = lines else {
                return None;
            };
            Some(
                izip!(a, b, c, d)
                    .filter(|&x| {
                        [*x.0, *x.1, *x.2, *x.3] == XMAS
                            || [*x.0, *x.1, *x.2, *x.3] == XMAS_REVERSED
                    })
                    .count(),
            )
        })
        .sum();
    let main_diagonal: usize = data
        .as_slice()
        .windows(4)
        .filter_map(|lines| {
            let [a, b, c, d] = lines else {
                return None;
            };
            Some(
                izip!(a, b.iter().skip(1), c.iter().skip(2), d.iter().skip(3))
                    .filter(|&x| {
                        [*x.0, *x.1, *x.2, *x.3] == XMAS
                            || [*x.0, *x.1, *x.2, *x.3] == XMAS_REVERSED
                    })
                    .count(),
            )
        })
        .sum();
    let opposite_diagonal: usize = data
        .as_slice()
        .windows(4)
        .filter_map(|lines| {
            let [a, b, c, d] = lines else {
                return None;
            };
            Some(
                izip!(a.iter().skip(3), b.iter().skip(2), c.iter().skip(1), d)
                    .filter(|&x| {
                        [*x.0, *x.1, *x.2, *x.3] == XMAS
                            || [*x.0, *x.1, *x.2, *x.3] == XMAS_REVERSED
                    })
                    .count(),
            )
        })
        .sum();
    let total = horizontal + vertical + main_diagonal + opposite_diagonal;
    println!("The total count of XMAS is: {total}");
}

fn part2() {
    let data = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let total: usize = data
        .as_slice()
        .windows(3)
        .filter_map(|lines| {
            let [a, b, c] = lines else {
                return None;
            };
            Some(
                izip!(a.windows(3), b.windows(3), c.windows(3))
                    .filter(|(x, y, z)| match (x, y, z) {
                        (['M', _, 'M'], [_, 'A', _], ['S', _, 'S']) => true,
                        (['M', _, 'S'], [_, 'A', _], ['M', _, 'S']) => true,
                        (['S', _, 'S'], [_, 'A', _], ['M', _, 'M']) => true,
                        (['S', _, 'M'], [_, 'A', _], ['S', _, 'M']) => true,
                        _ => false,
                    })
                    .count(),
            )
        })
        .sum();
    println!("The total count of X-MAS is: {total}");
}

fn main() {
    part1();
    part2();
}
