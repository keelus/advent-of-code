type Position = (isize, isize);

const COST_A: isize = 3;
const COST_B: isize = 1;

#[derive(Debug)]
pub struct Claw {
    a: Position,
    b: Position,
    prize: Position,
}

pub fn parse<'p>(input_data: String) -> Vec<Claw> {
    let line_count = input_data.lines().count();

    let mut claws: Vec<Claw> = Vec::new();

    let mut current_claw = None;
    input_data.lines().enumerate().for_each(|(i, line)| {
        let line = line.trim();

        if line.is_empty() {
            claws.push(current_claw.take().unwrap());
        } else {
            if current_claw.is_none() {
                current_claw = Some(Claw {
                    a: (0, 0),
                    b: (0, 0),
                    prize: (0, 0),
                });
            }

            if let Some(ref mut current_claw) = current_claw {
                let parts = line.split(":").collect::<Vec<_>>();

                let part_type = parts[0];
                let (x, y) = {
                    let values: Vec<isize> = parts[1]
                        .trim()
                        .split(", ")
                        .map(|v| v.get(2..).unwrap().parse().unwrap())
                        .collect();

                    (values[0], values[1])
                };

                match part_type {
                    "Button A" => current_claw.a = (x, y),
                    "Button B" => current_claw.b = (x, y),
                    "Prize" => current_claw.prize = (x, y),
                    _ => (),
                }
            }
        }

        if current_claw.is_some() && i == line_count - 1 {
            claws.push(current_claw.take().unwrap());
        }
    });

    claws
}

// Solves:
//
// times_a * a.0 + times_b * b.0 = prize.0
// times_a * a.1 + times_b * b.1 = prize.1
fn explore(claw: &Claw) -> Option<isize> {
    let times_b = (claw.a.1 * claw.prize.0 - claw.a.0 * claw.prize.1)
        / (claw.a.1 * claw.b.0 - claw.a.0 * claw.b.1);
    let times_a = (claw.prize.1 - claw.b.1 * times_b) / claw.a.1;

    let final_position = (
        times_a * claw.a.0 + times_b * claw.b.0,
        times_a * claw.a.1 + times_b * claw.b.1,
    );

    (final_position == claw.prize).then_some(times_a * COST_A + times_b * COST_B)
}

pub fn part_1(claws: &[Claw]) -> i64 {
    claws.iter().filter_map(|claw| explore(claw)).sum::<isize>() as i64
}

pub fn part_2(claws: &[Claw]) -> i64 {
    claws
        .iter()
        .filter_map(|claw| {
            let new_claw = Claw {
                a: claw.a,
                b: claw.b,
                prize: (
                    claw.prize.0 + 10_000_000_000_000,
                    claw.prize.1 + 10_000_000_000_000,
                ),
            };
            explore(&new_claw)
        })
        .sum::<isize>() as i64
}
