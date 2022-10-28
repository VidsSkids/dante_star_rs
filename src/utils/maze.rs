use std::fmt;

/// This structure represent a cell of the maze
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    /// This field represent the cell x position in the maze
    pub x: i32,
    /// This field represent the cell y position in the maze
    pub y: i32,
    /// This field is used to check if the cell is visited
    pub visited: bool,
    /// This field is used to represent the walls of the cell
    pub walls: [bool; 4],
}

impl Cell {
    /// This function is used to create a new cell
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visited: false,
            walls: [true; 4],
        }
    }
}

/// This structure represent direction
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
