use crate::grid::{Cell, Grid};
use crate::pathfinding::{NodeState, PathfindingAlgorithm};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Bfs {
    queue: VecDeque<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    parents: HashMap<(usize, usize), (usize, usize)>,
    node_states: HashMap<(usize, usize), NodeState>,
    start: (usize, usize),
    end: (usize, usize),
    finished: bool,
    found_path: bool,
}

impl Bfs {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        let mut bfs = Self {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            parents: HashMap::new(),
            node_states: HashMap::new(),
            start,
            end,
            finished: false,
            found_path: false,
        };

        bfs.queue.push_back(start);
        bfs.node_states.insert(start, NodeState::InQueue);

        bfs
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

impl PathfindingAlgorithm for Bfs {
    fn step(&mut self, grid: &Grid) -> bool {
        if self.finished {
            return false;
        }

        let current = match self.queue.pop_front() {
            Some(pos) => pos,
            None => {
                self.finished = true;
                return false;
            }
        };

        if self.visited.contains(&current) {
            return true;
        }

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

            if self.node_states.get(&(nx, ny)) == Some(&NodeState::InQueue) {
                continue;
            }

            self.parents.insert((nx, ny), current);
            self.queue.push_back((nx, ny));
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
        "BFS"
    }
}
