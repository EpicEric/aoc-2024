use std::iter;

static INPUT: &str = include_str!("./input.txt");

enum State {
    File(usize),
    Empty(usize),
}

fn part1() {
    // let mut memory: Vec<usize> = Vec::new();
    let mut checksum = 0usize;
    let mut memory_index = 0usize;
    let mut chars: Vec<u8> = INPUT
        .chars()
        .filter_map(|char| char.to_digit(10).map(|digit| digit as u8))
        .collect();
    if chars.len() % 2 == 0 {
        chars.pop();
    }

    let mut front_id = 0usize;
    let mut back_id = chars.len() / 2;
    let mut iter = chars.into_iter();
    let mut back_remaining = iter.next_back().unwrap();
    let mut state = State::File(iter.next().unwrap().into());
    'l: loop {
        state = match state {
            State::File(remaining) => {
                if remaining > 0 {
                    // memory.push(front_id);
                    checksum += front_id * memory_index;
                    memory_index += 1;
                    State::File(remaining - 1)
                } else {
                    let Some(empty) = iter.next() else {
                        break;
                    };
                    State::Empty(empty.into())
                }
            }
            State::Empty(remaining) => {
                while back_remaining == 0 {
                    let Some(_) = iter.next_back() else {
                        break 'l;
                    };
                    let Some(back) = iter.next_back() else {
                        break 'l;
                    };
                    back_remaining = back;
                    back_id -= 1;
                }
                if remaining > 0 {
                    back_remaining -= 1;
                    // memory.push(back_id);
                    checksum += back_id * memory_index;
                    memory_index += 1;
                    State::Empty(remaining - 1)
                } else {
                    front_id += 1;
                    let Some(file) = iter.next() else {
                        break;
                    };
                    State::File(file.into())
                }
            }
        };
    }
    while back_remaining > 0 {
        back_remaining -= 1;
        // memory.push(back_id);
        checksum += back_id * memory_index;
        memory_index += 1;
    }
    // println!("{:?}", &memory);
    // let checksum: usize = memory.iter().enumerate().map(|(a, b)| a * b).sum();
    println!("The checksum is {}", checksum);
}

#[derive(Debug)]
enum Node {
    File { id: usize, size: u32 },
    Span { size: u32 },
}

fn part2() {
    let mut memory: Vec<Node> = Vec::new();
    let mut chars: Vec<u32> = INPUT
        .chars()
        .filter_map(|char| char.to_digit(10).map(|digit| digit))
        .collect();
    if chars.len() % 2 == 0 {
        chars.pop();
    }

    let mut id = 0usize;
    let mut iter = chars.into_iter();
    let mut is_file = true;
    while let Some(size) = iter.next() {
        if is_file {
            if size > 0 {
                memory.push(Node::File { id, size });
            }
            id += 1;
            is_file = false;
        } else {
            if size > 0 {
                memory.push(Node::Span { size });
            }
            is_file = true;
        }
    }
    let mut i = memory.len() - 1;
    while i > 0 {
        if let Node::File { size, .. } = memory[i] {
            let mut j = 0;
            while j < i {
                if let Node::Span { size: span_size } = memory[j] {
                    if span_size >= size {
                        memory.swap(i, j);
                        if span_size > size {
                            memory.insert(
                                j + 1,
                                Node::Span {
                                    size: span_size - size,
                                },
                            );
                            i += 1;
                            let Some(Node::Span {
                                size: ref mut span_size,
                            }) = memory.get_mut(i)
                            else {
                                unreachable!()
                            };
                            *span_size = size;
                        }
                        // Resize spans
                        let mut decrease_i_by = 0usize;
                        if let Some(Node::Span { size: right_span }) = memory.get(i + 1) {
                            let right_span = *right_span;
                            let Some(Node::Span {
                                size: ref mut span_size,
                            }) = memory.get_mut(i)
                            else {
                                unreachable!()
                            };
                            *span_size += right_span;
                            memory.remove(i + 1);
                        }
                        if let Some(Node::Span { size: left_span }) = memory.get(i - 1) {
                            let left_span = *left_span;
                            let Some(Node::Span {
                                size: ref mut span_size,
                            }) = memory.get_mut(i)
                            else {
                                unreachable!()
                            };
                            *span_size += left_span;
                            memory.remove(i - 1);
                            decrease_i_by += 1;
                        }
                        i -= decrease_i_by;
                        break;
                    }
                }
                j += 1;
            }
        }
        i -= 1;
    }
    // println!("{:?}", &memory);
    let checksum: usize = memory
        .iter()
        .map(|node| match node {
            Node::File { id, size } => iter::repeat_n(*id, *size as usize),
            Node::Span { size } => iter::repeat_n(0, *size as usize),
        })
        .flatten()
        .enumerate()
        .map(|(a, b)| a * b)
        .sum();
    println!("The non-fragmented checksum is {}", checksum);
}

fn main() {
    part1();
    part2();
}
