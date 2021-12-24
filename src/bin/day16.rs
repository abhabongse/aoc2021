//! Day 16: Packet Decoder, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/16>
use std::collections::VecDeque;
use std::io::Read;

use anyhow::{bail, ensure, Context};
use itertools::Itertools;
use num::PrimInt;

use aoc2021::argparser;
use aoc2021::collect_array::CollectArray;

/// Main program
fn main() {
    let input_src = argparser::InputSrc::from_arg(std::env::args().nth(1).as_deref());
    let input_reader = input_src.get_raw_reader().expect("cannot open file");
    let mut input_stream = InputStream::new(input_reader);

    // Parses the packet from the input stream
    let packet = Packet::from_stream(&mut input_stream).expect("cannot parse packet");

    // Part 1: Sum of version values of all packets
    let p1_answer = packet
        .reduce(&|subpacket, children| subpacket.version as u64 + children.iter().sum::<u64>());
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Evaluate the packet
    let p2_answer = packet.eval().expect("error during evaluation");
    println!("Part 2 answer: {}", p2_answer);
}

/// Alias for bit type (can either be 0 or 1)
type Bit = u8;

/// Wrapper over program input to provide the stream as an iterator
struct InputStream<R: Read> {
    source: std::io::Bytes<R>,
    buffer: VecDeque<Bit>,
    bits_read: usize,
}

impl<R: Read> InputStream<R> {
    /// Creates a new input stream from [`std::io::Read`] object
    fn new(reader: R) -> Self {
        InputStream {
            source: reader.bytes(),
            buffer: VecDeque::with_capacity(4),
            bits_read: 0,
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
        self.bits_read += 1;
        self.buffer.pop_front().map(Ok)
    }
}

/// Packet in BITS transmission
#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    /// Parses the packet by consuming from the [`InputStream`].
    /// If successful, this method returns the number of bits read from the stream
    /// as well as the packet object itself.
    fn from_stream<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<Self> {
        let version = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let type_id = decimal_from_bits(stream.fetch::<3>()?.as_slice());
        let payload = match type_id {
            4 => Payload::parse_literal(stream)?,
            _ => Payload::parse_ops(stream, Operator::new(type_id)?)?,
        };
        Ok(Packet { version, payload })
    }

    /// Evaluates the expression described by the packet.
    ///
    /// # Implementation Note
    /// This method did not utilize [`Packet::reduce`] for two main reasons:
    /// -  [`Packet::reduce`] did not provide short-circuiting,
    ///    especially in cases when fallible result could happen
    /// -  This method reflects the original purpose of the existence of the [`Packet`]
    fn eval(&self) -> anyhow::Result<u64> {
        match &self.payload {
            Payload::Literal(value) => Ok(*value),
            Payload::Operation(op, children) => {
                let children: Vec<_> = children
                    .iter()
                    .map(|subpacket| subpacket.eval())
                    .collect::<anyhow::Result<_>>()?;
                op.apply(children.as_slice())
            }
        }
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
            Payload::Operation(_, children) => children.iter().map(|sp| sp.reduce(func)).collect(),
        };
        func(self, children.as_slice())
    }
}

/// Payload of the [`Packet`]
#[derive(Debug, Clone)]
enum Payload {
    /// Payload of [`Packet`] with `type_id == 4` containing the literal value
    Literal(u64),
    /// Payload of [`Packet`] containing an operation on subpackets
    Operation(Operator, Vec<Packet>),
}

