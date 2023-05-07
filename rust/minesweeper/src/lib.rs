const MINE: u8 = b'*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut grid = minefield
        .iter()
        .map(|&row| (*row).as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == MINE {
                continue;
            }

            let mut count = b'0';

            for ro in -1..=1 {
                for co in -1..=1 {
                    if (ro != 0 || co != 0)
                        && (r != 0 || ro != -1)
                        && (r != rows - 1 || ro != 1)
                        && (c != 0 || co != -1)
                        && (c != cols - 1 || co != 1)
                    {
                        if grid[r.wrapping_add(ro as usize)][c.wrapping_add(co as usize)] == MINE {
                            count += 1;
                        }
                    }
                }
            }

            if count != b'0' {
                grid[r][c] = count;
            };
        }
    }

    grid.iter()
        .map(|row| String::from_utf8_lossy(row).into())
        .collect()
}
