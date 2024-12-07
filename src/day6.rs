use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Guard {
    x: usize,
    y: usize,
    facing: Facing,
}

impl Guard {
    fn new(x: usize, y: usize, facing: Facing) -> Guard {
        Guard {
            x: x,
            y: y,
            facing: facing,
        }
    }

    fn peek(&mut self, grid: &Grid) -> bool {
        match self.facing {
            Facing::Up => grid.tiles[self.x - 1][self.y],
            Facing::Down => grid.tiles[self.x + 1][self.y],
            Facing::Left => grid.tiles[self.x][self.y - 1],
            Facing::Right => grid.tiles[self.x][self.y + 1],
        }
    }

    fn move_forward(&mut self) {
        match self.facing {
            Facing::Up => self.x -= 1,
            Facing::Down => self.x += 1,
            Facing::Left => self.y -= 1,
            Facing::Right => self.y += 1,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<bool>>,
}

fn is_guard_inside(guard: &Guard, grid: &Grid) -> bool {
    let above_zero = !(guard.facing == Facing::Up && guard.x <= 0 ||
        guard.facing == Facing::Left && guard.y <= 0 ||
        guard.facing == Facing::Down && guard.x >= grid.height ||
        guard.facing == Facing::Right && guard.y >= grid.width);

    guard.x < grid.width && guard.y < grid.height && above_zero
}

fn part1_internal(input: &str) -> HashSet<(usize, usize)> {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut grid = Grid { width, height, tiles: vec![vec![false; width]; height] };
    let mut guard = Guard::new(usize::MAX, usize::MAX, Facing::Up);

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.as_bytes().iter().enumerate() {
            match char {
                b'^' => guard = Guard::new(x, y, Facing::Up),
                b'#' => grid.tiles[x][y] = true,
                &_ => {}
            }
        };
    };

    let mut distinct_tiles = HashSet::new();
    distinct_tiles.insert((guard.x, guard.y));

    while is_guard_inside(&guard, &grid) {

        if guard.peek(&grid) {
            guard.turn_right();
        } else {
            guard.move_forward();
        }

        distinct_tiles.insert((guard.x, guard.y));
    }

    distinct_tiles
}

fn part1(input: &str) -> String {
    part1_internal(input).len().to_string()
}

fn part2(input: &str) -> String {
    let distinct_tiles = part1_internal(input);
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut temp_grid = Grid { width, height, tiles: vec![vec![false; width+1]; height+1] };
    let mut guard = Guard::new(usize::MAX, usize::MAX, Facing::Up);

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.as_bytes().iter().enumerate() {
            match char {
                b'^' => guard = Guard::new(x, y, Facing::Up),
                b'#' => temp_grid.tiles[x][y] = true,
                &_ => {}
            }
        };
    };
    let grid = temp_grid.clone();

    let mut blockable_positions = HashSet::new();

    for (x, y) in distinct_tiles {
        let mut new_tmpgrid = grid.clone();
        new_tmpgrid.tiles[x][y] = true;
        let new_grid = new_tmpgrid.clone();
        let mut new_guard = guard.clone();

        let mut visited_spots = HashSet::new();
        while is_guard_inside(&new_guard, &new_grid) {
            let state = ((new_guard.x, new_guard.y), new_guard.facing.clone());
            if new_guard.peek(&new_grid) {
                if !visited_spots.insert(state) {
                    blockable_positions.insert((x, y));
                    break;
                }
                new_guard.turn_right();
            } else {
                new_guard.move_forward();
            }
        }
    }

    blockable_positions.len().to_string()
}

pub fn day6() {
    let input = include_str!("../input/day6.txt");
    let response = part1(input);
    println!();
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}