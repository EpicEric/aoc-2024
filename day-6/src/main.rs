use std::collections::HashSet;

static INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Space {
    Empty,
    Obstacle,
    Path,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn is_leaving(&self, position: &(usize, usize), map: &Vec<Vec<Space>>) -> bool {
        match self {
            Direction::North => position.1 == 0,
            Direction::East => position.0 == map[0].len() - 1,
            Direction::South => position.1 == map.len() - 1,
            Direction::West => position.0 == 0,
        }
    }

    fn peek_next(&self, position: &(usize, usize), map: &Vec<Vec<Space>>) -> Option<Space> {
        match self {
            Direction::North => map
                .get(position.1 - 1)
                .and_then(|line| line.get(position.0).copied()),
            Direction::East => map
                .get(position.1)
                .and_then(|line| line.get(position.0 + 1).copied()),
            Direction::South => map
                .get(position.1 + 1)
                .and_then(|line| line.get(position.0).copied()),
            Direction::West => map
                .get(position.1)
                .and_then(|line| line.get(position.0 - 1).copied()),
        }
    }

    fn get_next_position(
        &self,
        position: &(usize, usize),
        map: &Vec<Vec<Space>>,
    ) -> Option<(usize, usize)> {
        if self.is_leaving(position, map) {
            return None;
        }
        match self {
            Direction::North => Some((position.0, position.1 - 1)),
            Direction::East => Some((position.0 + 1, position.1)),
            Direction::South => Some((position.0, position.1 + 1)),
            Direction::West => Some((position.0 - 1, position.1)),
        }
    }

    fn move_to(&self, position: &mut (usize, usize), map: &mut Vec<Vec<Space>>) {
        *position = self.get_next_position(&position, &map).unwrap();
        map[position.1][position.0] = Space::Path;
    }

    fn step(&self, position: &mut (usize, usize), map: &mut Vec<Vec<Space>>) -> Option<Self> {
        if self.is_leaving(position, map) {
            None
        } else if self.peek_next(position, map).unwrap() == Space::Obstacle {
            Some(self.turn_right())
        } else {
            self.move_to(position, map);
            Some(*self)
        }
    }
}

fn part1() {
    let mut position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<Space>> = INPUT
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.char_indices()
                .map(|(i, tile)| match tile {
                    '.' => Space::Empty,
                    '#' => Space::Obstacle,
                    '^' => {
                        position = (i, j);
                        Space::Path
                    }
                    other => panic!("Unexpected tile {other}"),
                })
                .collect()
        })
        .collect();
    let mut direction = Direction::North;
    loop {
        match direction.step(&mut position, &mut map) {
            Some(dir) => direction = dir,
            None => break,
        }
    }
    let positions = map
        .into_iter()
        .flat_map(|line| line.into_iter())
        .filter(|&tile| tile == Space::Path)
        .count();
    println!("The guard visits {} positions.", positions);
}

fn part2() {
    let mut position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<Space>> = INPUT
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.char_indices()
                .map(|(i, tile)| match tile {
                    '.' => Space::Empty,
                    '#' => Space::Obstacle,
                    '^' => {
                        position = (i, j);
                        Space::Path
                    }
                    other => panic!("Unexpected tile {other}"),
                })
                .collect()
        })
        .collect();
    let mut direction = Direction::North;
    let mut obstructions = 0;
    let mut corners: HashSet<((usize, usize), Direction)> = HashSet::new();
    loop {
        // Add one obstruction in front of the guard and check for loops
        if let Some(next_position) = direction.get_next_position(&position, &map) {
            if map[next_position.1][next_position.0] == Space::Empty {
                let mut map_2: Vec<Vec<Space>> = map
                    .iter()
                    .map(|line| line.iter().cloned().collect())
                    .collect();
                let mut position_2 = position;
                let mut direction_2 = direction;
                let mut corners_2 = corners.clone();
                map_2[next_position.1][next_position.0] = Space::Obstacle;
                loop {
                    match direction_2.step(&mut position_2, &mut map_2) {
                        Some(dir) if dir != direction_2 => {
                            if corners_2.contains(&(position_2, direction_2)) {
                                obstructions += 1;
                                break;
                            } else {
                                corners_2.insert((position_2, direction_2));
                            }
                            direction_2 = dir;
                        }
                        Some(dir) => direction_2 = dir,
                        None => break,
                    }
                }
            }
        }
        // Advance as normal
        match direction.step(&mut position, &mut map) {
            Some(dir) => {
                if dir != direction {
                    corners.insert((position, direction));
                }
                direction = dir;
            }
            None => break,
        }
    }
    println!("There are {} possible obstructions.", obstructions);
}

fn main() {
    part1();
    part2();
}
