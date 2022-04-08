use array2d::Array2D;

#[derive(PartialEq)]
pub struct Game {
    width: usize,
    height: usize,
    previous_generation: Array2D<bool>,
    new_generation: Array2D<bool>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        // Input validation todo!
        Game {
            width: width,
            height: height,
            previous_generation: Array2D::filled_with(false, height, width),
            new_generation: Array2D::filled_with(false, height, width),
        }
    }

    pub fn next_generation(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.new_generation.set(i, j, self.evolution_cycle(i, j)).unwrap();
            }
        }
        for i in 0..self.height {
            for j in 0..self.height {
                self.previous_generation.set(i,j, *self.new_generation.get(i, j).unwrap()).unwrap();
            }
        }
    }

    // Customize rules in this?
    pub fn evolution_cycle(&self, row: usize, col: usize) -> bool {
        let count: i32 = self.count_neighbors(row, col);
        if count == 2 || count == 3 {
            return true;
        } else {
            return false;
        }
    }

    pub fn count_neighbors(&self, row: usize, col: usize) -> i32 {
        let mut count: i32 = 0;
        let mut new_row: i32;
        let mut new_col: i32;
        for i in 0..3 {
            for j in 0..3 {

                if i == 1 && j == 1 {
                    continue;
                }

                new_row = row as i32;
                if new_row + i as i32 - 1 < 0 {
                    new_row = self.height as i32 - 1;
                } else if new_row + i as i32 - 1 >= self.height as i32 {
                    new_row = 0;
                } else {
                    new_row = new_row + i as i32 - 1;
                }

                new_col = col as i32;
                if new_col + j as i32 - 1 < 0 {
                    new_col = self.width as i32 - 1;
                } else if new_col + j as i32 - 1 >= self.width as i32 {
                    new_col = 0;
                } else {
                    new_col = new_col + j as i32 - 1;
                }

                if *self.previous_generation.get(new_row as usize, new_col as usize).unwrap() {
                    count += 1;
                }

            }
        }
        return count;
    }
}