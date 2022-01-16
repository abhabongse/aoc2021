//! Day 20: Trench Map, Advent of Code 2021  
//! <https://adventofcode.com/2021/day/20>
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use anyhow::{bail, ensure, Context};
use clap::Parser;
use itertools::iproduct;

use aoc2021::argparser::Cli;
use aoc2021::collect_array::CollectArray;
use aoc2021::grid::GridPoint;

/// Main program
fn main() {
    let cli = Cli::parse();
    let input_reader = BufReader::new(cli.input_reader().expect("cannot open file"));
    let Input {
        enhancer_table,
        input_image,
    } = Input::from_buffer(input_reader).expect("cannot parse input");

    // Part 1: Enhance image twice
    let p1_answer = {
        let image = input_image.enhance(&enhancer_table);
        let image = image.enhance(&enhancer_table);
        assert!(!image.fallback_pixels);
        image.on_pixels.len()
    };
    println!("Part 1 answer: {}", p1_answer);

    // Part 2: Enhance image 50 times
    let p2_answer = {
        let image = (0..50).fold(input_image, |image, _| image.enhance(&enhancer_table));
        assert!(!image.fallback_pixels);
        image.on_pixels.len()
    };
    println!("Part 2 answer: {}", p2_answer);
}

/// Program input data
#[derive(Debug, Clone)]
struct Input {
    /// Image enhancement algorithm lookup table
    enhancer_table: [bool; 512],
    /// Input image
    input_image: Image,
}

impl Input {
    /// Parses program input from buffered reader.
    fn from_buffer(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let enhancer_lookup = {
            let line = lines.next().context("expected first line")??;
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => bail!("invalid char: '{}'", c.escape_default()),
                })
                .try_collect_exact()?
        };

        let break_line = lines.next().context("expected empty second line")??;
        ensure!(break_line.trim().is_empty(), "expected empty second line");

        let mut on_pixels = HashSet::new();
        for (i, line) in lines.enumerate() {
            for (j, c) in line?.trim().chars().enumerate() {
                if c == '#' {
                    on_pixels.insert((i as i64, j as i64));
                }
            }
        }
        let x_values = on_pixels.iter().copied().map(|p| p.0);
        let x_max = x_values.max().context("empty image")?;
        let y_values = on_pixels.iter().copied().map(|p| p.1);
        let y_max = y_values.max().context("empty image")?;
        let input_image = Image {
            min_point: (0, 0),
            max_point: (x_max, y_max),
            on_pixels,
            fallback_pixels: false,
        };

        Ok(Input {
            enhancer_table: enhancer_lookup,
            input_image,
        })
    }
}

/// One possible representation of an image
#[derive(Debug, Clone)]
struct Image {
    /// Top-left corner position of the core image region
    min_point: GridPoint<i64>,
    /// Bottom-right corner position of the core image region
    max_point: GridPoint<i64>,
    /// Set of locations of light pixels being turned on within the core image region
    on_pixels: HashSet<GridPoint<i64>>,
    /// Whether the pixel is lit outside the core image region
    fallback_pixels: bool,
}

impl Image {
    /// Gets the boolean state of a pixel of the image
    fn get(&self, index: (i64, i64)) -> bool {
        if self.min_point.0 <= index.0
            && index.0 <= self.max_point.0
            && self.min_point.1 <= index.1
            && index.1 <= self.max_point.1
        {
            self.on_pixels.contains(&index)
        } else {
            self.fallback_pixels
        }
    }

    /// Enhance an image using the lookup `table` through Image Enhancement Algorithm.
    fn enhance(&self, enhancer_table: &[bool; 512]) -> Self {
        let x_min = self.min_point.0 - 1;
        let y_min = self.min_point.1 - 1;
        let x_max = self.max_point.0 + 1;
        let y_max = self.max_point.1 + 1;
        let on_pixels: HashSet<GridPoint<i64>> = iproduct!(x_min..=x_max, y_min..=y_max)
            .filter(|pos| {
                let index = iproduct!(-1..=1, -1..=1).fold(0, |acc, step| {
                    2 * acc + (self.get((pos.0 + step.0, pos.1 + step.1))) as usize
                });
                enhancer_table[index]
            })
            .collect();
        let fallback_pixels = match self.fallback_pixels {
            true => enhancer_table[511],
            false => enhancer_table[0],
        };
        Image {
            min_point: (x_min, y_min),
            max_point: (x_max, y_max),
            on_pixels,
            fallback_pixels,
        }
    }
}