impl Payload {
    /// Parses [`Payload::Literal`] by consuming the next few bits from the stream.
    fn parse_literal<R: Read>(stream: &mut InputStream<R>) -> anyhow::Result<Payload> {
        let mut bits = Vec::new();
        loop {
            let batch: [_; 5] = stream.fetch()?;
            bits.extend(&mut batch[1..5].iter());
            if batch[0] == 0 {
                break;
            }
        }
        let value = decimal_from_bits(bits.as_slice());
        Ok(Payload::Literal(value))
    }

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream.
    /// This method dispatches to subroutine depending on the length type ID being read next.
    fn parse_ops<R: Read>(stream: &mut InputStream<R>, op: Operator) -> anyhow::Result<Payload> {
        let [length_type_id] = stream.fetch()?;
        let children = match length_type_id {
            0 => Payload::parse_children_by_bit_length(stream)?,
            1 => Payload::parse_children_by_packet_count(stream)?,
            _ => unreachable!(),
        };
        Ok(Payload::Operation(op, children))
    }

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream,
    /// already knowing that the length type ID previously read was 0.
    /// Hence, the next 15 bits indicate the total length in bits of sub-packets, etc.
    fn parse_children_by_bit_length<R: Read>(
        stream: &mut InputStream<R>,
    ) -> anyhow::Result<Vec<Packet>> {
        let target_length: usize = decimal_from_bits(stream.fetch::<15>()?.as_slice());
        let count_start = stream.bits_read;
        let mut children = Vec::new();
        while stream.bits_read < count_start + target_length {
            children.push(Packet::from_stream(stream)?);
        }
        ensure!(
            stream.bits_read == count_start + target_length,
            "too many bits read while parsing subpackets: {} > {}",
            stream.bits_read - count_start,
            target_length
        );
        Ok(children)
    }

    /// Parses [`Payload::Operation`] by consuming the next few bits from the stream,
    /// already knowing that the length type ID previously read was 1.
    /// Hence, the next 11 bits indicate the number of sub-packets.
    fn parse_children_by_packet_count<R: Read>(
        stream: &mut InputStream<R>,
    ) -> anyhow::Result<Vec<Packet>> {
        let subpacket_count: usize = decimal_from_bits(stream.fetch::<11>()?.as_slice());
        (0..subpacket_count)
            .map(|_| Packet::from_stream(stream))
            .collect()
    }
}

/// Packet in BITS transmission
#[derive(Debug, Clone, Copy)]
enum Operator {
    /// Sum operator when packet's `type_id == 0`
    Sum,
    /// Product operator when packet's `type_id == 1`
    Product,
    /// Minimum operator when packet's `type_id == 2`
    Minimum,
    /// Maximum operator when packet's `type_id == 3`
    Maximum,
    /// Operator which returns `1` if the result of the first subpacket is greater than the second;
    /// otherwise returns `0`. This indicates that packet's `type_id == 5`.
    GreaterThan,
    /// Operator which returns `1` if the result of the first subpacket is less than the second;
    /// otherwise returns `0`. This indicates that packet's `type_id == 6`.
    LessThan,
    /// Operator which returns `1` if the result of the first subpacket is equal to the second;
    /// otherwise returns `0`. This indicates that packet's `type_id == 7`.
    EqualTo,
}

impl Operator {
    fn new(type_id: u8) -> anyhow::Result<Self> {
        Ok(match type_id {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            4 => unreachable!(),
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => bail!("unknown type_id {}", type_id),
        })
    }

    /// Applies the operation on the children.
    fn apply(&self, children: &[u64]) -> anyhow::Result<u64> {
        let children = children.iter().copied();
        Ok(match self {
            Operator::Sum => children.sum1().context("missing a child")?,
            Operator::Product => children.product1().context("missing a child")?,
            Operator::Maximum => children.max().context("missing a child")?,
            Operator::Minimum => children.min().context("missing a child")?,
            Operator::GreaterThan => {
                let [fst, snd] = children.collect_exact_array()?;
                (fst > snd) as u64
            }
            Operator::LessThan => {
                let [fst, snd] = children.collect_exact_array()?;
                (fst < snd) as u64
            }
            Operator::EqualTo => {
                let [fst, snd] = children.collect_exact_array()?;
                (fst == snd) as u64
            }
        })
    }
}

/// Converts a hexadecimal character into an array of four bits in MSB-first order.
/// Each bit in the output array is represented by `0` or `1`.
fn bits_from_hex(c: char) -> anyhow::Result<[Bit; 4]> {
    let decimal = c
        .to_digit(16)
        .map(|d| d as u8)
        .with_context(|| format!("not a hexadecimal character: '{}'", c.escape_default()))?;
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
