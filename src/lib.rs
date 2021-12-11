#[allow(clippy::redundant_static_lifetimes)]
pub mod common {

    const OFFSETS: &'static [(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
    const OFFSETS_WITH_DIAGONALS: &'static [(i32, i32)] = &[
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    pub struct Grid {
        pub cells: Vec<Vec<u32>>,
    }

    impl Grid {
        pub fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), &u32)> {
            self.cells
                .iter()
                .enumerate()
                .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, col)| ((x, y), col)))
        }

        pub fn iter_adjacent(
            &self,
            x: usize,
            y: usize,
            include_diagonals: bool,
        ) -> impl Iterator<Item = (usize, usize)> {
            let max_x = self.max_x() as i32;
            let max_y = self.max_y() as i32;
            let offsets = if include_diagonals {
                OFFSETS_WITH_DIAGONALS
            } else {
                OFFSETS
            };
            offsets
                .iter()
                .map(move |(dx, dy)| ((x as i32) + dx, (y as i32) + dy))
                .filter(move |(cx, cy)| (*cx >= 0 && *cx < max_x) && (*cy >= 0 && *cy < max_y))
                .map(|(cx, cy)| (cx as usize, cy as usize))
        }

        pub fn max_x(&self) -> usize {
            self.cells.len()
        }

        pub fn max_y(&self) -> usize {
            self.cells.get(0).map_or(0, |v| v.len())
        }

        pub fn print(&self) {
            for row in &self.cells {
                println!("{:?}", row);
            }
            println!();
        }
    }
}
