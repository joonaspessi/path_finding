use std::collections::{HashMap, HashSet, VecDeque};

use crate::grid::{Cell, Grid};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct CellularAutomata {
    pub wall_change: f32,
    pub smoothing_passes: u32,
    pub seed: u64,
}

impl Default for CellularAutomata {
    fn default() -> Self {
        Self {
            wall_change: 0.45,
            smoothing_passes: 1,
            seed: 12345,
        }
    }
}

impl CellularAutomata {
    pub fn generate(&self, grid: &mut Grid) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        // phase 1: random fill
        self.random_fill(grid, &mut rng);

        // phase 2: smooth
        for _ in 0..self.smoothing_passes {
            self.smooth(grid);
        }

        // phase 3: keep only largest connected region
        self.keep_largest_region(grid);

        // phase 4: set endpoints
        self.place_endpoints(grid, &mut rng);
    }

    fn random_fill(&self, grid: &mut Grid, rng: &mut StdRng) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let is_border = x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1;
                if is_border {
                    grid.set(x, y, Cell::Wall);
                } else {
                    let rand_val: f32 = rng.gen();
                    if rand_val < self.wall_change {
                        grid.set(x, y, Cell::Wall);
                    } else {
                        grid.set(x, y, Cell::Empty);
                    }
                }
            }
        }
    }

    fn count_wall_neighbors(&self, grid: &Grid, x: usize, y: usize) -> u32 {
        let mut count = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // out of bounds or actual wall
                if nx < 0
                    || ny < 0
                    || nx >= grid.width as i32
                    || ny >= grid.height as i32
                    || grid.get(nx as usize, ny as usize) == Some(Cell::Wall)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn smooth(&self, grid: &mut Grid) {
        for y in 1..grid.height - 1 {
            for x in 1..grid.width - 1 {
                let wall_count = self.count_wall_neighbors(grid, x, y);

                if wall_count >= 5 {
                    grid.set(x, y, Cell::Wall);
                } else if wall_count < 4 {
                    grid.set(x, y, Cell::Empty);
                }
            }
        }
    }

    fn find_regions(&self, grid: &Grid) -> HashMap<u32, HashSet<(usize, usize)>> {
        let mut regions: HashMap<u32, HashSet<(usize, usize)>> = HashMap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut region_id = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) != Some(Cell::Empty) {
                    continue;
                }
                if visited.contains(&(x, y)) {
                    continue;
                }

                let region = self.flood_fill(grid, x, y, &mut visited);
                regions.insert(region_id, region);
                region_id += 1;
            }
        }
        regions
    }

    fn flood_fill(
        &self,
        grid: &Grid,
        start_x: usize,
        start_y: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut region = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            // Skip walls
            if grid.get(x, y) != Some(Cell::Empty) {
                continue;
            }

            visited.insert((x, y));
            region.insert((x, y));

            let neighbors = grid.neighbors(x, y);

            for n in neighbors {
                queue.push_back(n);
            }
        }

        region
    }

    fn keep_largest_region(&self, grid: &mut Grid) {
        let regions = self.find_regions(grid);
        let largest = regions
            .iter()
            .max_by_key(|(_, cells)| cells.len())
            .map(|(id, _)| *id);

        let Some(largest_id) = largest else {
            return; // no regions found
        };

        for (id, cells) in &regions {
            if *id != largest_id {
                for &(x, y) in cells {
                    grid.set(x, y, Cell::Wall);
                }
            }
        }
    }

    fn find_furthest(&self, grid: &Grid, from: (usize, usize)) -> (usize, usize) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut furthest = from;

        queue.push_back(from);
        visited.insert(from);

        while let Some((x, y)) = queue.pop_front() {
            furthest = (x, y);

            for (nx, ny) in grid.neighbors(x, y) {
                if visited.contains(&(nx, ny)) {
                    continue;
                }

                if grid.get(nx, ny) != Some(Cell::Empty) {
                    continue;
                }

                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }

        furthest
    }

    fn place_endpoints(&self, grid: &mut Grid, rng: &mut StdRng) {
        let mut floor_cells: Vec<(usize, usize)> = Vec::new();

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) == Some(Cell::Empty) {
                    floor_cells.push((x, y));
                }

                if grid.get(x, y) == Some(Cell::Start) || grid.get(x, y) == Some(Cell::End) {
                    grid.set(x, y, Cell::Empty);
                    floor_cells.push((x, y));
                }
            }
        }

        if floor_cells.is_empty() {
            return;
        }

        let random_start = floor_cells[rng.gen_range(0..floor_cells.len())];
        let far1 = self.find_furthest(grid, random_start);
        let far2 = self.find_furthest(grid, far1);

        grid.set(far1.0, far1.1, Cell::Start);
        grid.set(far2.0, far2.1, Cell::End);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cellular_automata::CellularAutomata,
        grid::{Cell, Grid},
    };

    #[test]
    fn test_count_neighbors_cornre() {
        let mut grid = Grid::new(5, 5);
        let gen = CellularAutomata::default();

        grid.set(0, 0, Cell::Wall);
        // Corner (0,0) has 5 out-of-bounds neighbors + itself
        assert_eq!(gen.count_wall_neighbors(&grid, 0, 0), 6);
    }
}
