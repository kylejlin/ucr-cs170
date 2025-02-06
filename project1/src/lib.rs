use min_heap::MinHeap;
use std::collections::HashSet;

pub const PUZZLE_SIZE: usize = 3;

pub const GOAL_STATE: State = State {
    board: [
        [Tile(1), Tile(2), Tile(3)],
        [Tile(4), Tile(5), Tile(6)],
        [Tile(7), Tile(8), Tile(0)],
    ],
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tile(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinates(pub usize, pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    board: [[Tile; PUZZLE_SIZE]; PUZZLE_SIZE],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub state: State,

    /// The cost to reach this node starting from the root.
    /// For the root node, this is `0`.
    ///
    /// This is `g(x)` in the A* algorithm.
    ///
    /// For the 8-puzzle, this is simply the depth of the node.
    pub cost_to_reach: Cost,

    /// This is `f(x)` in the A* algorithm.
    pub total_cost: Cost,
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
pub fn search(
    initial_state: State,
    algorithm: Algorithm,

    // A tracer is just a fancy callback function.
    // We can use it for things like printing trace information to stdout,
    // or keeping track of the maximum queue size.
    tracer: &mut impl io::SearchTracer,
) -> Option<Node> {
    let initial_node = Node {
        state: initial_state,
        cost_to_reach: 0,
        total_cost: algorithm.estimate_cost_to_goal(&initial_state),
    };

    let mut queue = MinHeap::new();
    let mut visited = HashSet::new();

    queue.push(initial_node);
    visited.insert(initial_state);

    loop {
        let Some(node) = queue.pop() else {
            // If the queue is empty, there is no solution.
            return None;
        };

        tracer.on_dequeue(&node);

        if node.state.is_goal() {
            return Some(node);
        }

        expand_queue(&node, &mut queue, &mut visited, algorithm, tracer);
    }
}

fn expand_queue(
    parent_node: &Node,
    queue: &mut MinHeap<Node>,
    visited: &mut HashSet<State>,
    algorithm: Algorithm,
    tracer: &mut impl io::SearchTracer,
) {
    parent_node.state.for_each_child(|child_state| {
        let child_node = Node {
            state: child_state,

            // g(x)_child = g(x)_parent + 1
            cost_to_reach: parent_node.cost_to_reach + 1,

            // f(x)_child = g(x)_child + h(x)_child = g(x)_parent + 1 + h(x)_child
            total_cost: parent_node.cost_to_reach
                + 1
                + algorithm.estimate_cost_to_goal(&child_state),
        };

        // Check to see if the child has already been visited before
        // enqueueing it.
        let was_newly_inserted = visited.insert(child_node.state);
        if was_newly_inserted {
            queue.push(child_node);

            tracer.on_enqueue(&child_node, queue);
        }
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
        let blank_coords = self.blank_coords();

        // We represent our operators as "Move the blank up", "Move the blank down", etc.
        // Thus, we have at most 4 legal operators in any given state.

        if let Some(c) = blank_coords.up() {
            f(self.with_swapped_tiles(blank_coords, c));
        }

        if let Some(c) = blank_coords.down() {
            f(self.with_swapped_tiles(blank_coords, c));
        }

        if let Some(c) = blank_coords.left() {
            f(self.with_swapped_tiles(blank_coords, c));
        }

        if let Some(c) = blank_coords.right() {
            f(self.with_swapped_tiles(blank_coords, c));
        }
    }

    fn with_swapped_tiles(&self, a: Coordinates, b: Coordinates) -> State {
        let mut out = *self;

        let temp = out[a];
        out[a] = out[b];
        out[b] = temp;

        out
    }

    fn blank_coords(&self) -> Coordinates {
        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                if self.board[row][col] == Tile(0) {
                    return Coordinates(row, col);
                }
            }
        }

        panic!("Unreachable: Every state should have a blank tile.")
    }

    fn number_of_misplaced_tiles(&self) -> u32 {
        let mut count = 0;

        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                // Don't count the blank.
                if (row, col) == (PUZZLE_SIZE - 1, PUZZLE_SIZE - 1) {
                    continue;
                }

                let expected = Tile((row * PUZZLE_SIZE + col + 1) as u8);

                let actual = self.board[row][col];

                if actual != expected {
                    count += 1;
                }
            }
        }

        count
    }

    fn manhattan_distance_to_goal(&self) -> u32 {
        let mut total_distance = 0;

        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                let tile = self.board[row][col];

                // Don't count the blank.
                if tile == Tile(0) {
                    continue;
                }

                let actual_coords = Coordinates(row, col);

                let expected_coords = tile.expected_coords();

                let distance = actual_coords.manhattan_distance_to(&expected_coords);

                total_distance += distance;
            }
        }

        total_distance
    }

    fn is_goal(&self) -> bool {
        // For this problem, there is only one goal state.
        *self == GOAL_STATE
    }
}

impl Coordinates {
    fn up(&self) -> Option<Coordinates> {
        if self.0 <= 0 {
            None
        } else {
            Some(Coordinates(self.0 - 1, self.1))
        }
    }

    fn down(&self) -> Option<Coordinates> {
        if self.0 >= PUZZLE_SIZE - 1 {
            None
        } else {
            Some(Coordinates(self.0 + 1, self.1))
        }
    }

    fn left(&self) -> Option<Coordinates> {
        if self.1 <= 0 {
            None
        } else {
            Some(Coordinates(self.0, self.1 - 1))
        }
    }

    fn right(&self) -> Option<Coordinates> {
        if self.1 >= PUZZLE_SIZE - 1 {
            None
        } else {
            Some(Coordinates(self.0, self.1 + 1))
        }
    }

    fn manhattan_distance_to(&self, other: &Coordinates) -> u32 {
        let row_distance = (self.0 as i32 - other.0 as i32).abs();
        let col_distance = (self.1 as i32 - other.1 as i32).abs();

        (row_distance + col_distance) as u32
    }
}

impl Tile {
    fn expected_coords(&self) -> Coordinates {
        let index = self.0 - 1;

        let row = index / PUZZLE_SIZE as u8;
        let col = index % PUZZLE_SIZE as u8;

        Coordinates(row as usize, col as usize)
    }
}

impl std::ops::Index<Coordinates> for State {
    type Output = Tile;

    fn index(&self, index: Coordinates) -> &Self::Output {
        &self.board[index.0][index.1]
    }
}

impl std::ops::IndexMut<Coordinates> for State {
    fn index_mut(&mut self, index: Coordinates) -> &mut Self::Output {
        &mut self.board[index.0][index.1]
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

pub mod io;
pub mod min_heap;
