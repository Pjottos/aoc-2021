use std::{
    fmt::{self, Display, Write},
    ops::{Index, IndexMut},
};

use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(12)
        .extract(|text| {
            let mut node_lut = NodeLut::new();

            for line in text.lines() {
                let mut parts = line.split('-');
                let key = NodeKey::from_text(parts.next().unwrap());
                let target = NodeKey::from_text(parts.next().unwrap());

                node_lut[key].push(target);
                node_lut[target].push(key);
            }

            node_lut
        })
        .run_part(1, |node_lut| {
            let mut key_stack = vec![NodeKey::START];
            let mut idx_stack = vec![0];
            let mut path_count = 0;

            while let Some((key, child_idx)) = key_stack
                .pop()
                .and_then(|key| idx_stack.pop().map(|idx| (key, idx)))
            {
                if let Some(&next_key) = node_lut[key].get(child_idx) {
                    key_stack.push(key);
                    idx_stack.push(child_idx + 1);

                    if next_key == NodeKey::END {
                        path_count += 1;
                    } else if !next_key.is_small() || key_stack.iter().all(|&k| k != next_key) {
                        key_stack.push(next_key);
                        idx_stack.push(0);
                    }
                }
            }

            path_count
        })
        .run_part(2, |_node_lut| {});
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NodeKey(u16);

impl NodeKey {
    const ONCE_FLAG: u16 = 1 << 10;
    const START: Self = Self((!0 >> 6) | Self::ONCE_FLAG);
    const END: Self = Self(!0 >> 6);

    fn from_text(text: &str) -> Self {
        match text {
            "start" => Self::START,
            "end" => Self::END,
            dynamic => {
                <[u8; 2]>::try_from(dynamic.as_bytes())
                    .ok()
                    .and_then(|bytes| {
                        let is_big = bytes.iter().all(|&b| b.wrapping_sub(b'A') < 26);
                        let is_small = bytes.iter().all(|&b| b.wrapping_sub(b'a') < 26);

                        (is_big || is_small).then(|| {
                            // Mask unnecessary bits
                            let mut res =
                                u16::from(bytes[0] & !0xE0) | (u16::from(bytes[1] & !0xE0) << 5);

                            if is_small {
                                res |= Self::ONCE_FLAG;
                            }

                            Self(res)
                        })
                    })
                    .expect("invalid node key")
            }
        }
    }

    #[inline]
    fn is_small(self) -> bool {
        self.0 & Self::ONCE_FLAG != 0
    }
}

impl Display for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::START => f.write_str("start"),
            Self::END => f.write_str("end"),
            _ => {
                let base = if self.is_small() { b'a' } else { b'A' };
                f.write_char(char::from(base + (self.0 as u8 & 0x1F)))?;
                f.write_char(char::from(base + ((self.0 >> 5) as u8 & 0x1F)))
            }
        }
    }
}

struct NodeLut {
    values: Box<[Vec<NodeKey>]>,
}

impl NodeLut {
    const LUT_SIZE: usize = (NodeKey::ONCE_FLAG as usize) << 1;

    fn new() -> Self {
        Self {
            values: (0..Self::LUT_SIZE).map(|_| vec![]).collect(),
        }
    }
}

impl Index<NodeKey> for NodeLut {
    type Output = Vec<NodeKey>;

    fn index(&self, index: NodeKey) -> &Self::Output {
        &self.values[index.0 as usize]
    }
}

impl IndexMut<NodeKey> for NodeLut {
    fn index_mut(&mut self, index: NodeKey) -> &mut Self::Output {
        &mut self.values[index.0 as usize]
    }
}
