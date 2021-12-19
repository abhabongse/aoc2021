/// A two-dimensional grid point, can be used to describe the shape of the grid
/// as well as describe an indexed position into a cell in the grid.
pub type GridPoint = (usize, usize);

/// Finds a sequence of up to four grid points which are orthogonally (horizontally or vertically)
/// adjacent to the given `pos` and are bound within the given rectangular `shape`,
pub fn orth_adjacent(pos: GridPoint, shape: GridPoint) -> Vec<GridPoint> {
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

/// Finds a sequence of up to eight grid points which is a king's move away from the given `pos`
/// (i.e. horizontally, vertically, or diagonally adjacent grid points)
/// and are bound within the given rectangular `shape`.
pub fn king_adjacent(pos: GridPoint, shape: GridPoint) -> Vec<GridPoint> {
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
