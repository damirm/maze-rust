use crate::maze::Maze;

mod maze;

fn main() {
    let maze = Maze::new(100, 100);

    maze.dfs();
}
