use crate::grid::{Cell, Grid};
use crate::pathfinding::{NodeState, PathfindingAlgorithm};
use std::collections::{HashMap, HashSet};

pub struct Dfs {
    stack: Vec<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    parents: HashMap<(usize, usize), (usize, usize)>,
    node_states: HashMap<(usize, usize), NodeState>,
    start: (usize, usize),
    end: (usize, usize),
    finished: bool,
    found_path: bool,
}

impl Dfs {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        let mut dfs = Dfs {
            stack: Vec::new(),
            visited: HashSet::new(),
            parents: HashMap::new(),
            node_states: HashMap::new(),
            start,
            end,
            finished: false,
            found_path: false,
        };

        dfs.stack.push(start);
        dfs.node_states.insert(start, NodeState::InQueue);

        dfs
    }

    fn mark_path(&mut self) {
        let mut current = self.end;
        while current != self.start {
            self.node_states.insert(current, NodeState::Path);
            if let Some(&parent) = self.parents.get(&current) {
                current = parent;
            } else {
                break;
            }
        }
        self.node_states.insert(self.start, NodeState::Path);
    }
}

impl PathfindingAlgorithm for Dfs {
    fn step(&mut self, grid: &Grid) -> bool {
        if self.finished {
            return false;
        }

        let current = match self.stack.pop() {
            Some(pos) => pos,
            None => {
                self.finished = true;
                return false;
            }
        };

        self.visited.insert(current);
        self.node_states.insert(current, NodeState::Visited);

        if current == self.end {
            self.finished = true;
            self.found_path = true;
            self.mark_path();
            return false;
        }

        for (nx, ny) in grid.neighbors(current.0, current.1) {
            if let Some(cell) = grid.get(nx, ny) {
                if cell == Cell::Wall {
                    continue;
                }
            }

            if self.visited.contains(&(nx, ny)) {
                continue;
            }

            // Add neighbor to stack
            // NOTE: We don't check "already in stack" like BFS does
            // Duplicates are OK as they'll we skipped when popped
            if !self.parents.contains_key(&(nx, ny)) {
                self.parents.insert((nx, ny), current);
            }
            self.stack.push((nx, ny));
            self.node_states.insert((nx, ny), NodeState::InQueue);
        }
        true
    }

    fn get_node_state(&self, x: usize, y: usize) -> NodeState {
        *self
            .node_states
            .get(&(x, y))
            .unwrap_or(&NodeState::Unvisited)
    }

    fn get_path(&self) -> Vec<(usize, usize)> {
        if !self.found_path {
            return Vec::new();
        }

        let mut path = Vec::new();
        let mut current = self.end;

        while current != self.start {
            path.push(current);
            if let Some(&parent) = self.parents.get(&current) {
                current = parent;
            } else {
                break;
            }
        }
        path.push(self.start);
        path.reverse();
        path
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn found_path(&self) -> bool {
        self.found_path
    }

    fn name(&self) -> &'static str {
        "DFS"
    }
}
