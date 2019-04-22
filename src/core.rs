use std::fmt;

pub struct Config {
    pub gamefield_width: usize,
    pub gamefield_height: usize,
    pub maximum_items_on_ground: isize,
}

impl Config {
    /// Create new config with parameters.
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
    karel_coordinates: (usize, usize),
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
            karel_coordinates: (0, 0),
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
                if self.karel_coordinates == coords {
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
            Query::ItemHere => match &self.get_gamefield(self.karel_coordinates) {
                Ok(num) => Ok(num > &0),
                Err(err) => Err(QueryError::OutOfBounds),
            },
            Query::WallInFrontOfMe => {
                let lookup_result = match &self.karel_orientation {
                    Direction::North => {
                        if self.karel_coordinates.1 == 0 {
                            return Err(QueryError::OutOfBounds);
                        }
                        self.get_gamefield((self.karel_coordinates.0, self.karel_coordinates.1 - 1))
                    }
                    Direction::South => {
                        self.get_gamefield((self.karel_coordinates.0, self.karel_coordinates.1 + 1))
                    }
                    Direction::West => {
                        self.get_gamefield((self.karel_coordinates.0 + 1, self.karel_coordinates.1))
                    }
                    Direction::East => {
                        if self.karel_coordinates.0 == 0 {
                            return Err(QueryError::OutOfBounds);
                        }
                        self.get_gamefield((self.karel_coordinates.0 - 1, self.karel_coordinates.1))
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
            Action::Move => match &self.query(Query::WallInFrontOfMe) {
                Ok(true) => Err(ActionError::MoveWall),
                Ok(false) => {
                    self.karel_coordinates = match &self.karel_orientation {
                        Direction::North => {
                            (self.karel_coordinates.0, self.karel_coordinates.1 - 1)
                        }
                        Direction::South => {
                            (self.karel_coordinates.0, self.karel_coordinates.1 + 1)
                        }
                        Direction::West => (self.karel_coordinates.0 + 1, self.karel_coordinates.1),
                        Direction::East => (self.karel_coordinates.0 - 1, self.karel_coordinates.1),
                    };
                    Ok(())
                }
                Err(QueryError::OutOfBounds) => Err(ActionError::MoveOutOfBounds),
            },
            Action::PlaceItem => match &self.get_gamefield(self.karel_coordinates) {
                Ok(number) => {
                    if number >= &self.configuration.maximum_items_on_ground {
                        return Err(ActionError::ExceedItemLimit);
                    } else {
                        &self
                            .set_gamefield(self.karel_coordinates, number + 1)
                            .unwrap();
                        return Ok(());
                    }
                }
                Err(_) => Err(ActionError::MoveOutOfBounds),
            },
            Action::RemoveItem => match &self.get_gamefield(self.karel_coordinates) {
                Ok(number) => {
                    if number == &0 {
                        return Err(ActionError::NoItemHere);
                    } else {
                        &self
                            .set_gamefield(self.karel_coordinates, number - 1)
                            .unwrap();
                        return Ok(());
                    }
                }
                Err(_) => Err(ActionError::MoveOutOfBounds),
            },
            Action::TurnLeft => {
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

    /// Borrow a readonly gamemap. The gamemap is 1D array, which is accessed as if it was
    /// 2D array. The index is computed this way:
    ///
    /// `let index: usize = configuration.gamefield_height * coords.0 + coords.1;`
    ///
    /// It contains numbers. Here is what they mean:
    ///
    /// `-1 => Wall`
    ///
    /// `X => (where X is >= 0) on this tile lies X items`
    ///
    /// If you want to get karel's position and rotation, use `read_karel`.
    pub fn read_gamemap(&self) -> &Vec<isize> {
        &self.gamefield
    }

    /// Borrow information where karel stands (x, y) and which direction is he facing.
    ///
    /// Returns a tuple. First item of tuple contains reference to pair of usize (in another tuple),
    /// which represents current coordinates. Second item of tuple is reference to current Karel direction.
    pub fn read_karel(&self) -> (&(usize, usize), &Direction) {
        (&self.karel_coordinates, &self.karel_orientation)
    }

    /// Read karel gamefield and print into STDOUT.
    /// This loops over all fields and prints them.
    ///
    /// `0 (empty) => .`
    /// `1+ (number of items on ground) => 1+`
    /// `-1 (wall) => #`
    /// ` (karel) =>  ▼ OR ▲ OR ◀ OR ▶ (depends on orientation)`
    pub fn print_karel(&self) {
        for i in 0..self.configuration.gamefield_width {
            for j in 0..self.configuration.gamefield_height {
                if self.karel_coordinates.0 == i && self.karel_coordinates.1 == j {
                    print!(
                        "{}",
                        match self.karel_orientation {
                            Direction::West => "◀",
                            Direction::North => "▲",
                            Direction::South => "▼",
                            Direction::East => "▶",
                        }
                    );
                } else {
                    match self.get_gamefield((i, j)).unwrap() {
                        0 => print!("."),
                        -1 => print!("#"),
                        x if x > 0 => print!("{}", x),
                        x => panic!("Unexpected field read while printing at ({}, {}) : {}", i, j, x),
                    };
                }
            }
            println!();
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

impl<'a> fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionError::MoveWall => write!(f, "Karel was ordered to run into a wall. Karel will terminate."),
            ActionError::MoveOutOfBounds => write!(f, "Karel was ordered to run out of the map. Karel will terminate."),
            ActionError::ExceedItemLimit => write!(f, "Karel exceeded item limit while placing an item. Karel will terminate."),
            ActionError::NoItemHere => write!(f, "Karel tried to pick up item, but there was none there. Karel will terminate."),
        }
    }
}

/// Enum that describes various errors that can be caused by querying Karel.
pub enum QueryError {
    /// This error means that Karel doesn't know how to answer a query, because he's
    /// asked about something that is not on the map. This is typically use as a response to invalid
    /// `WallInFrontOfMe` query.
    OutOfBounds,
}

impl<'a> fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryError::OutOfBounds => write!(f, "Karel tried to look forward if there is a wall, but there was end of map. Karel will terminate."),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// If this test fails, we need to change documentation
    fn config_default() {
        let config = Config::default();
        assert_eq!(config.gamefield_width, 10);
        assert_eq!(config.gamefield_height, 10);
        assert_eq!(config.maximum_items_on_ground, 8);
    }

    #[test]
    fn config_new() {
        let config = Config::new(12, 8, 4);
        assert_eq!(config.gamefield_width, 12);
        assert_eq!(config.gamefield_height, 8);
        assert_eq!(config.maximum_items_on_ground, 4);
    }

    #[test]
    fn karel_initialization() {
        let config = Config::default();
        let karel = Karel::new(config);
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::North));
        assert_eq!(karel.read_karel().0, &(0, 0));
    }

    #[test]
    fn karel_rotate() {
        let mut karel = Karel::new(Config::default());
        assert!(karel.action(Action::TurnLeft).is_ok());
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::West));
        assert!(karel.action(Action::TurnLeft).is_ok());
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::South));
        assert!(karel.action(Action::TurnLeft).is_ok());
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::East));
        assert!(karel.action(Action::TurnLeft).is_ok());
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::North));
        assert!(karel.action(Action::TurnLeft).is_ok());
        assert!(enum_variant_eq(karel.read_karel().1, &Direction::West));
    }

    #[test]
    fn outofbounds_fail() {
        let mut karel = Karel::new(Config::default());
        assert!(enum_variant_eq(
            &QueryError::OutOfBounds,
            &karel.query(Query::WallInFrontOfMe).unwrap_err()
        ));
        assert!(enum_variant_eq(
            &ActionError::MoveOutOfBounds,
            &karel.action(Action::Move).unwrap_err()
        ));
    }

    #[test]
    fn enum_variant_equals() {
        assert!(enum_variant_eq(&Direction::South, &Direction::South));
        assert_eq!(
            false,
            enum_variant_eq(&ActionError::ExceedItemLimit, &ActionError::MoveOutOfBounds)
        );
        assert!(enum_variant_eq(
            &Query::Direction(Direction::South),
            &Query::Direction(Direction::North)
        ));
        assert!(enum_variant_eq(
            &Query::Direction(Direction::West),
            &Query::Direction(Direction::West)
        ));
    }
}
