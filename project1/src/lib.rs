use min_heap::MinHeap;

pub const PUZZLE_SIZE: usize = 3;

pub const GOAL_STATE: State = State {
    board: [
        [Tile(1), Tile(2), Tile(3)],
        [Tile(4), Tile(5), Tile(6)],
        [Tile(7), Tile(8), Tile(0)],
    ],
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct State {
    board: [[Tile; PUZZLE_SIZE]; PUZZLE_SIZE],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node {
    state: State,

    /// The cost to reach this node starting from the root.
    /// For the root node, this is `0`.
    ///
    /// This is `g(x)` in the A* algorithm.
    cost_to_reach: Cost,

    /// This is `f(x)` in the A* algorithm.
    total_cost: Cost,
}

pub type Cost = u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Algorithm {
    UniformCostSearch,
    MisplacedTileHeuristic,
    ManhattanDistanceHeuristic,
}

pub const DEFAULT_INITIAL_STATE: State = State {
    board: [
        [Tile(1), Tile(2), Tile(3)],
        [Tile(4), Tile(0), Tile(6)],
        [Tile(7), Tile(5), Tile(8)],
    ],
};

/// Returns `Some(solution)` if a solution was found, or `None` if no solution was found.
pub fn search(initial_state: State, algorithm: Algorithm) -> Option<State> {
    let initial_node = Node {
        state: initial_state,
        cost_to_reach: 0,
        total_cost: algorithm.estimate_cost_to_goal(&initial_state),
    };

    let mut queue = MinHeap::new();

    queue.push(initial_node);

    loop {
        let Some(node) = queue.pop() else {
            // If the queue is empty, there is no solution.
            return None;
        };

        if node.state.is_goal() {
            return Some(node.state);
        }

        expand_queue(&node, &mut queue, algorithm);
    }
}

fn expand_queue(parent_node: &Node, queue: &mut MinHeap<Node>, algorithm: Algorithm) {
    parent_node.state.for_each_child(|child_state| {
        let child_node = Node {
            state: child_state,
            cost_to_reach: parent_node.cost_to_reach + 1,
            total_cost: parent_node.cost_to_reach
                + 1
                + algorithm.estimate_cost_to_goal(&child_state),
        };

        queue.push(child_node);
    })
}

impl Algorithm {
    /// This is `h(x)` in the A* algorithm.
    fn estimate_cost_to_goal(self, state: &State) -> Cost {
        match self {
            // With Uniform Cost Search, we simply hardcode `h(x)` to `0`.
            Algorithm::UniformCostSearch => 0,

            Algorithm::MisplacedTileHeuristic => state.number_of_misplaced_tiles(),

            Algorithm::ManhattanDistanceHeuristic => state.manhattan_distance_to_goal(),
        }
    }
}

impl State {
    /// Iterates over every child,
    /// calling the visitor function `f` on each one.
    fn for_each_child(&self, mut f: impl FnMut(State)) {
        todo!()
    }

    fn number_of_misplaced_tiles(&self) -> u32 {
        todo!()
    }

    fn manhattan_distance_to_goal(&self) -> u32 {
        todo!()
    }

    fn is_goal(&self) -> bool {
        // For this problem, there is only one goal state.
        *self == GOAL_STATE
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub mod min_heap;
pub mod pretty_print;
pub mod ui;
