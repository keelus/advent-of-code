use colored::Colorize;
use core::panic;
use std::time::Instant;

mod common;
mod day01;

macro_rules! time_action {
    ($action:path, $($args:expr),*) => {{
        let start = Instant::now();
        let result = $action($($args),*);
        let spent = Instant::now().duration_since(start);

        let (time, unit) = match spent.as_nanos() {
            0..=999 => (spent.as_nanos(), "ns"),
            1_000..=9_999 => (spent.as_micros(), "μs"),
            1_000_000..=9_999_999 => (spent.as_millis(), "ms"),
            _ => (spent.as_secs().into(), "s")
        };

        ActionResult {
            value: result,
            time,
            time_unit: unit,
        }
    }};
}

struct ActionResult<T> {
    value: T,
    time: u128,
    time_unit: &'static str,
}

fn get_time_str(prefix_len: usize, terminal_width: usize, time: u128, time_unit: &str) -> String {
    let visual = format!("({}{})", time, time_unit);
    let width = terminal_width.saturating_sub(prefix_len);
    format!("{:>width$}", visual.truecolor(150, 150, 150))
}

fn print_part(
    prefix: &str,
    terminal_width: usize,
    value: Option<i64>,
    time: u128,
    time_unit: &str,
) {
    let complete_prefix = if let Some(value) = value {
        format!(
            "{}: {}",
            prefix.cyan().bold(),
            value.to_string().white().bold()
        )
    } else {
        prefix.cyan().bold().to_string()
    };
    let complete_prefix_len = if let Some(value) = value {
        format!("{}: {}", prefix, value).len()
    } else {
        prefix.len()
    };
    println!(
        "{complete_prefix}{}{}",
        get_time_str(complete_prefix_len, terminal_width, time, time_unit),
        if value.is_none() { "\n" } else { "" }
    );
}

macro_rules! advent_day {
    ($day_num:expr, $day_mod:ident) => {{
        let parse = time_action!($day_mod::parse,);
        let part1 = time_action!($day_mod::part_1, &parse.value);
        let part2 = time_action!($day_mod::part_2, &parse.value);

        let term_width = if let Some((w, _)) = term_size::dimensions() {
            w
        } else {
            150
        };

        // Print day title and box
        println!("+{}+", "=".repeat(term_width - 2));
        let day_str = format!("🎄 Day {:0>2} 🎄", $day_num);
        let day_side_width = (term_width - 2 /* subtract both + */ - day_str.chars().count() - 2 /* approx emoji extra w */) / 2;
        println!(
            "|{:>day_side_width$}{}{:>day_side_width$}|",
            " ", day_str.yellow(), " "
        );
        println!("+{}+", "=".repeat(term_width - 2));

        // Print parts
        print_part("Parse", term_width, None, parse.time, parse.time_unit);
        print_part("Part 1", term_width, Some(part1.value), part1.time, part1.time_unit);
        print_part("Part 2", term_width, Some(part2.value), part2.time, part2.time_unit);
    }};
}

fn main() {
    let day_to_run: usize = 1;

    match day_to_run {
        1 => advent_day!(1, day01),
        // 2 => advent_day!(2, day02),
        // 3 => advent_day!(3, day03),
        // 4 => advent_day!(4, day04),
        // 5 => advent_day!(5, day05),
        // 6 => advent_day!(6, day06),
        // 7 => advent_day!(7, day07),
        // 8 => advent_day!(8, day08),
        // 9 => advent_day!(9, day09),
        // 10 => advent_day!(10, day10),
        // 11 => advent_day!(11, day11),
        // 12 => advent_day!(12, day12),
        // 13 => advent_day!(13, day13),
        // 14 => advent_day!(14, day14),
        // 15 => advent_day!(15, day15),
        // 16 => advent_day!(16, day16),
        // 17 => advent_day!(17, day17),
        // 18 => advent_day!(18, day18),
        // 19 => advent_day!(19, day19),
        // 20 => advent_day!(20, day20),
        // 21 => advent_day!(21, day21),
        // 22 => advent_day!(22, day22),
        // 23 => advent_day!(23, day23),
        // 24 => advent_day!(24, day24),
        _ => panic!("Day does not exist!"),
    }
}
