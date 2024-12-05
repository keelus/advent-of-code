use std::collections::HashMap;

pub struct Input {
    page_orderings: HashMap<i64, Vec<i64>>,
    page_updates: Vec<Vec<i64>>,
}

pub fn parse(input_data: String) -> Input {
    let page_orderings: HashMap<i64, Vec<i64>> = input_data
        .lines()
        .map_while(|l| (!l.is_empty()).then_some(l.trim().split("|").collect::<Vec<&str>>()))
        .map(|p| [p[0].parse::<i64>().unwrap(), p[1].parse().unwrap()])
        .fold(HashMap::new(), |mut acc, [num, after_num]| {
            acc.entry(num)
                .and_modify(|e| e.push(after_num))
                .or_insert(vec![after_num]);
            acc
        });

    let page_updates: Vec<Vec<i64>> = input_data
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.trim().split(","))
        .map(|p| p.map(|n| n.parse().unwrap()).collect())
        .collect();

    Input {
        page_orderings,
        page_updates,
    }
}

enum FilterMode {
    RemoveValids,
    RemoveInvalids,
}

fn filter_page_updates<'a>(
    filter_mode: FilterMode,
    page_updates: &'a Vec<Vec<i64>>,
    page_orderings: &'a HashMap<i64, Vec<i64>>,
) -> impl Iterator<Item = &'a [i64]> {
    page_updates.iter().filter_map(move |update_nums| {
        // Strategy: Reverse the update number list, and check if
        // the one we are evaluating exists in the ordering rules
        // of already viewed numbers.
        // If so, then is not valid.

        let valid: bool = update_nums
            .iter()
            .rev()
            .fold((true, vec![]), |(mut valid, mut viewed_nums), num| {
                viewed_nums.iter().for_each(|viewed_num| {
                    if let Some(ordering) = page_orderings.get(viewed_num) {
                        if ordering.contains(num) {
                            valid = false;
                        }
                    }
                });
                viewed_nums.push(*num);

                (valid, viewed_nums)
            })
            .0;

        match filter_mode {
            FilterMode::RemoveInvalids => valid.then_some(update_nums.as_slice()),
            FilterMode::RemoveValids => (!valid).then_some(update_nums.as_slice()),
        }
    })
}

pub fn part_1(input: &Input) -> i64 {
    filter_page_updates(
        FilterMode::RemoveInvalids,
        &input.page_updates,
        &input.page_orderings,
    )
    .map(|nums| nums[nums.len() / 2])
    .sum()
}

pub fn part_2(input: &Input) -> i64 {
    let incorrect_updates = filter_page_updates(
        FilterMode::RemoveValids,
        &input.page_updates,
        &input.page_orderings,
    );

    // Given the sample, we can notice this "pattern":
    //
    // 75,97,47,61,53
    // 97 ->goes before [13, 61, 47, 29, 53, 75]  -> [47, 53, 61, 75] -> 4
    // 75 ->goes before [29, 53, 47, 61, 13]      -> [47, 53, 61] -> 3
    // 47 ->goes before [53, 13, 61, 29]          -> [53, 61] -> 2
    // 61 ->goes before [13, 53, 29]              -> [53] -> 1
    // 53 ->goes before [29, 13]                  -> [] -> 0
    //
    // 61,13,29
    // 61 ->goes before [13, 53, 29] -> [13, 29]
    // 29 ->goes before [13]         -> [13]
    // 13 ->goes before []           -> []
    // ...

    incorrect_updates
        .map(|update_nums| {
            let mut filtered_nums = update_nums
                .iter()
                .map(|num| {
                    (
                        *num,
                        input
                            .page_orderings
                            .get(num)
                            .unwrap_or(&vec![])
                            .iter()
                            .filter(|&after_num| update_nums.contains(after_num))
                            .map(|&after_num| after_num)
                            .collect::<Vec<i64>>(),
                    )
                })
                .collect::<Vec<(i64, Vec<i64>)>>();

            let l = filtered_nums.len();
            filtered_nums.sort_by_key(|nums| nums.1.len());

            *filtered_nums
                .iter()
                .rev()
                .map(|(num, _)| num)
                .collect::<Vec<_>>()[l / 2]
        })
        .sum()
}
