use macroquad::prelude::*;
use path_finding::{
    astar::AStar,
    bfs::Bfs,
    cellular_automata::CellularAutomata,
    dfs::Dfs,
    dijkstra::Dijkstra,
    grid::{Cell, Grid},
    pathfinding::{NodeState, PathfindingAlgorithm},
};

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const STEP_DELAY: f32 = 0.01;
const STATUS_BAR_HEIGHT: f32 = 50.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AlgorithmType {
    Dijkstra,
    AStar,
    Bfs,
    Dfs,
}

impl AlgorithmType {
    pub fn all() -> &'static [Self] {
        &[Self::Dijkstra, Self::AStar, Self::Bfs, Self::Dfs]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Dijkstra => "Dijkstra",
            Self::AStar => "A*",
            Self::Bfs => "BFS",
            Self::Dfs => "DFS",
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::all();
        let current_idx = all.iter().position(|&a| a == *self).unwrap_or(0);
        let next_idx = (current_idx + 1) % all.len();
        all[next_idx]
    }
}

impl Default for AlgorithmType {
    fn default() -> Self {
        Self::AStar
    }
}

fn create_algorithm(
    algorithm_type: AlgorithmType,
    start: (usize, usize),
    end: (usize, usize),
) -> Box<dyn PathfindingAlgorithm> {
    match algorithm_type {
        AlgorithmType::Dijkstra => Box::new(Dijkstra::new(start, end)),
        AlgorithmType::AStar => Box::new(AStar::new(start, end)),
        AlgorithmType::Bfs => Box::new(Bfs::new(start, end)),
        AlgorithmType::Dfs => Box::new(Dfs::new(start, end)),
    }
}

enum AppState {
    Editing,
    Running,
    Finished,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dijkstra Visualization".to_owned(),
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE + STATUS_BAR_HEIGHT) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut app_state = AppState::Editing;
    let mut path_algo: Option<Box<dyn PathfindingAlgorithm>> = None;
    let mut current_algorithm = AlgorithmType::default();
    let mut step_timer = 0.0;
    let mut cave_seed: u64 = 0;
    let mut first_run: bool = true;

    loop {
        if is_key_pressed(KeyCode::Tab) {
            if let AppState::Editing = app_state {
                current_algorithm = current_algorithm.next();
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some((x, y)) = mouse_to_grid(&grid) {
                let current = grid.get(x, y).unwrap_or(Cell::Empty);
                let new_cell = if current == Cell::Wall {
                    Cell::Empty
                } else {
                    Cell::Wall
                };
                grid.set(x, y, new_cell);
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if let Some((x, y)) = mouse_to_grid(&grid) {
                let current = grid.get(x, y).unwrap_or(Cell::Empty);
                let (start, end) = find_start_end(&grid);

                match (start, end) {
                    (None, None) if current == Cell::Empty => {
                        grid.set(x, y, Cell::Start);
                    }
                    (Some(_), None) if current == Cell::Empty => {
                        grid.set(x, y, Cell::End);
                    }
                    _ => {
                        // do nothing
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            match app_state {
                AppState::Editing => {
                    let (start, end) = find_start_end(&grid);
                    if let (Some(s), Some(e)) = (start, end) {
                        path_algo = Some(create_algorithm(current_algorithm, s, e));
                        app_state = AppState::Running;
                        step_timer = 0.0;
                    }
                }
                AppState::Running => {
                    app_state = AppState::Editing;
                }
                AppState::Finished => {
                    path_algo = None;
                    app_state = AppState::Editing;
                }
            }
        }

        if is_key_pressed(KeyCode::G) || first_run {
            cave_seed += 1;
            path_algo = None;
            app_state = AppState::Editing;

            let generate = CellularAutomata {
                seed: cave_seed,
                ..Default::default()
            };
            generate.generate(&mut grid);
            first_run = false;
        }

        if let AppState::Running = app_state {
            step_timer += get_frame_time();
            while step_timer >= STEP_DELAY {
                step_timer -= STEP_DELAY;
                if let Some(ref mut d) = path_algo {
                    if !d.step(&grid) {
                        app_state = AppState::Finished;
                        break;
                    }
                }
            }
        }

        clear_background(BLACK);
        draw_grid(&grid, path_algo.as_deref());

        let selector_y = GRID_HEIGHT as f32 * CELL_SIZE + 22.0;
        let mut x_offset = 10.0;

        for algo_type in AlgorithmType::all() {
            let is_selected = *algo_type == current_algorithm;
            let name = algo_type.name();

            if is_selected {
                let text_size = measure_text(name, None, 16, 1.0);
                let padding = 4.0;
                draw_rectangle(
                    x_offset - padding,
                    selector_y - text_size.height - padding,
                    text_size.width + padding * 2.0,
                    text_size.height + padding * 2.0,
                    DARKGRAY,
                );
            }

            let color = if is_selected { WHITE } else { GRAY };
            draw_text(name, x_offset, selector_y, 16.0, color);

            let text_width = measure_text(name, None, 16, 1.0).width;
            x_offset += text_width + 20.0;
        }

        let status = match app_state {
            AppState::Editing => &format!(
                "Seed: {} | Tab: switch algorithm | G: new cave | SPACE: pathfind",
                cave_seed
            ),
            AppState::Running => "Running... SPACE to pause",
            AppState::Finished => {
                if let Some(ref d) = path_algo {
                    if d.found_path() {
                        "Path found! SPACE to reset"
                    } else {
                        "No path exists! SPACE to reset"
                    }
                } else {
                    "SPACE to reset"
                }
            }
        };
        let status_y = GRID_HEIGHT as f32 * CELL_SIZE + 45.0;
        draw_text(status, 10.0, status_y, 16.0, WHITE);
        next_frame().await
    }
}

fn draw_grid(grid: &Grid, path_algo: Option<&dyn PathfindingAlgorithm>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let base_color = match grid.get(x, y) {
                Some(Cell::Empty) => DARKGRAY,
                Some(Cell::Wall) => BLACK,
                Some(Cell::Start) => GREEN,
                Some(Cell::End) => RED,
                None => DARKGRAY,
            };

            let color = if let Some(d) = path_algo {
                match d.get_node_state(x, y) {
                    NodeState::Path => LIME,
                    NodeState::Visited => SKYBLUE,
                    NodeState::InQueue => YELLOW,
                    NodeState::Unvisited => base_color,
                }
            } else {
                base_color
            };

            let final_color = match grid.get(x, y) {
                Some(Cell::Start) => GREEN,
                Some(Cell::End) => RED,
                _ => color,
            };

            draw_rectangle(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE - 1.0,
                CELL_SIZE - 1.0,
                final_color,
            );
        }
    }
}

fn mouse_to_grid(grid: &Grid) -> Option<(usize, usize)> {
    let (mx, my) = mouse_position();
    let gx = (mx / CELL_SIZE) as usize;
    let gy = (my / CELL_SIZE) as usize;

    if gx < grid.width && gy < grid.height {
        Some((gx, gy))
    } else {
        None
    }
}

type Position = (usize, usize);

fn find_start_end(grid: &Grid) -> (Option<Position>, Option<Position>) {
    let mut start = None;
    let mut end = None;

    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some(Cell::Start) => start = Some((x, y)),
                Some(Cell::End) => end = Some((x, y)),
                _ => {}
            }
        }
    }

    (start, end)
}
