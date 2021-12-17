use std::ops::{Index, IndexMut};

use crate::aoc_base::Day;

pub struct Day16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PacketKey(u32);
impl PacketKey {
    const NULL: PacketKey = PacketKey(!0);
    pub fn is_null(self) -> bool { self == PacketKey::NULL }
    pub fn is_not_null(self) -> bool { self != PacketKey::NULL }
}

#[derive(Debug, Clone)]
pub struct PacketStore {
    data: Vec<PacketData>
}

impl Index<PacketKey> for PacketStore {
    type Output = PacketData;

    fn index(&self, index: PacketKey) -> &Self::Output {
        &self.data[index.0 as usize]
    }
}

impl IndexMut<PacketKey> for PacketStore {
    fn index_mut(&mut self, index: PacketKey) -> &mut Self::Output {
        &mut self.data[index.0 as usize]
    }
}

impl PacketStore {
    fn checksum(&self) -> u32 {
        self.data.iter().map(|data| data.version as u32).sum()
    }

    fn eval(&self, packet: PacketKey) -> Word {
        match &self[packet].payload {
            Payload::Literal(x) => *x,
            Payload::Subpackets(sub) => {
                match self[packet].type_id {
                    0 => sub.iter().map(|&x| self.eval(x)).sum(),
                    1 => sub.iter().map(|&x| self.eval(x)).product(),
                    2 => sub.iter().map(|&x| self.eval(x)).min().unwrap(),
                    3 => sub.iter().map(|&x| self.eval(x)).max().unwrap(),
                    5 => (self.eval(sub[0]) > self.eval(sub[1])) as Word,
                    6 => (self.eval(sub[0]) < self.eval(sub[1])) as Word,
                    7 => (self.eval(sub[0]) == self.eval(sub[1])) as Word,
                    _ => panic!("unknown operator ID")
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketData {
    version: u8,
    type_id: u8,
    payload: Payload
}

pub type Word = u64;

#[derive(Debug, Clone)]
pub enum Payload {
    Literal(Word), Subpackets(Vec<PacketKey>)
}

struct PacketDecoder {
    store: PacketStore,
    bytes: Vec<u8>,
    bit_pt: usize
}

impl PacketDecoder {
    fn new(input: &str) -> Self {
        let mut bytes = Vec::<u8>::new();
        for (i, ch) in input.bytes().enumerate() {
            let nibble = match ch {
                b'0'..=b'9' => { ch - b'0' },
                b'A'..=b'F' => { ch - b'A' + 10 },
                b'a'..=b'f' => { ch - b'a' + 10 },
                _ => panic!("invalid hexdigit")
            };
            if i & 1 == 0 {
                bytes.push(nibble << 4);
            } else {
                *bytes.last_mut().unwrap() |= nibble;
            }
        }

        Self {
            store: PacketStore { data: Vec::new() },
            bytes,
            bit_pt: 0
        }
    }

    fn read_bit(&mut self) -> bool {
        let byte_index = self.bit_pt / 8;
        let bit_index = 7 - self.bit_pt % 8;
        self.bit_pt += 1;
        self.bytes[byte_index] >> bit_index & 1 == 1
    }

    fn read_bits(&mut self, num_bits: usize) -> Word {
        let mut res: Word = 0;
        for _ in 0..num_bits {
            res *= 2;
            res |= self.read_bit() as Word;
        }
        res
    }

    // return value = (packet key, bits read)
    fn decode(&mut self) -> (PacketKey, usize) {
        let packet_key = PacketKey(self.store.data.len() as _);
        let start = self.bit_pt;

        let version = self.read_bits(3) as u8;
        let type_id = self.read_bits(3) as u8;

        if type_id == 4 {
            let mut literal: Word = 0;
            loop {
                let cont = self.read_bit();
                literal *= 1 << 4;
                literal |= self.read_bits(4) as Word;
                if !cont { break; }
            }
            self.store.data.push(PacketData { version, type_id, payload: Payload::Literal(literal) });
        } else {
            self.store.data.push(PacketData { version, type_id, payload: Payload::Subpackets(vec![]) });

            let len_type = self.read_bit() as u8;
            if len_type == 0 {
                let mut len_bits = self.read_bits(15) as usize;
                let mut subpackets = vec![];
                while len_bits > 0 {
                    let (sub, used) = self.decode();
                    subpackets.push(sub);
                    len_bits = len_bits.checked_sub(used).expect("packets not aligned");
                }
                self.store[packet_key].payload = Payload::Subpackets(subpackets);
            } else {
                let subpackets = (0..self.read_bits(11))
                    .map(|_| self.decode().0)
                    .collect();
                self.store[packet_key].payload = Payload::Subpackets(subpackets);
            }
        }

        (packet_key, self.bit_pt - start)
    }
}

impl Day for Day16 {
    type Parsed = PacketStore;

    type Output1 = u32;

    type Output2 = Word;

    fn num() -> usize {
        16
    }

    fn title() -> &'static str {
        "Packet Decoder"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut decoder = PacketDecoder::new(input);
        decoder.decode();
        decoder.store
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        data.checksum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        data.eval(PacketKey(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day16::test(InputSource::gmail(16), Some(929), Some(911945136934))
    }
}