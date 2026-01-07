use crate::grid::{Cell, Grid};
use crate::pathfinding::{NodeState, PathfindingAlgorithm};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    g_cost: u32, // actual cost from start
    f_cost: u32, // g_cost + heuristic (estimated total cost)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.f_cost.cmp(&self.f_cost) {
            Ordering::Equal => self.g_cost.cmp(&other.g_cost), // higher g = closer to goal,
            other => other,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic_manhantan(from: (usize, usize), to: (usize, usize)) -> u32 {
    let dx = (from.0 as i32 - to.0 as i32).abs() as u32;
    let dy = (from.1 as i32 - to.1 as i32).abs() as u32;
    dx + dy
}

pub struct AStar {
    g_costs: HashMap<(usize, usize), u32>,
    parents: HashMap<(usize, usize), (usize, usize)>,
    visited: HashSet<(usize, usize)>,
    node_states: HashMap<(usize, usize), NodeState>,
    queue: BinaryHeap<Node>,
    start: (usize, usize),
    end: (usize, usize),
    pub finished: bool,
    pub found_path: bool,
}

impl PathfindingAlgorithm for AStar {
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

        let current_g = *self.g_costs.get(&pos).unwrap_or(&u32::MAX);

        for (nx, ny) in grid.neighbors(pos.0, pos.1) {
            if let Some(cell) = grid.get(nx, ny) {
                if cell == Cell::Wall {
                    continue;
                }
            }

            if self.visited.contains(&(nx, ny)) {
                continue;
            }

            let new_g = current_g + 1;
            let old_g = *self.g_costs.get(&(nx, ny)).unwrap_or(&u32::MAX);

            if new_g < old_g {
                let new_f = new_g + heuristic_manhantan((nx, ny), self.end);
                self.g_costs.insert((nx, ny), new_g);
                self.parents.insert((nx, ny), pos);
                self.queue.push(Node {
                    position: (nx, ny),
                    g_cost: new_g,
                    f_cost: new_f,
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
        "A*"
    }
}

impl AStar {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        let mut astar = AStar {
            g_costs: HashMap::new(),
            parents: HashMap::new(),
            visited: HashSet::new(),
            node_states: HashMap::new(),
            queue: BinaryHeap::new(),
            start,
            end,
            finished: false,
            found_path: false,
        };

        astar.g_costs.insert(start, 0);
        let h = heuristic_manhantan(start, end);
        astar.queue.push(Node {
            position: start,
            g_cost: 0,
            f_cost: h, // f = g + h = 0 + h
        });

        astar
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_heuristics() {
        let a = heuristic_manhantan((0, 0), (2, 2));
        assert_eq!(a, 4);

        let b = heuristic_manhantan((2, 2), (0, 0));
        assert_eq!(b, 4);
    }
}
