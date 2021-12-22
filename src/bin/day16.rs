//! Day N: PROBLEM NAME, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/N>
use std::collections::VecDeque;
use std::io::{Bytes, Read};

use anyhow::Context;
use num::PrimInt;

use aoc2021::argparser;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_raw_reader().expect("cannot open file");
    let mut input_stream = InputStream::new(input_reader);

    // Parses the packet from the input stream
    let (_, packet) = Packet::from_stream(&mut input_stream).expect("cannot parse packet");
    eprintln!("{:?}", packet);

    // Part 1: TODO
    let p1_answer = 0;
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Alias for bit type (can either be 0 or 1)
type Bit = u8;

/// Program input data
struct InputStream<R: Read> {
    source: Bytes<R>,
    buffer: VecDeque<Bit>,
}

impl<R: Read> InputStream<R> {
    /// Creates a new input stream object
    fn new(reader: R) -> Self {
        InputStream {
            source: reader.bytes(),
            buffer: VecDeque::with_capacity(4),
        }
    }

    /// Fetches a few bits from the stream.
    fn fetch<const SIZE: usize>(&mut self) -> anyhow::Result<[Bit; SIZE]> {
        let mut target = [0; SIZE];
        for element in target.iter_mut() {
            *element = self.next().context("no more bits to consume")??;
        }
        Ok(target)
    }
}

impl<R: Read> Iterator for InputStream<R> {
    type Item = anyhow::Result<Bit>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            let c = match self.source.next()? {
                Ok(c) => c as char,
                Err(err) => return Some(Err(anyhow::Error::new(err))),
            };
            let bits = match bits_from_hex_char(c) {
                Ok(bits) => bits,
                Err(err) => return Some(Err(err)),
            };
            self.buffer = VecDeque::from(bits);
        }
        self.buffer.pop_front().map(Ok)
    }
}

/// Packet in BITS transmission
#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

/// Payload of the packet, depending on the `type_id`
#[derive(Debug, Clone)]
enum Payload {
    /// Payload for `type_id == 4`
    Literal(u64),
    /// Payload for `type_id` other than `type_id == 4`
    Operation(Vec<Packet>),
}

impl Packet {
    /// Parses the packet from the [`InputStream`].
    /// If successful, this method returns the number of bits read and the packet itself.
    fn from_stream<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<(usize, Self)> {
        let version = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let type_id = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let (nbits_read, payload) = if type_id == 4 {
            Payload::parse_literal(stream)?
        } else {
            Payload::parse_ops(stream)?
        };
        let packet = Packet {
            version,
            type_id,
            payload,
        };
        Ok((nbits_read + 6, packet))
    }
}

impl Payload {
    /// Parses the payload as literal value.
    fn parse_literal<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<(usize, Payload)> {
        let mut bits = Vec::new();
        let mut nbits_read = 0;
        loop {
            let batch: [_; 5] = stream.fetch()?;
            nbits_read += 5;
            bits.extend(&mut batch[1..5].iter());
            if batch[0] == 0 {
                break;
            }
        }
        let payload = Payload::Literal(decimal_from_bits(bits.as_slice()));
        Ok((nbits_read, payload))
    }

    /// Parses the payload as an operation packet.
    /// This method mainly dispatches to subroutine depending on the length type ID.
    fn parse_ops<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<(usize, Payload)> {
        let [length_type_id] = stream.fetch()?;
        let (nbits_read, payload) = match length_type_id {
            0 => Payload::parse_ops_by_bit_length(stream)?,
            1 => Payload::parse_ops_by_packet_count(stream)?,
            _ => unreachable!(),
        };
        Ok((nbits_read + 1, payload))
    }

    /// Parses the operation packet where the length type ID was 0,
    /// i.e. the next 15 bits indicate the total length in bits of sub-packets.
    fn parse_ops_by_bit_length<R: Read>(
        stream: &mut InputStream<R>,
    ) -> anyhow::Result<(usize, Payload)> {
        let target_length: usize = decimal_from_bits(stream.fetch::<15>()?.as_slice());
        let mut nbits_read = 0;
        let mut subpackets = Vec::new();
        while nbits_read < target_length {
            let (nbits_more, subpacket) = Packet::from_stream(stream)?;
            nbits_read += nbits_more;
            subpackets.push(subpacket);
        }
        let payload = Payload::Operation(subpackets);
        Ok((nbits_read + 15, payload))
    }

    /// Parses the operation packet where the length type ID was 1,
    /// i.e. the next 11 bits indicate the number of sub-packets.
    fn parse_ops_by_packet_count<R: Read>(
        stream: &mut InputStream<R>,
    ) -> anyhow::Result<(usize, Payload)> {
        let subpacket_count: usize = decimal_from_bits(stream.fetch::<11>()?.as_slice());
        let mut nbits_read = 0;
        let mut subpackets = Vec::with_capacity(subpacket_count);
        for _ in 0..subpacket_count {
            let (nbits_more, subpacket) = Packet::from_stream(stream)?;
            nbits_read += nbits_more;
            subpackets.push(subpacket);
        }
        let payload = Payload::Operation(subpackets);
        Ok((nbits_read + 11, payload))
    }
}

/// Converts a hexadecimal character into an array of four bits in MSB order,
/// where each bit is represented by `0` or `1` under type `u8`.
fn bits_from_hex_char(c: char) -> anyhow::Result<[Bit; 4]> {
    let decimal = c
        .to_digit(16)
        .map(|d| d as u8)
        .with_context(|| format!("not a hexadecimal character: {:?}", c))?;
    Ok([
        (0b1000 & decimal) / 0b1000,
        (0b0100 & decimal) / 0b0100,
        (0b0010 & decimal) / 0b0010,
        (0b0001 & decimal),
    ])
}

/// Converts a slice of bits in MSB order into an integer.
fn decimal_from_bits<T>(bits: &[Bit]) -> T
where
    T: PrimInt,
{
    bits.iter()
        .fold(T::zero(), |acc, &bit| acc + acc + T::from(bit).unwrap())
}
