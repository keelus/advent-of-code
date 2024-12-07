use clap::{command, Arg, ArgAction};
use colored::{Color, Colorize};
use std::{
    fs,
    time::{Duration, Instant},
};

mod common;
mod solutions;
use solutions::*;

fn format_nanos(nanos: u128) -> (u128, &'static str) {
    let spent = Duration::from_nanos(nanos as u64);
    match spent.as_nanos() {
        0..=999 => (spent.as_nanos(), "ns"),
        1_000..=999_999 => (spent.as_micros(), "μs"),
        1_000_000..=999_999_999 => (spent.as_millis(), "ms"),
        _ => (spent.as_secs().into(), "s"),
    }
}

macro_rules! time_action {
    ($action:path, $($args:expr),*) => {{
        let start = Instant::now();
        let result = std::hint::black_box($action($($args),*));
        let spent = Instant::now().duration_since(start);

        let (time, unit) = format_nanos(spent.as_nanos());

        ActionResult {
            value: result,
            time_nanos: spent.as_nanos(),
            time,
            time_unit: unit,
        }
    }};
}

struct ActionResult<T> {
    value: T,
    time_nanos: u128,
    time: u128,
    time_unit: &'static str,
}

fn get_time_str(
    prefix_len: usize,
    terminal_width: usize,
    time: u128,
    time_unit: &str,
    show_avg: bool,
) -> String {
    let visual = format!(
        "({}{}{})",
        if show_avg { "avg=" } else { "" },
        time,
        time_unit
    );
    let width = terminal_width.saturating_sub(prefix_len);
    format!("{:>width$}", visual.truecolor(150, 150, 150))
}

fn print_part(
    prefix: &str,
    terminal_width: usize,
    value: Option<i64>,
    time: u128,
    time_unit: &str,
    is_bench: bool,
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
        get_time_str(
            complete_prefix_len,
            terminal_width,
            time,
            time_unit,
            is_bench
        ),
        if !is_bench && value.is_none() {
            "\n"
        } else {
            ""
        }
    );
}

fn print_centered(
    text: &str,
    side_margin: usize,
    visual_width: usize,
    terminal_width: usize,
    color: Color,
) {
    let (side_left, side_right) = {
        let side_width = ((terminal_width - side_margin * 2) as f32 - visual_width as f32) / 2.0;
        (side_width.floor() as usize, side_width.ceil() as usize)
    };

    println!(
        "|{:>side_left$}{}{:>side_right$}|",
        " ",
        text.color(color),
        " "
    );
}

fn print_title_box(day_number: usize, benchmark_runs: usize, terminal_width: usize) {
    // Top bar
    println!("+{}+", "=".repeat(terminal_width - 2));

    // Day information
    let day_str = format!("🎄 Day {:0>2} 🎄", day_number);
    let day_str_visual_width = day_str.chars().count() + 2;
    print_centered(
        &day_str,
        1,
        day_str_visual_width,
        terminal_width,
        Color::Yellow,
    );

    // Bench information
    if benchmark_runs > 0 {
        let bench_str = format!("Benchmarking {} times", benchmark_runs);
        let bench_str_visual_width = bench_str.chars().count();
        print_centered(
            &bench_str,
            1,
            bench_str_visual_width,
            terminal_width,
            Color::Red,
        );
    }

    // Bottom bar
    println!("+{}+", "=".repeat(terminal_width - 2));
}

// ($day_num:expr, $day_mod:ident, $input_path:expr, $benchmark_runs:expr) => {{
macro_rules! advent_day {
    ($day_module:ident, $args:expr) => {{
        let input_data = fs::read_to_string(format!(
            "./inputs/day{:0>2}/{}",
            $args.day, $args.input_path
        ))
        .expect("The day input file does not exist");

        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(150);

        print_title_box($args.day, $args.benchmark_runs, term_width);

        if $args.benchmark_runs > 0 {
            let mut benchs = [("Parse", 0), ("Part 1", 0), ("Part 2", 0)];

            for _i in 0..$args.benchmark_runs {
                let parse = time_action!($day_module::parse, input_data.clone());

                benchs[0].1 += parse.time_nanos;
                benchs[1].1 += time_action!($day_module::part_1, &parse.value).time_nanos;
                benchs[2].1 += time_action!($day_module::part_2, &parse.value).time_nanos;
            }

            benchs.iter().for_each(|(bench_name, time)| {
                let avg_time = time / ($args.benchmark_runs as u128);
                let (parsed_time, time_unit) = format_nanos(avg_time);

                print_part(bench_name, term_width, None, parsed_time, time_unit, true)
            });
        } else {
            let parse = time_action!($day_module::parse, input_data);
            let part1 = time_action!($day_module::part_1, &parse.value);
            let part2 = time_action!($day_module::part_2, &parse.value);

            // Print parts
            print_part(
                "Parse",
                term_width,
                None,
                parse.time,
                parse.time_unit,
                false,
            );
            print_part(
                "Part 1",
                term_width,
                Some(part1.value),
                part1.time,
                part1.time_unit,
                false,
            );
            print_part(
                "Part 2",
                term_width,
                Some(part2.value),
                part2.time,
                part2.time_unit,
                false,
            );
        }
    }};
}

struct Args<'a> {
    day: usize,
    input_path: &'a str,
    benchmark_runs: usize,
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
        .arg(
            Arg::new("benchmark_runs")
                .short('b')
                .long("bench")
                .default_value("0"),
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

    let benchmark_runs: usize = {
        let bench_str = matches.get_one::<String>("benchmark_runs").unwrap();

        bench_str
            .parse::<usize>()
            .expect(&format!("Invalid bench amount to run \"{bench_str}\""))
    };

    let args = Args {
        day: day_to_run,
        input_path,
        benchmark_runs,
    };

    match day_to_run {
        1 => advent_day!(day01, args),
        2 => advent_day!(day02, args),
        3 => advent_day!(day03, args),
        4 => advent_day!(day04, args),
        5 => advent_day!(day05, args),
        6 => advent_day!(day06, args),
        7 => advent_day!(day07, args),
        // 8 => advent_day!(day08, args),
        // 9 => advent_day!(day09, args),
        // 10 => advent_day!(day10, args),
        // 11 => advent_day!(day11, args),
        // 12 => advent_day!(day12, args),
        // 13 => advent_day!(day13, args),
        // 14 => advent_day!(day14, args),
        // 15 => advent_day!(day15, args),
        // 16 => advent_day!(day16, args),
        // 17 => advent_day!(day17, args),
        // 18 => advent_day!(day18, args),
        // 19 => advent_day!(day19, args),
        // 20 => advent_day!(day20, args),
        // 21 => advent_day!(day21, args),
        // 22 => advent_day!(day22, args),
        // 23 => advent_day!(day23, args),
        // 24 => advent_day!(day24, args),
        // 25 => advent_day!(day25, args),
        _ => eprintln!("The day {day_to_run} is not yet implemented!"),
    }
}
