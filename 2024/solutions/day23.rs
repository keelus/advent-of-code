use std::collections::{self, HashMap, HashSet};

use itertools::Itertools;

pub fn parse(input_data: String) -> HashMap<String, HashSet<String>> {
    input_data.lines().fold(HashMap::new(), |mut acc, l| {
        let mut computer_pair = l.trim().split("-").collect::<Vec<_>>();
        computer_pair.sort();

        if let Some(ref mut set) = acc.get_mut(computer_pair[0]) {
            set.insert(computer_pair[1].to_owned());
        } else {
            let mut set = HashSet::new();
            set.insert(computer_pair[1].to_owned());
            acc.insert(computer_pair[0].to_owned(), set);
        }

        acc
    })
}

pub fn part_1(computer_connections: &HashMap<String, HashSet<String>>) -> i64 {
    let triple_connections = computer_connections
        .iter()
        .map(|(from, to_v)| {
            to_v.iter()
                .combinations(2)
                .filter_map(|computers| {
                    let mut computers = computers.clone();
                    computers.sort();

                    if let Some(v) = computer_connections.get(computers[0]) {
                        if v.contains(computers[1]) {
                            computers.push(from);
                            return Some(computers);
                        }
                    }

                    None
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    triple_connections
        .iter()
        .filter(|triplet| triplet.iter().any(|name| name.starts_with("t")))
        .count() as i64
}

pub fn part_2(computer_connections: &HashMap<String, HashSet<String>>) -> i64 {
    let mut largest_interconnected_computers = computer_connections
        .iter()
        .filter_map(|(from, to_v)| {
            let computers = to_v.iter().chain(std::iter::once(from)).collect::<Vec<_>>();
            let len = computers.len();

            // Get biggest connected computer group
            (2..=len).into_iter().rev().find_map(|len| {
                let interconnected_computers = computers
                    .clone()
                    .into_iter()
                    .combinations(len)
                    .filter_map(|computers| {
                        let pairs = computers.iter().combinations(2).collect::<Vec<_>>();

                        let all_connected = pairs.into_iter().all(|mut pair| {
                            pair.sort();

                            if let Some(v) = computer_connections.get(*pair[0]) {
                                if v.contains(*pair[1]) {
                                    return true;
                                }
                            }
                            false
                        });

                        all_connected.then_some(computers)
                    })
                    .collect::<Vec<_>>();

                (interconnected_computers.len() > 0).then_some(interconnected_computers)
            })
        })
        .flatten()
        .max_by_key(|computers| computers.len())
        .unwrap();
    largest_interconnected_computers.sort();

    println!(
        "Answer: \"{}\"",
        largest_interconnected_computers.iter().join(","),
    );

    0
}
