use crate::grid::{Cell, Grid};
use crate::pathfinding::{NodeState, PathfindingAlgorithm};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    distance: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra {
    pub distances: HashMap<(usize, usize), u32>,
    pub parents: HashMap<(usize, usize), (usize, usize)>,
    pub visited: HashSet<(usize, usize)>,
    pub node_states: HashMap<(usize, usize), NodeState>,
    queue: BinaryHeap<Node>,
    start: (usize, usize),
    end: (usize, usize),
    pub finished: bool,
    pub found_path: bool,
}

impl PathfindingAlgorithm for Dijkstra {
    fn step(&mut self, grid: &Grid) -> bool {
        if self.finished {
            return false;
        }

        let current = match self.queue.pop() {
            Some(node) => node,
            None => {
                self.finished = true;
                return false;
            }
        };

        let pos = current.position;

        if self.visited.contains(&pos) {
            return true;
        }

        self.visited.insert(pos);
        self.node_states.insert(pos, NodeState::Visited);

        if pos == self.end {
            self.finished = true;
            self.found_path = true;
            self.mark_path();
            return false;
        }

        let current_dist = *self.distances.get(&pos).unwrap_or(&u32::MAX);

        for (nx, ny) in grid.neighbors(pos.0, pos.1) {
            if let Some(cell) = grid.get(nx, ny) {
                if cell == Cell::Wall {
                    continue;
                }
            }

            if self.visited.contains(&(nx, ny)) {
                continue;
            }

            let new_dist = current_dist + 1;
            let old_dist = *self.distances.get(&(nx, ny)).unwrap_or(&u32::MAX);
            if new_dist < old_dist {
                self.distances.insert((nx, ny), new_dist);
                self.parents.insert((nx, ny), pos);
                self.queue.push(Node {
                    position: (nx, ny),
                    distance: new_dist,
                });
                self.node_states.insert((nx, ny), NodeState::InQueue);
            }
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
        "Dijkstra"
    }
}

impl Dijkstra {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        let mut dijkstra = Dijkstra {
            distances: HashMap::new(),
            parents: HashMap::new(),
            visited: HashSet::new(),
            node_states: HashMap::new(),
            queue: BinaryHeap::new(),
            start,
            end,
            finished: false,
            found_path: false,
        };

        dijkstra.distances.insert(start, 0);
        dijkstra.queue.push(Node {
            position: start,
            distance: 0,
        });
        dijkstra.node_states.insert(start, NodeState::InQueue);
        dijkstra
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
