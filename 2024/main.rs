use clap::{command, Arg, ArgAction};
use colored::Colorize;
use std::{fs, time::Instant};

mod common;
mod solutions;
use solutions::*;

macro_rules! time_action {
    ($action:path, $($args:expr),*) => {{
        let start = Instant::now();
        let result = $action($($args),*);
        let spent = Instant::now().duration_since(start);

        let (time, unit) = match spent.as_nanos() {
            0..=999 => (spent.as_nanos(), "ns"),
            1_000..=999_999 => (spent.as_micros(), "μs"),
            1_000_000..=999_999_999 => (spent.as_millis(), "ms"),
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
    ($day_num:expr, $day_mod:ident, $input_path:expr) => {{
        let input_data = fs::read_to_string(format!("./inputs/day{:0>2}/{}", $day_num, $input_path)).expect("The day input file does not exist");

        let parse = time_action!($day_mod::parse, input_data);
        let part1 = time_action!($day_mod::part_1, &parse.value);
        let part2 = time_action!($day_mod::part_2, &parse.value);

        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(150);

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
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("day").short('d').long("day").default_value("1"))
        .arg(
            Arg::new("sample")
                .short('s')
                .long("sample")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let day_to_run: usize = {
        let day_str = matches.get_one::<String>("day").unwrap();

        day_str
            .parse::<usize>()
            .expect(&format!("Invalid day to run \"{day_str}\""))
    };

    let input_path = if matches.get_flag("sample") {
        "sample.txt"
    } else {
        "input.txt"
    };

    match day_to_run {
        1 => advent_day!(1, day01, input_path),
        // 2 => advent_day!(2, day02, input_path),
        // 3 => advent_day!(3, day03, input_path),
        // 4 => advent_day!(4, day04, input_path),
        // 5 => advent_day!(5, day05, input_path),
        // 6 => advent_day!(6, day06, input_path),
        // 7 => advent_day!(7, day07, input_path),
        // 8 => advent_day!(8, day08, input_path),
        // 9 => advent_day!(9, day09, input_path),
        // 10 => advent_day!(10, day10, input_path),
        // 11 => advent_day!(11, day11, input_path),
        // 12 => advent_day!(12, day12, input_path),
        // 13 => advent_day!(13, day13, input_path),
        // 14 => advent_day!(14, day14, input_path),
        // 15 => advent_day!(15, day15, input_path),
        // 16 => advent_day!(16, day16, input_path),
        // 17 => advent_day!(17, day17, input_path),
        // 18 => advent_day!(18, day18, input_path),
        // 19 => advent_day!(19, day19, input_path),
        // 20 => advent_day!(20, day20, input_path),
        // 21 => advent_day!(21, day21, input_path),
        // 22 => advent_day!(22, day22, input_path),
        // 23 => advent_day!(23, day23, input_path),
        // 24 => advent_day!(24, day24, input_path),
        // 25 => advent_day!(25, day25, input_path),
        _ => eprintln!("The day {day_to_run} is not yet implemented!"),
    }
}
