//! Day N: PROBLEM NAME, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/N>
use std::collections::VecDeque;
use std::io::Read;

use anyhow::{ensure, Context};
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

    // Part 1: Sum of version values of all packets
    let p1_answer = packet
        .reduce(&|subpacket, children| subpacket.version as u64 + children.iter().sum::<u64>());
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: TODO
    let p2_answer = 0;
    println!("Part 2 answer: {}", p2_answer);
}

/// Alias for bit type (can either be 0 or 1)
type Bit = u8;

/// Wrapper over program input to provide the stream as an iterator
struct InputStream<R: Read> {
    source: std::io::Bytes<R>,
    buffer: VecDeque<Bit>,
}

impl<R: Read> InputStream<R> {
    /// Creates a new input stream from [`std::io::Read`] object
    fn new(reader: R) -> Self {
        InputStream {
            source: reader.bytes(),
            buffer: VecDeque::with_capacity(4),
        }
    }

    /// Fetches the next few bits from the stream and returns as an array.
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
            let bits = match bits_from_hex(c) {
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

impl Packet {
    /// Parses the packet by consuming from the [`InputStream`].
    /// If successful, this method returns the number of bits read from the stream
    /// as well as the packet object itself.
    fn from_stream<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<(usize, Self)> {
        let version = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let type_id = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let (nbits_read, payload) = match type_id {
            4 => Payload::parse_literal(stream)?,
            _ => Payload::parse_ops(stream)?,
        };
        let packet = Packet {
            version,
            type_id,
            payload,
        };
        Ok((nbits_read + 6, packet))
    }

    /// Reduces the packet tree structure into a single value.
    /// The reducer function (`func`) must compute the reduced value for this packet
    /// based on the following two input arguments:
    /// -  The packet itself, and
    /// -  The slice of reduced values from each subpacket.
    fn reduce<T, F>(&self, func: &F) -> T
    where
        F: Fn(&Self, &[T]) -> T,
    {
        let children: Vec<_> = match &self.payload {
            Payload::Literal(_) => Vec::new(),
            Payload::Operation(subpackets) => subpackets.iter().map(|sp| sp.reduce(func)).collect(),
        };
        func(self, children.as_slice())
    }
}

/// Payload of the [`Packet`]
#[derive(Debug, Clone)]
enum Payload {
    /// Payload of the [`Packet`] with `type_id == 4`, containing the literal value
    Literal(u64),
    /// Payload of the [`Packet`] with `type_id != 4`, containing sequence of sub-packets
    Operation(Vec<Packet>),
}

impl Payload {
    /// Parses [`Payload::Literal`] by consuming the next few bits from the stream.
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

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream.
    /// This method dispatches to subroutine depending on the length type ID being read next.
    fn parse_ops<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<(usize, Payload)> {
        let [length_type_id] = stream.fetch()?;
        let (nbits_read, payload) = match length_type_id {
            0 => Payload::parse_ops_by_bit_length(stream)?,
            1 => Payload::parse_ops_by_packet_count(stream)?,
            _ => unreachable!(),
        };
        Ok((nbits_read + 1, payload))
    }

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream,
    /// already knowing that the length type ID previously read was 0.
    /// Hence, the next 15 bits indicate the total length in bits of sub-packets, etc.
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
        ensure!(
            nbits_read == target_length,
            "too many bits read while parsing subpackets: {} > {}",
            nbits_read,
            target_length
        );
        Ok((nbits_read + 15, Payload::Operation(subpackets)))
    }

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream,
    /// already knowing that the length type ID previously read was 1.
    /// Hence, the next 11 bits indicate the number of sub-packets.
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
        Ok((nbits_read + 11, Payload::Operation(subpackets)))
    }
}

/// Converts a hexadecimal character into an array of four bits in MSB-first order.
/// Each bit in the output array is represented by `0` or `1`.
fn bits_from_hex(c: char) -> anyhow::Result<[Bit; 4]> {
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

/// Converts a sequence of bits arranged in MSB-first order into an integer.
fn decimal_from_bits<T>(bits: &[Bit]) -> T
where
    T: PrimInt,
{
    bits.iter().fold(T::zero(), |acc, &bit| {
        T::from(2).unwrap() * acc + T::from(bit).unwrap()
    })
}
