

pub struct Config {
    pub gamefield_width: usize;
    pub gamefield_height: usize;
    pub maximum_items_on_ground: usize;
}

impl Config {
    pub fn new(gamefield_width: usize, gamefield_height: usize, maximum_items_on_ground: usize) -> Config {
        Config{
            gamefield_width,
            gamefield_height,
            maximum_items_on_ground
        }
    }

    pub fn default() -> Config {
        Config {
            10,
            10,
            8
        }
    }
}

pub struct Karel {
    gamefield: Vec<usize>;
    karel_position: (usize, usize);
    karel_orientation: Direction;
    configuration: Config;
}

impl Karel {
    pub fn new(configuration: Config) -> Karel {
    }
}

/// Enum that describes actions that can karel do
enum Action {
    /// Move forward
    Move,
    /// Place item on current tile
    PlaceItem,
    /// Remove item from current tile
    RemoveItem,
    /// Turn left
    TurnLeft,
}

/// Enum that describes queries that can be asked to karel
enum Query {
    /// Returns true if there is a wall in front of Karel
    WallInFrontOfMe,
    /// Returns true if there is at least one item on current tile
    ItemHere,
    /// Returns one item from Direction enum, describes Karel orientation
    Direction
}

enum Direction {
    North,
    South,
    East,
    West
}

/// Enum that describes various errors that can be caused by
/// ordering Karel do do an action.
enum ActionError {
    /// This error means that Karel was ordered to run into wall.
    MoveWall,
    /// This error means that Karel was ordered to move out of game field.
    MoveOutOfBounds,
    /// This error means that Karel was ordered to place an item on tile which
    /// already contains maximum amount of items.
    ExceedItemLimit,
    /// This error means that Karel was ordered to pick up an item on tile, which
    /// has no items on it.
    NoItemHere
}