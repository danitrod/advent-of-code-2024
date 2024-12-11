use std::{
    fmt::{self, Error, Formatter},
    fs, iter,
};

fn main() {
    let input = fs::read_to_string("inputs/day09.txt")
        .unwrap()
        .trim()
        .to_owned();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("Day 09 - Part 1");

    let mut blocks = Vec::<Option<isize>>::new();
    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                blocks.extend(
                    iter::repeat((i / 2) as isize)
                        .take(c.to_digit(10).unwrap() as usize)
                        .map(Some),
                );
            }
            _ => {
                blocks.extend(iter::repeat(None).take(c.to_digit(10).unwrap() as usize));
            }
        }
    }

    let mut left_pointer = 0;
    let mut right_pointer = blocks.len() - 1;
    while left_pointer < right_pointer {
        if blocks[left_pointer].is_none() {
            while blocks[right_pointer].is_none() {
                right_pointer -= 1;
            }

            blocks[left_pointer] = blocks[right_pointer];
            blocks[right_pointer] = None;
            right_pointer -= 1;
        }

        left_pointer += 1;
    }

    let mut checksum = 0;
    left_pointer = 0;
    while let Some(block_id) = blocks[left_pointer] {
        checksum += left_pointer * block_id as usize;
        left_pointer += 1;
    }

    println!("Total checksum: {}", checksum);
}

#[derive(Clone)]
struct Block {
    contents: Vec<Option<isize>>,
}

impl Block {
    fn new(id: isize, size: usize) -> Self {
        Block {
            contents: iter::repeat(Some(id)).take(size).collect(),
        }
    }

    fn new_empty(size: usize) -> Self {
        Block {
            contents: iter::repeat(None).take(size).collect(),
        }
    }

    fn free_space(&self) -> usize {
        self.contents.iter().filter(|c| c.is_none()).count()
    }

    fn size(&self) -> usize {
        self.contents.iter().filter(|c| c.is_some()).count()
    }

    fn add_block(&mut self, block: Block) {
        for c in block.contents.iter().filter(|c| c.is_some()) {
            for i in 0..self.contents.len() {
                if self.contents[i].is_none() {
                    self.contents[i] = *c;
                    break;
                }
            }
        }
    }

    fn clear(&mut self) {
        self.contents = iter::repeat(None).take(self.contents.len()).collect();
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for c in self.contents.iter() {
            match c {
                Some(id) => write!(f, "{}", id)?,
                None => write!(f, ".")?,
            }
        }

        Ok(())
    }
}

fn part2(input: &str) {
    println!("Day 09 - Part 2");

    let mut blocks = Vec::<Block>::new();
    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                blocks.push(Block::new(
                    (i / 2) as isize,
                    c.to_digit(10).unwrap() as usize,
                ));
            }
            _ => {
                blocks.push(Block::new_empty(c.to_digit(10).unwrap() as usize));
            }
        }
    }

    let mut right_pointer = blocks.len() - 1;
    while right_pointer > 0 {
        let mut left_pointer = 0;
        while blocks[right_pointer].size() > blocks[left_pointer].free_space() {
            left_pointer += 1;
        }

        if left_pointer < right_pointer {
            let block_to_move = blocks[right_pointer].clone();
            blocks[left_pointer].add_block(block_to_move);
            blocks[right_pointer].clear();
        }

        right_pointer -= 1;
    }

    let mut checksum = 0;
    let mut position = 0;
    for block in blocks {
        for c in block.contents.iter() {
            if c.is_some() {
                checksum += position * c.unwrap() as usize;
            }
            position += 1;
        }
    }
    println!("Total checksum: {}", checksum);
}
