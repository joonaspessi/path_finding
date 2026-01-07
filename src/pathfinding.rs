use crate::grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeState {
    Unvisited,
    InQueue,
    Visited,
    Path,
}

/// Common interface for all pathfinding algorithms
pub trait PathfindingAlgorithm {
    /// Execute one step of the algorithm
    /// Returns true if still running, false if finished
    fn step(&mut self, grid: &Grid) -> bool;

    /// Get the visual state of a node for rendering
    fn get_node_state(&self, x: usize, y: usize) -> NodeState;

    /// Get the path from start to end (empty if no path found)
    fn get_path(&self) -> Vec<(usize, usize)>;

    /// Check if the algorithm has finished executing
    fn is_finished(&self) -> bool;

    /// Check if a path was found (only valid after finished)
    fn found_path(&self) -> bool;

    /// Get the algorithm's display name for the UI
    fn name(&self) -> &'static str;
}
