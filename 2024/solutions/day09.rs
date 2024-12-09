use std::collections::HashSet;

pub struct ParsedParts {
    part_1: part_01::Memory,
    part_2: part_02::Memory,
}

pub fn parse<'p>(input_data: String) -> ParsedParts {
    ParsedParts {
        part_1: part_01::parse(&input_data),
        part_2: part_02::parse(&input_data),
    }
}

mod part_01 {
    pub type BlockId = i64;
    pub type Memory = Vec<Option<BlockId>>;

    pub fn parse<'p>(input_data: &str) -> Memory {
        let mut memory = vec![];

        let mut is_blank = false;
        let mut disk_index = 0;

        for element in input_data.trim().chars() {
            let num: i64 = element.to_string().parse().unwrap();

            if is_blank {
                for _i in 0..num {
                    memory.push(None);
                }
            } else {
                for _i in 0..num {
                    memory.push(Some(disk_index));
                }
                disk_index += 1;
            }

            is_blank = !is_blank;
        }

        memory
    }

    pub fn leftmost_free_block_index(memory: &Memory) -> Option<usize> {
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

    pub fn rightmost_block(memory: &Memory) -> (usize, BlockId) {
        for (i, block) in memory.iter().enumerate().rev() {
            if !block.is_none() {
                return (i, block.unwrap());
            }
        }

        unreachable!("Found empty only");
    }

    pub fn calculate_checksum(memory: &Memory) -> i64 {
        memory
            .iter()
            .enumerate()
            .take_while(|&(_, b)| b.is_some())
            .map(|(i, b)| (i as i64) * b.unwrap())
            .sum()
    }
}

// Will return the leftmost free block index, only if the
// disk is non contiguous.
pub fn part_1(parsed_parts: &ParsedParts) -> i64 {
    let mut memory = parsed_parts.part_1.clone();

    while let Some(leftmost_free_index) = part_01::leftmost_free_block_index(&memory) {
        // Optimization TODO:
        // Save the leftmost and rightmos indexes? So is easier where begin

        let (rightmost_index, block_id) = part_01::rightmost_block(&memory);

        memory[leftmost_free_index] = Some(block_id);
        memory[rightmost_index] = None;
    }

    part_01::calculate_checksum(&memory)
}

mod part_02 {
    #[derive(Debug, Copy, Clone)]
    pub struct Block {
        pub size: usize,
        pub id: Option<i64>,
    }

    impl Block {
        pub fn new(size: usize, id: Option<i64>) -> Self {
            Self { size, id }
        }
        pub fn new_empty(size: usize) -> Self {
            Self { size, id: None }
        }
        pub fn is_empty(&self) -> bool {
            self.id.is_none()
        }
    }

    pub type Memory = Vec<Block>;

    pub fn parse<'p>(input_data: &str) -> Memory {
        let mut memory = vec![];

        let mut is_blank = false;
        let mut disk_index = 0;

        for element in input_data.trim().chars() {
            let num: usize = element.to_string().parse().unwrap();

            if is_blank {
                memory.push(Block::new_empty(num));
            } else {
                memory.push(Block::new(num, Some(disk_index)));
                disk_index += 1;
            }

            is_blank = !is_blank;
        }

        memory
    }

    pub fn is_space_available(memory: &Memory, size_needed: usize) -> Option<usize> {
        for i in 0..memory.len() {
            if memory[i].is_empty() && memory[i].size >= size_needed {
                return Some(i);
            }
        }

        None
    }

    pub fn find_first_non_empty_merged(memory: &Memory) -> Option<usize> {
        for i in 0..memory.len() - 1 {
            let cur = memory[i];
            let next = memory[i + 1];

            if cur.is_empty() && next.is_empty() {
                return Some(i);
            }
        }

        None
    }

    pub fn merge_empty_blocks(mut memory: Memory) -> Memory {
        if let Some(i) = find_first_non_empty_merged(&memory) {
            // Merge with right one
            let next_block = memory.remove(i + 1);

            if memory[i].is_empty() {
                if next_block.is_empty() {
                    memory[i].size += next_block.size;
                }
            }

            merge_empty_blocks(memory)
        } else {
            memory
        }
    }

    pub fn calculate_checksum_2(memory: &Memory) -> i64 {
        let mut global_index = 0i64;
        memory
            .iter()
            .map(|b| {
                if let Some(id) = b.id {
                    let mut loc_sum = 0;
                    for _i in 0..b.size {
                        loc_sum += global_index * id;
                        global_index += 1 as i64;
                    }
                    loc_sum
                } else {
                    global_index += b.size as i64;
                    0
                }
            })
            .sum()
    }

    pub fn print_memory(memory: &Memory) {
        let mut output = String::new();

        for block in memory.iter() {
            if let Some(id) = block.id {
                for _i in 0..block.size {
                    if id < 10 {
                        output.push_str(&id.to_string());
                    } else {
                        output.push_str(&format!("({id})"));
                    }
                }
            } else {
                for _i in 0..block.size {
                    output.push_str(".");
                }
            }
        }

        println!("{output}");
    }
}

pub fn part_2(parsed_parts: &ParsedParts) -> i64 {
    let mut memory = parsed_parts.part_2.clone();

    let non_empty_block_len = memory.iter().filter(|block| !block.is_empty()).count();

    let mut viewed_blocks = HashSet::new();

    loop {
        for (i, &file_block) in memory.iter().enumerate().rev() {
            let (id, size) = { (file_block.id, file_block.size) };

            if let Some(id) = id {
                if viewed_blocks.contains(&id) {
                    continue;
                }

                viewed_blocks.insert(id);

                let mut added_block = false;
                let mut new_memory = memory.clone();
                if let Some(empty_index) = part_02::is_space_available(&memory, size) {
                    if empty_index < i {
                        // Do the replacement
                        let empty_block = memory[empty_index];
                        new_memory[i] = part_02::Block::new_empty(size);
                        added_block = true;

                        let empty_block_size = empty_block.size;

                        // Replace / insert the block into the empty space
                        if empty_block_size == size {
                            new_memory[empty_index] = part_02::Block::new(size, Some(id));
                        } else {
                            let new_empty_block =
                                part_02::Block::new_empty(empty_block_size - size);

                            new_memory[empty_index] = part_02::Block::new(size, Some(id));
                            new_memory.insert(empty_index + 1, new_empty_block);
                        }
                    }
                }

                if added_block {
                    // Merge contiguous empty blocks
                    let new_memory = part_02::merge_empty_blocks(new_memory);
                    memory = new_memory;
                    break;
                }
            }
        }

        if viewed_blocks.len() == non_empty_block_len {
            break;
        }
    }

    part_02::calculate_checksum_2(&memory)
}
