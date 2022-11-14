use crate::utils::maze::{Cell, Coordinate, Direction, Maze};
use rand::Rng;
use std::ops::Range;

fn gen_coordinate(maze: &Maze, coordinate: &Coordinate) -> (Option<Coordinate>, Option<Direction>) {
    let (valid_neighbours, valid_direction) = maze.get_neighbours(coordinate);
    if valid_neighbours.is_empty() {
        return (None, None);
    }
    let mut rng = rand::thread_rng();
    let index = rng.gen_range::<usize, Range<usize>>(0..valid_neighbours.len());
    (Some(valid_neighbours[index]), Some(valid_direction[index]))
}

impl Maze {
    /// This function is used to make a maze perfect
    pub fn generate(&mut self) -> Self {
        let mut visited_cell = vec![Cell {
            visited: true,
            ..Cell::new(0, 0)
        }];
        loop {
            let current_coord = visited_cell.last().unwrap().coordinate;
            let (coordinate, direction) = gen_coordinate(self, &current_coord);
            match coordinate {
                Some(next_coordinate) => {
                    let direction = direction.unwrap();
                    self.remove_wall(&current_coord, &direction);
                    self.remove_wall(&next_coordinate, &direction.opposite());
                    self.visit(&current_coord);
                    self.visit(&next_coordinate);
                    visited_cell.push(self.get_cell(&next_coordinate).unwrap());
                }
                None => {
                    visited_cell.pop();
                }
            }
            if visited_cell.is_empty() {
                drop(visited_cell);
                return self.clone();
            }
        }
    }
}
