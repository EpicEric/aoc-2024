static INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq)]
enum Orientation {
    Increasing,
    Decreasing,
}

fn is_safe(report: &[isize], can_dampen: bool) -> bool {
    let mut orientation: Option<Orientation> = None;
    for (index, slice) in report.windows(2).enumerate() {
        let [first, second] = slice else {
            return false;
        };
        match second - first {
            1 | 2 | 3 => {
                if orientation == Some(Orientation::Decreasing) {
                    if can_dampen {
                        if index == 1 {
                            let report_without_initial = &report[1..];
                            let report_without_first =
                                [report[..index].to_vec(), report[index + 1..].to_vec()].concat();
                            let report_without_second =
                                [report[..index + 1].to_vec(), report[index + 2..].to_vec()]
                                    .concat();
                            return is_safe(report_without_initial, false)
                                || is_safe(report_without_first.as_slice(), false)
                                || is_safe(report_without_second.as_slice(), false);
                        } else {
                            let report_without_first =
                                [report[..index].to_vec(), report[index + 1..].to_vec()].concat();
                            let report_without_second =
                                [report[..index + 1].to_vec(), report[index + 2..].to_vec()]
                                    .concat();
                            return is_safe(report_without_first.as_slice(), false)
                                || is_safe(report_without_second.as_slice(), false);
                        }
                    } else {
                        return false;
                    }
                }
                orientation = Some(Orientation::Increasing);
            }
            -1 | -2 | -3 => {
                if orientation == Some(Orientation::Increasing) {
                    if can_dampen {
                        if index == 1 {
                            let report_without_initial = &report[1..];
                            let report_without_first =
                                [report[..index].to_vec(), report[index + 1..].to_vec()].concat();
                            let report_without_second =
                                [report[..index + 1].to_vec(), report[index + 2..].to_vec()]
                                    .concat();
                            return is_safe(report_without_initial, false)
                                || is_safe(report_without_first.as_slice(), false)
                                || is_safe(report_without_second.as_slice(), false);
                        } else {
                            let report_without_first =
                                [report[..index].to_vec(), report[index + 1..].to_vec()].concat();
                            let report_without_second =
                                [report[..index + 1].to_vec(), report[index + 2..].to_vec()]
                                    .concat();
                            return is_safe(report_without_first.as_slice(), false)
                                || is_safe(report_without_second.as_slice(), false);
                        }
                    } else {
                        return false;
                    }
                }
                orientation = Some(Orientation::Decreasing);
            }
            _ if can_dampen => {
                let report_without_first =
                    [report[..index].to_vec(), report[index + 1..].to_vec()].concat();
                let report_without_second =
                    [report[..index + 1].to_vec(), report[index + 2..].to_vec()].concat();
                return is_safe(report_without_first.as_slice(), false)
                    || is_safe(report_without_second.as_slice(), false);
            }
            _ => return false,
        }
    }
    true
}

fn part1() {
    let reports = INPUT.lines().filter_map(|line| {
        line.split(' ')
            .map(|level| level.parse::<isize>().ok())
            .collect::<Option<Vec<_>>>()
    });
    let safe_reports = reports
        .filter_map(|report| {
            if is_safe(report.as_slice(), false) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("The number of safe reports is {safe_reports}");
}

fn part2() {
    let reports = INPUT.lines().filter_map(|line| {
        line.split(' ')
            .map(|level| level.parse::<isize>().ok())
            .collect::<Option<Vec<_>>>()
    });
    let safe_reports = reports
        .filter_map(|report| {
            if is_safe(report.as_slice(), true) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("The number of safe reports with dampening is {safe_reports}");
}

fn main() {
    part1();
    part2();
}
