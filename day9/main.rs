use std::{cmp, fmt};
use std::convert::TryFrom;

/// Represents a block of memory on a disk.
/// A block has an ID (which groups blocks together), the number of repetitions
/// in memory, and a gap (empty space) that follows it.
#[derive(Debug, Clone, Copy)]
struct Block {
    /// The ID of the block. Blocks with the same ID are grouped together.
    id: usize,
    /// The number of times this block is repeated in memory.
    repetitions: usize,
    /// The gap (empty space) following this block.
    gap: usize,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: String = self.id.to_string().repeat(self.repetitions);
        let gap: String = ".".repeat(self.gap);
        write!(f, "{}{}", content, gap)
    }
}

/// Represents a disk containing a collection of memory blocks.
/// A disk manages multiple blocks of memory.
#[derive(Debug, Clone)]
struct Disk {
    /// A vector containing all the blocks in this disk.
    blocks: Vec<Block>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: String = self.blocks.iter().map(|block| block.to_string()).collect();
        write!(f, "{}", display)
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
        value
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .enumerate()
            .map(|(id, chunk)| {
                let (reps_char, gaps_char) = match chunk {
                    [reps_char, gaps_char] => (reps_char, gaps_char),
                    [reps_char] => (reps_char, &'0'),
                    _ => return Err(DiskParseError::InvalidChunk),
                };

                let repetitions = reps_char
                    .to_digit(10)
                    .ok_or(DiskParseError::InvalidCharacter(*reps_char, id * 2))?
                    as usize;
                let gap = gaps_char
                    .to_digit(10)
                    .ok_or(DiskParseError::InvalidCharacter(*gaps_char, id * 2 + 1))?
                    as usize;

                Ok(Block { id, repetitions, gap })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|blocks| Disk { blocks })
    }
}

impl Disk {
    /// Condenses the disk by removing all gaps between blocks.
    ///
    /// This method iterates through the blocks of the disk and shifts the memory
    /// contents to the left, removing all gaps. The resulting disk has no gaps,
    /// and the memory layout is continuous.
    ///
    /// # Returns
    /// - `Ok(Disk)` if condensation succeeds, containing the new, condensed disk.
    /// - `Err(CondenseError)` if condensation fails due to recoverable issues.
    ///
    /// # Errors
    /// - `CondenseError::EmptyDisk` if the disk contains no blocks.
    /// - `CondenseError::InconsistencyDetected` if an unexpected issue arises during condensation.
    pub fn condense(&self) -> Disk {
        let mut backlog = self.blocks.iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, block)| (block.repetitions > 0).then_some((idx, *block)));
        let mut current = backlog.next();
        let blocks = self.blocks.iter().enumerate().filter_map(|(idx, &(mut block))| {
            if let Some((back_idx, transferring)) = current {
                if back_idx == idx { return Some(vec![Block { id: transferring.id, repetitions: transferring.repetitions, gap: 0 }]); }
                if back_idx < idx { return None; }
            } else {
                return None;
            }
            if block.gap == 0 { return Some(vec![block]); }
            let mut transferred = Vec::with_capacity(block.gap);
            transferred.push(Block { id: block.id, repetitions: block.repetitions, gap: 0 });
            while let Some((back_idx, transferring)) = (block.gap > 0).then_some(current.as_mut()).flatten() {
                if *back_idx <= idx { current = None; break; }
                let transfer_size = cmp::min(block.gap, transferring.repetitions);
                block.gap -= transfer_size;
                transferring.repetitions -= transfer_size;
                transferred.push(Block { id: transferring.id, repetitions: transfer_size, gap: 0 });
                if transferring.repetitions == 0 { current = backlog.next(); }
            }
            transferred.last_mut().unwrap().gap = block.gap;
            Some(transferred)
        }).flatten().collect();
        Self { blocks }
    }

    pub fn condense_blocks(&self) -> Disk {
        let mut blocks = self.blocks.iter().enumerate().map(|(idx, block)| (idx as i32, *block)).collect::<Vec<_>>();
        for (fragmented_id, fragmenting) in self.blocks.iter().enumerate().rev() {
            if fragmenting.repetitions == 0 { continue }
            let existing = blocks.iter_mut().enumerate().find(|(_, (_, block))| block.gap >= fragmenting.repetitions);
            let (idx, (_, block)) = if let Some((idx, block)) = existing { (idx, block) } else { continue };
            let gap = block.gap - fragmenting.repetitions;
            blocks.insert(idx, (-1, Block { id: fragmenting.id, repetitions: fragmenting.repetitions, gap }));
            let fragmented_position = blocks.iter().position(|(block_id, _)| *block_id == fragmented_id as i32).unwrap();
            blocks.remove(fragmented_position);
        }
        Self { blocks: blocks.into_iter().map(|(_, block)| block).collect() }
    }

    /// Gets the checksum of the disk where each block's position is multipled by its ID and summed.
    fn get_checksum(&self) -> usize {
        self.blocks.iter()
            .flat_map(|block| vec![block.id; block.repetitions])
            .enumerate()
            .map(|(idx, id)| idx * id)
            .sum()
    }
}

/// Gets the checksum of the disk
fn part1_solution(input: &str) -> Result<usize, DiskParseError> {
    Ok(Disk::try_from(input)?.condense().get_checksum())
}

/// Gets the checksum of the disk
fn part2_solution(input: &str) -> Result<usize, DiskParseError> {
    println!("{}", Disk::try_from(input)?.condense_blocks());
    Ok(Disk::try_from(input)?.condense_blocks().get_checksum())
}


/// Entry point
pub fn main() {
    let example = "2333133121414131402";
    let input = include_str!("day9.txt");

    println!("Part 1 Solution on Example: {:#?}", part1_solution(example));
	println!("Part 1 Solution on Input: {:#?}", part1_solution(input));

	println!("Part 2 Solution on Example: {:#?}", part2_solution(example));
	// println!("Part 2 Solution on Input: {:#?}", part2_solution(input));
}
