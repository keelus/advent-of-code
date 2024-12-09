use std::collections::HashSet;

type Part1BlockId = i64;
type Part1Memory = Vec<Option<Part1BlockId>>;

#[derive(Debug, Copy, Clone)]
pub enum Part2Block {
    Empty { size: usize },             // Block size
    NonEmpty { size: usize, id: i64 }, // Block size, file id
}
pub type Part2Memory = Vec<Part2Block>;

pub struct ParsedParts {
    part_1: Part1Memory,
    part_2: Part2Memory,
}

pub fn parse<'p>(input_data: String) -> ParsedParts {
    let mut disk_line = String::new();

    let part_1_memory = {
        let mut memory = vec![];

        let mut is_blank = false;
        let mut disk_index = 0;

        for element in input_data.trim().chars() {
            let num: i64 = element.to_string().parse().unwrap();

            if is_blank {
                for _i in 0..num {
                    disk_line.push_str(".");
                    memory.push(None);
                }
            } else {
                for _i in 0..num {
                    disk_line.push_str(&disk_index.to_string());
                    memory.push(Some(disk_index));
                }
                disk_index += 1;
            }

            is_blank = !is_blank;
        }

        memory
    };

    let part_2_memory = {
        let mut memory = vec![];

        let mut is_blank = false;
        let mut disk_index = 0;

        for element in input_data.trim().chars() {
            let num: usize = element.to_string().parse().unwrap();

            if is_blank {
                for _i in 0..num {
                    disk_line.push_str(".");
                }
                memory.push(Part2Block::Empty { size: num });
            } else {
                for _i in 0..num {
                    disk_line.push_str(&disk_index.to_string());
                }
                memory.push(Part2Block::NonEmpty {
                    size: num,
                    id: disk_index,
                });
                disk_index += 1;
            }

            is_blank = !is_blank;
        }

        memory
    };

    // println!("{:?}", part_2_memory);

    ParsedParts {
        part_1: part_1_memory,
        part_2: part_2_memory,
    }
}

// Will return the leftmost free block index, only if the
// disk is non contiguous.
fn leftmost_free_block_index(memory: &Part1Memory) -> Option<usize> {
    let mut first_empty = None;

    for (i, block) in memory.iter().enumerate() {
        if block.is_none() {
            if first_empty.is_none() {
                first_empty = Some(i);
            }
        } else if first_empty.is_some() {
            return first_empty;
        }
    }

    return None;
}

fn rightmost_block(memory: &Part1Memory) -> (usize, Part1BlockId) {
    for (i, block) in memory.iter().enumerate().rev() {
        if !block.is_none() {
            return (i, block.unwrap());
        }
    }

    unreachable!("Found empty only");
}

fn calculate_checksum(memory: &Part1Memory) -> i64 {
    memory
        .iter()
        .enumerate()
        .take_while(|&(_, b)| b.is_some())
        .map(|(i, b)| (i as i64) * b.unwrap())
        .sum()
}

pub fn part_1(parsed_parts: &ParsedParts) -> i64 {
    let mut memory = parsed_parts.part_1.clone();

    while let Some(leftmost_free_index) = leftmost_free_block_index(&memory) {
        // Optimization TODO:
        // Save the leftmost and rightmos indexes? So is easier where begin

        let (rightmost_index, block_id) = rightmost_block(&memory);

        memory[leftmost_free_index] = Some(block_id);
        memory[rightmost_index] = None;
    }

    calculate_checksum(&memory)
}

pub fn is_space_available(memory: &Part2Memory, size_needed: usize) -> Option<usize> {
    for i in 0..memory.len() {
        match memory[i] {
            Part2Block::NonEmpty { .. } => continue,
            Part2Block::Empty { size } => {
                if size >= size_needed {
                    return Some(i);
                }
            }
        }
    }

    None
}

pub fn print_memory(memory: &Part2Memory) {
    let mut output = String::new();

    for block in memory.iter() {
        match block {
            Part2Block::Empty { size } => {
                for _i in 0..*size {
                    output.push_str(".");
                }
            }
            Part2Block::NonEmpty { id, size } => {
                for _i in 0..*size {
                    if *id < 10 {
                        output.push_str(&id.to_string());
                    } else {
                        output.push_str(&format!("({id})"));
                    }
                }
            }
        }
    }

    println!("{output}");
}

pub fn find_first_non_empty_merged(memory: &Part2Memory) -> Option<usize> {
    for i in 0..memory.len() - 1 {
        let cur = memory[i];
        let next = memory[i + 1];

        if let Part2Block::Empty { .. } = cur {
            if let Part2Block::Empty { .. } = next {
                return Some(i);
            }
        }
    }

    None
}

pub fn merge_empty_blocks(mut memory: Part2Memory) -> Part2Memory {
    if let Some(i) = find_first_non_empty_merged(&memory) {
        // Merge with right one
        let next_block = memory.remove(i + 1);

        if let Part2Block::Empty {
            size: ref mut cur_size,
        } = memory[i]
        {
            if let Part2Block::Empty { size: next_size } = next_block {
                *cur_size += next_size;
            }
        }

        merge_empty_blocks(memory)
    } else {
        memory
    }
}

fn calculate_checksum_2(memory: &Part2Memory) -> i64 {
    let mut global_index = 0i64;
    memory
        .iter()
        .map(|b| match b {
            Part2Block::NonEmpty { size, id } => {
                let mut loc_sum = 0;
                for _i in 0..*size {
                    loc_sum += global_index * id;
                    global_index += 1 as i64;
                }
                loc_sum
            }
            Part2Block::Empty { size } => {
                global_index += *size as i64;
                0
            }
        })
        .sum()
}

pub fn part_2(parsed_parts: &ParsedParts) -> i64 {
    let mut memory = parsed_parts.part_2.clone();

    let non_empty_block_len = memory
        .iter()
        .filter(|block| {
            if let Part2Block::Empty { .. } = block {
                return false;
            } else {
                return true;
            }
        })
        .count();

    let mut viewed_blocks = HashSet::new();

    loop {
        for (i, &file_block) in memory.iter().enumerate().rev() {
            if let Part2Block::NonEmpty { size, id } = file_block {
                if viewed_blocks.contains(&id) {
                    continue;
                }

                viewed_blocks.insert(id);

                let mut added_block = false;
                let mut new_memory = memory.clone();
                if let Some(empty_index) = is_space_available(&memory, size) {
                    if empty_index < i {
                        // Do the replacement
                        let empty_block = memory[empty_index];
                        new_memory[i] = Part2Block::Empty { size };
                        added_block = true;

                        if let Part2Block::Empty {
                            size: empty_block_size,
                        } = empty_block
                        {
                            // Replace / insert the block into the empty space
                            if empty_block_size == size {
                                new_memory[empty_index] = Part2Block::NonEmpty { size, id };
                            } else {
                                let new_empty_block = Part2Block::Empty {
                                    size: empty_block_size - size,
                                };
                                new_memory[empty_index] = Part2Block::NonEmpty { size, id };
                                new_memory.insert(empty_index + 1, new_empty_block);
                            }
                        } else {
                            panic!("Returned a non empty instead of an empty?");
                        }
                    }
                }

                if added_block {
                    // Merge contiguous empty blocks
                    let new_memory = merge_empty_blocks(new_memory);
                    memory = new_memory;
                    break;
                }
            }
        }

        if viewed_blocks.len() == non_empty_block_len {
            break;
        }
    }

    // print_memory(&memory);
    calculate_checksum_2(&memory)
}
