#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![Cell::Empty; width]; height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y][x] = cell;
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let yy = y as isize + dy;
                let xx = x as isize + dx;

                if yy >= 0 && yy < self.height as isize && xx >= 0 && xx < self.width as isize {
                    // Only add up/down/left/right neighbors, no diagonals
                    if (dy == 0 && (dx == -1 || dx == 1)) || ((dy == -1 || dy == 1) && dx == 0) {
                        neighbors.push((xx as usize, yy as usize));
                    }
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(3, 2);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.cells.len(), 2);
        assert_eq!(grid.cells[0].len(), 3);
        assert_eq!(grid.get(0, 0), Some(Cell::Empty));
        assert_eq!(grid.get(2, 1), Some(Cell::Empty));
        assert_eq!(grid.get(3, 0), None);
        assert_eq!(grid.get(0, 2), None);
    }

    #[test]
    fn test_grid_set_and_get() {
        let mut grid = Grid::new(2, 2);
        grid.set(1, 1, Cell::Wall);
        assert_eq!(grid.get(1, 1), Some(Cell::Wall));
        grid.set(0, 0, Cell::Start);
        assert_eq!(grid.get(0, 0), Some(Cell::Start));
        // Out of bounds should have no effect
        grid.set(2, 0, Cell::End);
        assert_eq!(grid.get(2, 0), None);
    }

    #[test]
    fn test_neighbors_center() {
        let grid = Grid::new(3, 3);
        let neighbors = grid.neighbors(1, 1);
        let expected = vec![(0, 1), (2, 1), (1, 0), (1, 2)];
        neighbors.iter().for_each(|n| assert!(expected.contains(n)));
        assert_eq!(neighbors.len(), 4);
    }

    #[test]
    fn test_neighbors_corner() {
        let grid = Grid::new(3, 3);
        let neighbors = grid.neighbors(0, 0);
        let expected = vec![(1, 0), (0, 1)];
        neighbors.iter().for_each(|n| assert!(expected.contains(n)));
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn test_neighbors_edge() {
        let grid = Grid::new(3, 3);
        let neighbors = grid.neighbors(2, 1);
        let expected = vec![(1, 1), (2, 0), (2, 2)];
        neighbors.iter().for_each(|n| assert!(expected.contains(n)));
        assert_eq!(neighbors.len(), 3);
    }
}
