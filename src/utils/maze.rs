use std::fmt;

/// This struct is used to represent a position in the maze
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    /// The x coordinate
    pub x: i32,
    /// The y coordinate
    pub y: i32,
}

impl Coordinate {
    /// This function is used to create a new coordinate
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Coordinate {
    /// This function is used to get the coordinate north of the current coordinate
    pub fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    /// This function is used to get the coordinate east of the current coordinate
    pub fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    /// This function is used to get the coordinate south of the current coordinate
    pub fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    /// This function is used to get the coordinate west of the current coordinate
    pub fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    /// This function is used to check if the coordinate is valid
    pub fn is_valide(&self, maze: &Maze) -> bool {
        self.x < maze.width && self.x >= 0 && self.y < maze.height && self.y >= 0
    }
}

/// This structure represent a cell of the maze
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    /// The coordinate of the cell
    pub coordinate: Coordinate,
    /// This field is used to check if the cell is visited
    pub visited: bool,
    /// This field is used to represent the walls of the cell
    pub walls: [bool; 4],
}

impl Cell {
    /// This function is used to create a new cell
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            coordinate: Coordinate::new(x, y),
            visited: false,
            walls: [true; 4],
        }
    }
}

/// This structure represent direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// This variant represent the north direction
    North,
    /// This variant represent the east direction
    East,
    /// This variant represent the south direction
    South,
    /// This variant represent the west direction
    West,
}

impl From<i32> for Direction {
    fn from(direction: i32) -> Self {
        match direction {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Direction {
    /// This function is used to get the opposite of a [Direction]
    ///
    /// # Return
    ///
    /// The opposite direction of self
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Cell {
    fn format(&self) -> Vec<String> {
        let mut cell_content = vec![vec!['#'; 3]; 3];
        cell_content[1][1] = '*';
        if !self.walls[Direction::North as usize] {
            cell_content[0][1] = '*';
        }
        if !self.walls[Direction::South as usize] {
            cell_content[2][1] = '*';
        }
        if !self.walls[Direction::West as usize] {
            cell_content[1][0] = '*';
        }
        if !self.walls[Direction::East as usize] {
            cell_content[1][2] = '*';
        }
        vec![
            cell_content[0].iter().collect::<String>(),
            cell_content[1].iter().collect::<String>(),
            cell_content[2].iter().collect::<String>(),
        ]
    }
}

/// This structure represent a maze
#[derive(Clone, Debug)]
pub struct Maze {
    /// This field represent the width of the maze
    pub width: i32,
    /// This field represent the height of the maze
    pub height: i32,
    /// This field represent the maze as a 2D vector
    pub maze: Vec<Vec<Cell>>,
}

impl Maze {
    /// This function is used to create a new maze
    pub fn new(width: i32, height: i32) -> Self {
        let mut maze = Vec::new();
        for y in 0..height {
            let mut line = Vec::new();
            for x in 0..width {
                line.push(Cell::new(x, y));
            }
            maze.push(line);
        }
        Self {
            maze,
            height,
            width,
        }
    }
    /// This function is used to check if the given coordinate is visited
    ///
    /// # Argument
    ///
    /// * `self` - The maze to check [Maze]
    /// * `coordinate` - The coordinate to check [Coordinate]
    ///
    /// # Return
    ///
    /// True if the coordinate is invalid or if it is already visited
    /// Else return false
    pub fn is_visited(&self, coordinate: &Coordinate) -> bool {
        if !coordinate.is_valide(self) {
            true
        } else {
            self.maze[coordinate.y as usize][coordinate.x as usize].visited
        }
    }
    /// This function is used to visit a cell [Cell]
    ///
    /// # Argument
    ///
    /// * `self` - The maze to use [Maze]
    /// * `coordinate` - The coordinate to visit [Coordinate]
    pub fn visit(&mut self, coordinate: &Coordinate) {
        if coordinate.is_valide(self) {
            self.maze[coordinate.y as usize][coordinate.x as usize].visited = true;
        }
    }
    /// This function is used to get a cell at the given coordinate
    ///
    /// # Argument
    ///
    /// * `self` - The maze to use [Maze]
    /// * `coordinate` - The coordinate to get the cell [Coordinate]
    ///
    /// # Return
    ///
    /// If the coordinate is invalid return [None]
    /// Else return the Cell [Cell]
    pub fn get_cell(&self, coordinate: &Coordinate) -> Option<Cell> {
        if coordinate.is_valide(self) {
            Some(self.maze[coordinate.y as usize][coordinate.x as usize])
        } else {
            None
        }
    }
    /// This function is used to remove the wall of the Cell at the given coordinate
    /// The direction will determine which wall will be removed
    ///
    /// # Argument
    ///
    /// * `self` - The maze to use [Maze]
    /// * `coordinate` - The coordinate to get the cell [Coordinate]
    /// * `direction` - The direction to remove the wall [Direction]
    pub fn remove_wall(&mut self, coordinate: &Coordinate, direction: &Direction) {
        let cell = self.get_cell(coordinate);
        if let Some(mut new_cell) = cell {
            new_cell.walls[*direction as usize] = false;
            self.maze[coordinate.y as usize][coordinate.x as usize] = new_cell;
        }
    }
    /// This function is used to get all valid and not visited neighbours of the [Cell] at the given [Coordinate]
    ///
    /// # Argument
    ///
    /// * `self` - The maze to use [Maze]
    /// * `coordinate` - The coordinate to get the cell [Coordinate]
    ///
    /// # Return
    ///
    /// A list of valid and not visited coordinate and the list of corespond direction
    pub fn get_neighbours(&self, coordinate: &Coordinate) -> (Vec<Coordinate>, Vec<Direction>) {
        let mut neighbours = Vec::new();
        let mut directions = Vec::new();
        let north = coordinate.north();
        let east = coordinate.east();
        let south = coordinate.south();
        let west = coordinate.west();
        if north.is_valide(self) && !self.is_visited(&north) {
            neighbours.push(north);
            directions.push(Direction::North);
        }
        if east.is_valide(self) && !self.is_visited(&east) {
            neighbours.push(east);
            directions.push(Direction::East);
        }
        if south.is_valide(self) && !self.is_visited(&south) {
            neighbours.push(south);
            directions.push(Direction::South);
        }
        if west.is_valide(self) && !self.is_visited(&west) {
            neighbours.push(west);
            directions.push(Direction::West);
        }
        (neighbours, directions)
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Maze of size {}x{}", self.width, self.height)?;
        for line in &self.maze {
            let mut first_line = String::new();
            let mut second_line = String::new();
            let mut third_line = String::new();
            for cell in line {
                let cell_contant = cell.format();
                first_line.push_str(&cell_contant[0]);
                second_line.push_str(&cell_contant[1]);
                third_line.push_str(&cell_contant[2]);
            }
            writeln!(f, "{first_line}\n{second_line}\n{third_line}")?;
        }
        Ok(())
    }
}
