type Report = Vec<i64>;

pub fn parse(input_data: String) -> Vec<Report> {
    input_data
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|level| level.parse::<i64>().expect("Invalid input data."))
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &Report) -> bool {
    let mut increasing = None;

    report
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .all(|pair| {
            let (current, next) = pair;

            let diff = next - current;

            if diff.abs() > 3 || diff == 0 {
                return false;
            }

            if let Some(increasing) = increasing {
                if increasing != (diff > 0) {
                    return false;
                }
            } else {
                increasing = Some(diff > 0)
            }

            true
        })
}

pub fn part_1(reports: &[Report]) -> i64 {
    reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count() as i64
}

pub fn part_2(reports: &[Report]) -> i64 {
    reports
        .iter()
        .filter(|&report| {
            if is_report_safe(report) {
                true
            } else {
                report.iter().enumerate().any(|(i, _)| {
                    let mut new_report = report.clone();
                    new_report.remove(i);
                    is_report_safe(&new_report)
                })
            }
        })
        .count() as i64
}
