use super::GridIndex;

/// Obtains a list of up to four positions which are orthogonally adjacent to the given `pos`
/// and are bounded within `(0..rows, 0..cols)` where `(rows, cols) == shape`.
pub fn orthogonal_neighbors(pos: GridIndex, shape: GridIndex) -> Vec<GridIndex> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(di, dj)| {
            Some((
                clipped_add(pos.0, di, shape.0)?,
                clipped_add(pos.1, dj, shape.1)?,
            ))
        })
        .collect()
}

/// Obtains a list of up to eight positions which is a king's move away from the given `pos`
/// (i.e. are orthogonally and diagonally adjacent to such position)
/// and are bounded within `(0..rows, 0..cols)` where `(rows, cols) == shape`.
pub fn king_step_neighbors(pos: GridIndex, shape: GridIndex) -> Vec<GridIndex> {
    [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ]
    .into_iter()
    .filter_map(|(di, dj)| {
        Some((
            clipped_add(pos.0, di, shape.0)?,
            clipped_add(pos.1, dj, shape.1)?,
        ))
    })
    .collect()
}

fn clipped_add(a: usize, b: i64, size: usize) -> Option<usize> {
    let total = (a as i64) + b;
    (0..size as i64).contains(&total).then(|| total as usize)
}
