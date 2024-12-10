use std::{cmp, fmt};
use std::convert::TryFrom;
use std::fmt::Display;

/// Represents a block of memory on a disk.
/// A block has an ID (which groups blocks together), size, and offset.
#[derive(Debug, Clone, PartialEq)]
struct Block {
    /// The ID of the block. Blocks with the same ID are grouped together.
    id: usize,
    /// The size of this block in memory.
    size: usize,
    /// The offset of this block in memory.
    offset: usize,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id.to_string().repeat(self.size))
    }
}

impl Block {
    /// Gets the checksum of the block where each bit's position is multipled by its ID and summed.
    fn get_checksum(&self) -> usize {
        (self.offset..self.offset + self.size).map(|pos| pos * self.id).sum()
    }
}

/// Represents a disk containing a collection of memory blocks.
/// A disk manages multiple blocks of memory.
#[derive(Debug, Clone)]
struct Disk {
    /// A vector containing all the blocks in this disk.
    blocks: Vec<Block>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let back = if let Some(back) = self.blocks.last() { back } else { return Ok(()) };
        let mut disk = vec![String::from("."); back.offset + back.size];
        for block in &self.blocks {
            disk[block.offset..block.offset + block.size].fill(block.id.to_string());
        }
        f.write_str(&disk.join(""))
    }
}

/// Custom error type for parsing a Disk from a string.
#[derive(Debug)]
enum DiskParseError {
    /// Error when a character in the input string is not a valid digit.
    InvalidCharacter(char, usize),
    /// Error when a chunk does not have exactly two elements.
    InvalidChunk,
}

impl fmt::Display for DiskParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskParseError::InvalidCharacter(c, idx) => write!(f, "Invalid character '{}' at position {}.", c, idx),
            DiskParseError::InvalidChunk => write!(f, "Input string contains an invalid chunk. Each chunk must consist of exactly two characters."),
        }
    }
}

impl std::error::Error for DiskParseError {}

impl TryFrom<&str> for Disk {
    type Error = DiskParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let blocks = value.chars().collect::<Vec<_>>()
            .chunks(2).enumerate().map(|(id, chunk)| {
                let (size_char, gaps_char) = match chunk {
                    [size_char, gaps_char] => (size_char, gaps_char),
                    [size_char] => (size_char, &'0'),
                    _ => return Err(DiskParseError::InvalidChunk),
                };

                let size = size_char.to_digit(10).ok_or(DiskParseError::InvalidCharacter(*size_char, id * 2))? as usize;
                let gap = gaps_char.to_digit(10).ok_or(DiskParseError::InvalidCharacter(*gaps_char, id * 2 + 1))? as usize;
                Ok((id, size, gap))
            }).collect::<Result<Vec<_>, _>>()?
            .iter().scan(0usize, |offset, &(id, size, gap)| {
                let block = Some(Block { id, size, offset: *offset });
                *offset += size + gap;
                block
            }).collect();
        Ok(Self { blocks })
    }
}

impl Disk {
    /// Condenses the disk by removing all gaps between blocks.
    ///
    /// This method iterates through the blocks of the disk and shifts the memory
    /// contents to the left, removing all gaps. The resulting disk has no gaps,
    /// and the memory layout is continuous.
    pub fn condense(&self) -> Disk {
        let mut blocks = self.blocks.clone();
        for mut block in blocks.clone().into_iter().rev() {
            let removal_block = block.clone();
            while block.size > 0 {
                let Some((idx, offset, size)) = blocks.windows(2).enumerate().find_map(|(idx, window)| {
                    let [current, next] = window else { return None };
                    if current.offset + current.size >= block.offset { return None };
                    let gap = next.offset - (current.offset + current.size);
                    (gap > 0).then_some((idx + 1, current.offset + current.size, cmp::min(gap, block.size)))
                }) else { break; };
                block.size -= size;
                blocks.insert(idx, Block { id: block.id, size, offset })
            }
            let existing_idx = blocks.iter().position(|x| *x == removal_block).unwrap();
            if block.size == 0 { blocks.remove(existing_idx); }
            else { blocks[existing_idx].size = block.size; }
        }
        Self { blocks }
    }

    /// Condenses only full blocks at a time. 
    ///
    /// This method moves full blocks to fill gaps without fragmenting the blocks themselves.
    /// If a block cannot be moved in its entirety due to insufficient space, it will remain in its current position.
    pub fn condense_blocks(&self) -> Disk {
        let mut blocks = self.blocks.clone();
        for block in blocks.clone().into_iter().rev() {
            let Some((idx, offset)) = blocks.windows(2).enumerate().find_map(|(idx, window)| {
                let [current, next] = window else { return None };
                if current.offset + current.size >= block.offset { return None };
                (next.offset - (current.offset + current.size) >= block.size)
                    .then_some((idx + 1, current.offset + current.size))
            }) else { continue };
            let removal_idx = blocks.iter().position(|x| *x == block).unwrap();
            let mut block = blocks.remove(removal_idx);
            block.offset = offset;
            blocks.insert(idx, block);
        }
        Self { blocks }
    }

    /// Gets the checksum of the disk where each block's position is multipled by its ID and summed.
    fn get_checksum(&self) -> usize {
        self.blocks.iter().map(|block| block.get_checksum()).sum()
    }
}

/// Gets the checksum of the disk
fn part1_solution(input: &str) -> Result<usize, DiskParseError> {
    Ok(Disk::try_from(input)?.condense().get_checksum())
}

/// Gets the checksum of the disk
fn part2_solution(input: &str) -> Result<usize, DiskParseError> {
    Ok(Disk::try_from(input)?.condense_blocks().get_checksum())
}


/// Entry point
pub fn main() {
    let example = "2333133121414131402";
    let input = include_str!("day9.txt");

    println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

	println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
