mod util;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

// Type for (row, column)
type Position = (u32, u32);

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

// Private methods that are not exported.

impl Cell {
  fn toggle(&mut self) {
    *self = match *self {
      Cell::Dead => Cell::Alive,
      Cell::Alive => Cell::Dead
    };
  }
}

impl Universe {
  fn get_index(&self, pos: Position) -> usize {
    assert!(pos.0 < self.height);
    assert!(pos.1 < self.width);
    (pos.1 + pos.0 * self.width) as usize
  }
}

impl Universe {
  pub fn neighbors(&self, pos: Position) -> Vec<Cell> {
    let mut out = Vec::with_capacity(8);
    self.each_neighbor_pos(pos, |neighbor| {
      out.push(neighbor);
    });
    out
  }

  pub fn each_neighbor_pos<F>(&self, pos: Position, mut f: F)
  where
    F: FnMut(Cell),
  {
    for delta_row in [self.height - 1, 0, 1] {
      for delta_col in [self.width - 1, 0, 1] {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }
        let neighbor_row = (pos.0 + delta_row) % self.height;
        let neighbor_col = (pos.1 + delta_col) % self.width;
        let index = self.get_index((neighbor_row, neighbor_col));
        f(self.cells[index])
      }
    }
  }

  fn live_neighbor_count(&self, pos: Position) -> u32 {
    return self
      .neighbors(pos)
      .iter()
      .filter(|cell| matches!(cell, Cell::Alive))
      .count() as u32;
  }
}

impl Universe {
  /// Get the dead and alive values of the entire universe.
  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  /// Set cells to be alive in a univers by passing the row and column
  /// of each cell as an array.
  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index((row, col));
      self.cells[idx] = Cell::Alive;
    }
  }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
      self.cells.as_ptr()
  }

  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
  }

  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
  }

  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index((row, column));
    self.cells[idx].toggle();
  }

  pub fn new() -> Universe {
    util::set_panic_hook();
    let width = 64;
    let height = 64;

    kyle_log!(
      "New universe[{}, {}]",
      width,
      height
    );

    let cells = (0..width * height)
      .map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index((row, col));
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count((row, col));

        next[idx] = match (cell, live_neighbors) {
          // Rule 1. Any live cell with fewer than two live
          // neighbours dies, as if by underpopulation.
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          // Rule 2. Any live cell with two or three live
          // neighbours lives on to the next generation.
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          // Rule 3. Any live cell with more than three live
          // neighbours dies, as if by overpopulation.
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          // Rule 4. Any dead cell with exactly three live
          // neighbours becomes a live cell, as if by reproduction.
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise,
        };
      }
    }
    self.cells = next;
  }
}

use std::fmt;

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn live_neighbor_count_all() {
    let universe = Universe {
      width: 3,
      height: 3,
      cells: vec![
        Cell::Alive, Cell::Alive, Cell::Alive,
        Cell::Alive, Cell::Dead, Cell::Alive,
        Cell::Alive, Cell::Alive, Cell::Alive,
      ],
    };
    let count = universe.live_neighbor_count((1, 1));
    assert_eq!(8, count);
  }

  #[test]
  fn live_neighbor_count_left_corner() {
    let universe = Universe {
      width: 3,
      height: 3,
      cells: vec![
        Cell::Dead, Cell::Dead, Cell::Alive,
        Cell::Dead, Cell::Dead, Cell::Alive,
        Cell::Dead, Cell::Dead, Cell::Alive,
      ],
    };

    assert_eq!(3, universe.live_neighbor_count((0, 0)));
  }

  #[test]
  fn get_index_first_row() {
    let universe = Universe {
      width: 4,
      height: 1,
      cells: vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
    };

    assert_eq!(2, universe.get_index((0, 2)))
  }

  #[test]
  fn get_index_second_row() {
    let universe = Universe {
      width: 2,
      height: 2,
      cells: vec![
        Cell::Dead, Cell::Dead,
        Cell::Dead, Cell::Dead
      ],
    };

    assert_eq!(3, universe.get_index((1, 1)))
  }
}
