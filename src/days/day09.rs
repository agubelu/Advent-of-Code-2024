use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::cmp::Ordering::*;

///////////////////////////////////////////////////////////////////////////////

const SPACE: i64 = -1;

#[derive(Copy, Clone, Debug)]
struct Block {
    size: i64,
    ident: i64, // Spaces have ident = -1
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09.txt").unwrap();
    let layout = parse_layout(&input);

    let sol1 = compact_v1(&layout);
    let sol2 = compact_v2(layout);

    (Solution::from(sol1), Solution::from(sol2))
}

fn compact_v1(layout: &[Block]) -> i64 {
    let mut layout = layout.to_owned();
    let mut space_ptr = 1;
    let mut file_ptr = layout.len() - 1;

    while file_ptr > space_ptr {
        move_block(&mut layout, file_ptr, space_ptr);
        while layout[space_ptr].ident != SPACE { space_ptr += 1 }
        while layout[file_ptr].ident == SPACE { file_ptr -= 1 }
    }

    checksum(&layout)
}

fn compact_v2(mut layout: Vec<Block>) -> i64 {
    let mut file_ptr = layout.len();
    let mut target_ident = layout.len() as i64 / 2;

    while file_ptr > 0 {
        file_ptr -= 1;

        // Skip if this is a space or have already moved this block
        let block_to_move = layout[file_ptr];
        if block_to_move.ident != target_ident { continue }
        target_ident -= 1;

        // Find the first space where this block can fit
        let space = (0..file_ptr).find(|&i| layout[i].ident == SPACE && layout[i].size >= block_to_move.size);
        if let Some(space_ix) = space {
            move_block(&mut layout, file_ptr, space_ix);

            if layout[space_ix + 1].ident == SPACE {
                file_ptr += 1; // A new space has been added
            }
        }
    }

    checksum(&layout)
}

fn checksum(layout: &[Block]) -> i64 {
    let (mut sum, mut pos) = (0, 0);
    for b in layout {
        if b.ident != SPACE {
            let (start, end) = (pos, pos + b.size - 1);
            sum += (end - start + 1) * (end + start) / 2 * b.ident; // Sum of the interval [start, end] * ident
        }
        pos += b.size;
    }
    sum
}

fn move_block(layout: &mut Vec<Block>, source_ix: usize, target_ix: usize) {
    // Moves the block in source_ix into the space in target_ix
    let file_size = layout[source_ix].size;
    let space_size = layout[target_ix].size;

    match space_size.cmp(&file_size) {
        Equal => layout.swap(target_ix, source_ix), // Block fits perfectly in the new space
        Greater => {
            // The block partially fills the space; a new space is added
            let leftover_space = space_size - file_size;
            layout[target_ix] = layout[source_ix];
            layout[source_ix] = Block{ size: file_size, ident: SPACE };
            layout.insert(target_ix + 1, Block{ size: leftover_space, ident: SPACE });
        },
        Less => {
            // The block is bigger than the space; a leftover remains in the block
            // This can only happen in part 1, so no extra space added because it doesn't matter.
            layout[target_ix] = Block { ident: layout[source_ix].ident, size: space_size };
            layout[source_ix].size = file_size - space_size;
        },
    };
}

fn parse_layout(input: &str) -> Vec<Block> {
    input.trim_end().char_indices()
         .map(|(i, ch)| {
             let size = ch.to_digit(10).unwrap() as i64;
             let ident = if i % 2 == 0 { i as i64 / 2 } else { SPACE };
             Block { size, ident }
         }).collect()
}
