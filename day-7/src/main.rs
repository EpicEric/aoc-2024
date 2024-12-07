static INPUT: &str = include_str!("./input.txt");

fn has_valid_equation(result: usize, operands: &[usize], stack: Option<usize>) -> bool {
    match (operands, stack) {
        (&[], _) => return result == 0,
        (&[last], None) => return result == last,
        (&[last], Some(factor)) => return result == last + factor || result == last * factor,
        ([next, remainder @ ..], None) => {
            if *next > result {
                return false;
            }
            return has_valid_equation(result, &remainder, Some(*next));
        }
        ([next, remainder @ ..], Some(factor)) => {
            if next + factor > result {
                return false;
            }
            if next * factor > result {
                return has_valid_equation(result, &remainder, Some(next + factor));
            } else {
                return has_valid_equation(result, &remainder, Some(next + factor))
                    || has_valid_equation(result, &remainder, Some(next * factor));
            }
        }
    }
}

fn has_valid_equation_with_concatenation(
    result: usize,
    operands: &[usize],
    stack: Option<usize>,
) -> bool {
    match (operands, stack) {
        (&[], _) => return result == 0,
        (&[last], None) => return result == last,
        (&[last], Some(factor)) => {
            return result == last + factor
                || result == last * factor
                || result == format!("{}{}", factor, last).parse().unwrap()
        }
        ([next, remainder @ ..], None) => {
            if *next > result {
                return false;
            }
            return has_valid_equation_with_concatenation(result, &remainder, Some(*next));
        }
        ([next, remainder @ ..], Some(factor)) => {
            if next + factor > result {
                return false;
            }
            let concatenated = format!("{}{}", factor, next).parse().unwrap();
            if next * factor > result {
                if concatenated > result {
                    return has_valid_equation_with_concatenation(
                        result,
                        &remainder,
                        Some(next + factor),
                    );
                } else {
                    return has_valid_equation_with_concatenation(
                        result,
                        &remainder,
                        Some(next + factor),
                    ) || has_valid_equation_with_concatenation(
                        result,
                        &remainder,
                        Some(concatenated),
                    );
                }
            } else if concatenated > result {
                return has_valid_equation_with_concatenation(
                    result,
                    &remainder,
                    Some(next + factor),
                ) || has_valid_equation_with_concatenation(
                    result,
                    &remainder,
                    Some(next * factor),
                );
            } else {
                return has_valid_equation_with_concatenation(
                    result,
                    &remainder,
                    Some(next + factor),
                ) || has_valid_equation_with_concatenation(
                    result,
                    &remainder,
                    Some(next * factor),
                ) || has_valid_equation_with_concatenation(
                    result,
                    &remainder,
                    Some(concatenated),
                );
            }
        }
    }
}

fn part1() {
    let equations =
        INPUT
            .lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(value, operands)| {
                (
                    value.parse::<usize>().unwrap(),
                    operands
                        .split(' ')
                        .map(|operand| operand.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            });
    let result: usize = equations
        .filter(|(result, operands)| has_valid_equation(*result, operands.as_slice(), None))
        .map(|(value, _)| value)
        .sum();
    println!("The total calibration result is {}", result);
}

fn part2() {
    let equations =
        INPUT
            .lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(value, operands)| {
                (
                    value.parse::<usize>().unwrap(),
                    operands
                        .split(' ')
                        .map(|operand| operand.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            });
    let result: usize = equations
        .filter(|(result, operands)| {
            has_valid_equation_with_concatenation(*result, operands.as_slice(), None)
        })
        .map(|(value, _)| value)
        .sum();
    println!("The correct total calibration result is {}", result);
}

fn main() {
    part1();
    part2();
}
