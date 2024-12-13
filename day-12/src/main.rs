use std::collections::BTreeSet;

static INPUT: &str = include_str!("./input.txt");

fn get_matching_neighbors(
    plant: char,
    (i, j): (usize, usize),
    farm: &[Vec<char>],
) -> Vec<(usize, usize)> {
    [
        // Left
        if i > 0 {
            farm.get(j)
                .and_then(|line| line.get(i - 1))
                .map(|tile| (i - 1, j, tile))
        } else {
            None
        },
        // Up
        if j > 0 {
            farm.get(j - 1)
                .and_then(|line| line.get(i))
                .map(|tile| (i, j - 1, tile))
        } else {
            None
        },
        // Right
        farm.get(j)
            .and_then(|line| line.get(i + 1))
            .map(|tile| (i + 1, j, tile)),
        // Down
        farm.get(j + 1)
            .and_then(|line| line.get(i))
            .map(|tile| (i, j + 1, tile)),
    ]
    .into_iter()
    .filter_map(|direction| {
        direction.and_then(|(i, j, tile)| if *tile == plant { Some((i, j)) } else { None })
    })
    .collect()
}

fn get_perimeter(plant: char, pos: (usize, usize), farm: &[Vec<char>]) -> usize {
    4 - get_matching_neighbors(plant, pos, farm).len()
}

fn part1() {
    let farm: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut unchecked: BTreeSet<(usize, usize)> = (0..farm.len())
        .flat_map(|j| (0..farm[0].len()).map(move |i| (i, j)))
        .collect();
    let mut cost = 0usize;
    while let Some((i, j)) = unchecked.pop_first() {
        let plant = farm[j][i];
        let mut area = 1usize;
        let mut perimeter = get_perimeter(plant, (i, j), &farm);
        let mut search = vec![(i, j)];
        while !search.is_empty() {
            search = search
                .into_iter()
                .flat_map(|pos| get_matching_neighbors(plant, pos, &farm))
                .filter(|pos| unchecked.take(pos).is_some())
                .collect();
            for pos in search.iter() {
                area += 1;
                perimeter += get_perimeter(plant, *pos, &farm);
            }
        }
        cost += area * perimeter;
    }
    println!("The total cost for fencing is {}", cost);
}

fn part2() {
    let farm: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut unchecked: BTreeSet<(usize, usize)> = (0..farm.len())
        .flat_map(|j| (0..farm[0].len()).map(move |i| (i, j)))
        .collect();
    let mut cost = 0usize;
    while let Some((i, j)) = unchecked.pop_first() {
        let plant = farm[j][i];
        let mut area = 1usize;
        let mut sides = 0usize;
        let mut search = vec![(i, j)];
        let mut shape = BTreeSet::new();
        shape.insert((i, j));
        // Bounding box
        let mut top_left = (i, j);
        let mut bottom_right = (i, j);
        while !search.is_empty() {
            search = search
                .into_iter()
                .flat_map(|pos| get_matching_neighbors(plant, pos, &farm))
                .filter(|pos| unchecked.take(pos).is_some())
                .collect();
            for pos in search.iter() {
                area += 1;
                shape.insert(*pos);
                if pos.0 < top_left.0 {
                    top_left.0 = pos.0;
                } else if pos.0 > bottom_right.0 {
                    bottom_right.0 = pos.0;
                }
                if pos.1 < top_left.1 {
                    top_left.1 = pos.1;
                } else if pos.1 > bottom_right.1 {
                    bottom_right.1 = pos.1;
                }
            }
        }
        // Scan contiguous horizontal lines
        for j in top_left.1..=bottom_right.1 {
            let mut is_side_top = false;
            let mut is_side_bottom = false;
            for i in top_left.0..=bottom_right.0 {
                if shape.contains(&(i, j)) {
                    if !is_side_top && (j == 0 || !shape.contains(&(i, j - 1))) {
                        is_side_top = true;
                        sides += 1;
                    }
                    if is_side_top && j > 0 && shape.contains(&(i, j - 1)) {
                        is_side_top = false;
                    }
                    if !is_side_bottom && !shape.contains(&(i, j + 1)) {
                        is_side_bottom = true;
                        sides += 1;
                    }
                    if is_side_bottom && shape.contains(&(i, j + 1)) {
                        is_side_bottom = false;
                    }
                } else {
                    is_side_top = false;
                    is_side_bottom = false;
                }
            }
        }
        // Scan contiguous vertical lines
        for i in top_left.0..=bottom_right.0 {
            let mut is_side_left = false;
            let mut is_side_right = false;
            for j in top_left.1..=bottom_right.1 {
                if shape.contains(&(i, j)) {
                    if !is_side_left && (i == 0 || !shape.contains(&(i - 1, j))) {
                        is_side_left = true;
                        sides += 1;
                    }
                    if is_side_left && i > 0 && shape.contains(&(i - 1, j)) {
                        is_side_left = false;
                    }
                    if !is_side_right && !shape.contains(&(i + 1, j)) {
                        is_side_right = true;
                        sides += 1;
                    }
                    if is_side_right && shape.contains(&(i + 1, j)) {
                        is_side_right = false;
                    }
                } else {
                    is_side_left = false;
                    is_side_right = false;
                }
            }
        }
        cost += area * sides;
    }
    println!("The new total cost for fencing is {}", cost);
}

fn main() {
    part1();
    part2();
}
