use anyhow::bail;

use aoc2021::collect_array::CollectArray;

fn main() {
    let result: [i64; 50] = (0..100)
        .map(|v| if v < 100 { Ok(v) } else { bail!("no") })
        .try_collect_trunc()
        .unwrap();
    println!("{:?}", result);

    let result: [i64; 30] = (0..5).collect_trunc_recoverable().unwrap();
    println!("{:?}", result);
}
