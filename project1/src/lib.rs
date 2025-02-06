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
    };

    // We'll use a simple (albeit inefficient) queue design:
    // Store the nodes in a vector in descending order of cost.
    // - To dequeue, just `pop()` the last element.
    // - To enqueue, insert the new node at the correct position.
    // This means each enqueue operation is O(n), but the dequeues are O(1),
    // where n is the length of the queue.
    //
    // I'm sure there's a cleaner way to do this with trees or heaps or something,
    // but that's not the focus of this project, so I'm using this simple approach.
    let mut queue = Vec::new();

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

fn expand_queue(parent: &Node, queue: &mut Vec<Node>, algorithm: Algorithm) {
    parent.state.for_each_child(|child| {
        let child_cost = algorithm.cost(&child);

        // Calculate the index where the child should be inserted.
        let index = queue
            .iter()
            // Try to find the first index where the cost is greater than the child's cost...
            .position(|other| algorithm.cost(other) > child_cost)
            // ...but if all the costs are less than or equal to the child's cost,
            // then insert the child at the end.
            .unwrap_or(queue.len());

        queue.insert(index, child);
    })
}

impl Algorithm {
    /// This is `f(x) := g(x) + h(x)` in the A* algorithm.
    fn cost(self, node: &Node) -> Cost {
        node.cost_to_reach + self.estimate_cost_to_goal(node)
    }

    /// This is `h(x)` in the A* algorithm.
    fn estimate_cost_to_goal(self, node: &Node) -> Cost {
        match self {
            // With Uniform Cost Search, we simply hardcode h(x) to 0.
            Algorithm::UniformCostSearch => 0,

            Algorithm::MisplacedTileHeuristic => node.state.number_of_misplaced_tiles(),

            Algorithm::ManhattanDistanceHeuristic => node.state.manhattan_distance_to_goal(),
        }
    }
}

impl State {
    /// Iterates over every child,
    /// calling the visitor function `f` on each one.
    fn for_each_child(&self, mut f: impl FnMut(Node)) {
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

pub mod min_heap;
pub mod pretty_print;
pub mod ui;
