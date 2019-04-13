pub struct Config {
    pub gamefield_width: usize,
    pub gamefield_height: usize,
    pub maximum_items_on_ground: isize,
}

impl Config {
    pub fn new(
        gamefield_width: usize,
        gamefield_height: usize,
        maximum_items_on_ground: isize,
    ) -> Config {
        Config {
            gamefield_width,
            gamefield_height,
            maximum_items_on_ground,
        }
    }

    /// By default, gamefield width and height are both 10,
    /// and maximum number of items on ground is limited to 8.
    pub fn default() -> Config {
        Config {
            gamefield_width: 10,
            gamefield_height: 10,
            maximum_items_on_ground: 8,
        }
    }
}

pub struct Karel {
    gamefield: Vec<isize>,
    karel_position: (usize, usize),
    karel_orientation: Direction,
    configuration: Config,
}

impl Karel {
    pub fn new(configuration: Config) -> Karel {
        Karel {
            gamefield: Vec::with_capacity(
                configuration.gamefield_width * configuration.gamefield_height,
            ),
            karel_orientation: Direction::North,
            karel_position: (0, 0),
            configuration,
        }
    }

    /// Try to access gamefield as if it was 2D array.
    /// On success, return value on target index.
    /// On failure (because of out of bounds), return
    /// error with accessed index that was out of bounds.
    fn get_gamefield(&self, coords: (usize, usize)) -> Result<isize, usize> {
        let index: usize = self.configuration.gamefield_height * coords.0 + coords.1;
        match self.gamefield.get(index) {
            Some(num) => Ok(*num),
            None => Err(index),
        }
    }

    /// Try to set value on gamefield as if it was 2D array.
    /// On success, nothing will be returned.
    /// On failure (because of out of bounds), return
    /// error with accessed index that was out of bounds.
    fn set_gamefield(&mut self, coords: (usize, usize), value: isize) -> Result<(), usize> {
        let index: usize = self.configuration.gamefield_height * coords.0 + coords.1;
        match self.gamefield.get_mut(index) {
            Some(num) => {
                *num = value;
                Ok(())
            }
            None => Err(index),
        }
    }

    /// Try to build/destroy wall on target tile.
    /// This might fail in several ways, see
    /// `ToggleWallError` enum.
    pub fn toggle_wall(&mut self, coords: (usize, usize)) -> Result<(), ToggleWallError> {
        match &self.get_gamefield(coords) {
            Ok(number) => {
                if number > &0 {
                    // There is an item here
                    return Err(ToggleWallError::ItemOnGround);
                }
                if self.karel_position == coords {
                    // There is karel here
                    return Err(ToggleWallError::KarelIsHere);
                }

                // Toggle the wall
                if number == &-1 {
                    &self.set_gamefield(coords, 0);
                } else {
                    &self.set_gamefield(coords, -1);
                }

                Ok(())
            }
            Err(_) => Err(ToggleWallError::OutOfBounds),
        }
    }

    pub fn query(&self, query: Query) -> Result<bool, QueryError> {
        match query {
            Query::Direction(direction) => Ok(enum_variant_eq(&direction, &self.karel_orientation)),
            Query::ItemHere => match &self.get_gamefield(self.karel_position) {
                Ok(num) => Ok(num > &0),
                Err(err) => Err(QueryError::OutOfBounds),
            },
            Query::WallInFrontOfMe => {
                let lookup_result = match &self.karel_orientation {
                    Direction::North => {
                        self.get_gamefield((self.karel_position.0, self.karel_position.1 - 1))
                    }
                    Direction::South => {
                        self.get_gamefield((self.karel_position.0, self.karel_position.1 + 1))
                    }
                    Direction::West => {
                        self.get_gamefield((self.karel_position.0 + 1, self.karel_position.1))
                    }
                    Direction::East => {
                        self.get_gamefield((self.karel_position.0 - 1, self.karel_position.1))
                    }
                };
                match lookup_result {
                    Ok(num) => Ok(num == -1),
                    Err(_) => Err(QueryError::OutOfBounds),
                }
            }
        }
    }

    pub fn action(&mut self, action: Action) -> Result<(), ActionError> {
        match action {
            Move => match &self.query(Query::WallInFrontOfMe) {
                Ok(true) => Err(ActionError::MoveWall),
                Ok(false) => {
                    self.karel_position = match &self.karel_orientation {
                        Direction::North => (self.karel_position.0, self.karel_position.1 - 1),
                        Direction::South => (self.karel_position.0, self.karel_position.1 + 1),
                        Direction::West => (self.karel_position.0 + 1, self.karel_position.1),
                        Direction::East => (self.karel_position.0 - 1, self.karel_position.1),
                    };
                    Ok(())
                }
                Err(QueryError::OutOfBounds) => Err(ActionError::MoveOutOfBounds),
            },
            PlaceItem => match &self.get_gamefield(self.karel_position) {
                Ok(number) => {
                    if number >= &self.configuration.maximum_items_on_ground {
                        return Err(ActionError::ExceedItemLimit);
                    } else {
                        &self.set_gamefield(self.karel_position, number + 1).unwrap();
                        return Ok(());
                    }
                }
                Err(_) => Err(ActionError::MoveOutOfBounds),
            },
            RemoveItem => match &self.get_gamefield(self.karel_position) {
                Ok(number) => {
                    if number == &0 {
                        return Err(ActionError::NoItemHere);
                    } else {
                        &self.set_gamefield(self.karel_position, number - 1).unwrap();
                        return Ok(());
                    }
                }
                Err(_) => Err(ActionError::MoveOutOfBounds),
            },
            TurnLeft => {
                match &self.karel_orientation {
                    Direction::North => {
                        self.karel_orientation = Direction::West;
                    }
                    Direction::West => {
                        self.karel_orientation = Direction::South;
                    }
                    Direction::South => {
                        self.karel_orientation = Direction::East;
                    }
                    Direction::East => {
                        self.karel_orientation = Direction::North;
                    }
                };
                Ok(())
            }
        }
    }
}

/// Enum that describes actions that can karel do
pub enum Action {
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
pub enum Query {
    /// Returns true if there is a wall in front of Karel
    WallInFrontOfMe,
    /// Returns true if there is at least one item on current tile
    ItemHere,
    /// Returns true if Karel is facing asked direction
    Direction(Direction),
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Enum that describes various errors that can be caused by
/// ordering Karel do do an action.
pub enum ActionError {
    /// This error means that Karel was ordered to run into wall.
    MoveWall,
    /// This error means that Karel was ordered to move out of game field.
    MoveOutOfBounds,
    /// This error means that Karel was ordered to place an item on tile which
    /// already contains maximum amount of items.
    ExceedItemLimit,
    /// This error means that Karel was ordered to pick up an item on tile, which
    /// has no items on it.
    NoItemHere,
}

/// Enum that describes various errors that can be caused by querying Karel.
pub enum QueryError {
    /// This error means that Karel doesn't know how to answer a query, because he's
    /// asked about something that is not on the map. This is typically use as a response to invalid
    /// `WallInFrontOfMe` query.
    OutOfBounds,
}

/// Enum that describes various errors that can be caused by
/// building a wall on a wrong place.
pub enum ToggleWallError {
    /// You cannot build wall here while there is some item on ground. Remove it first.
    ItemOnGround,
    /// You cannot build wall here while karel is still there. Move him first.
    KarelIsHere,
    /// You cannot build wall out of the gameplay area.
    OutOfBounds,
}

/// Check if two enum variants are the same ones
/// https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
fn enum_variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
