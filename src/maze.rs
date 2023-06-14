pub struct Walls {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl Walls {
    pub fn new() -> Self {
        Walls {
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }

    pub fn remove_left(&mut self) {
        self.left = false;
    }
}

pub struct Cell {
    col: usize,
    row: usize,
    visited: bool,
    walls: Walls,
}

pub struct Maze {
    cols: usize,
    rows: usize,
    cells: Vec<Cell>,
}

impl Maze {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut cells = Vec::new();

        for i in 0..(cols * rows) - 1 {
            let walls = Walls::new();
            let cell = Cell {
                col: i,
                row: i,
                walls,
                visited: false,
            };

            cells.push(cell);
        }

        Maze { cols, rows, cells }
    }

    pub fn dfs(&self) {
        let mut stack: Vec<usize> = vec![0];
        let mut current;
        let mut cell;

        while !stack.is_empty() {
            current = stack.pop().unwrap();
            cell = self.cells.get(current).unwrap();

            match self.random_unvisited_neighbor(cell) {
                Some(neighbor) => {
                    stack.push(current);

                    let mut my_cell = &mut cell;
                    self.remove_walls_bettwen(&mut my_cell, neighbor);

                    neighbor.visited = true;
                    stack.push(self.index(&neighbor));
                }
                None => {}
            };
        }
    }

    fn random_unvisited_neighbor(&self, cell: &Cell) -> Option<&mut Cell> {
        None
    }

    fn remove_walls_bettwen(&self, a: &mut Cell, b: &mut Cell) {
        let dx = a.col - b.col;
        let dy = a.row - b.row;

        if dx < 0 {
            a.walls.remove_left();
            a.walls.right = false;
            b.walls.left = false;
        } else if dx > 0 {
            a.walls.left = false;
            a.walls.right = false;
        }

        if dy < 0 {
            a.walls.bottom = false;
            b.walls.top = false;
        } else if dy > 0 {
            a.walls.top = false;
            b.walls.bottom = false;
        }
    }

    fn index(&self, cell: &Cell) -> usize {
        return cell.col + cell.row * self.cols;
    }
}
