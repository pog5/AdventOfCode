#[derive(Debug, Clone, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Guard {
    coordinate: Coordinate,
    facing: Facing,
}

impl Guard {
    fn new(coordinate: Coordinate, facing: Facing) -> Guard {
        Guard {
            coordinate: coordinate,
            facing: facing,
        }
    }

    fn peek(&self) -> Coordinate {
        match self.facing {
            Facing::Up => Coordinate { x: self.coordinate.x - 1, y: self.coordinate.y },
            Facing::Down => Coordinate { x: self.coordinate.x + 1, y: self.coordinate.y },
            Facing::Left => Coordinate { x: self.coordinate.x, y: self.coordinate.y - 1 },
            Facing::Right => Coordinate { x: self.coordinate.x, y: self.coordinate.y + 1 },
        }
    }

    fn move_forward(&mut self) {
        match self.facing {
            Facing::Up => self.coordinate.x -= 1,
            Facing::Down => self.coordinate.x += 1,
            Facing::Left => self.coordinate.y -= 1,
            Facing::Right => self.coordinate.y += 1,
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

    fn as_string(&self) -> String {
        match self.facing {
            Facing::Up => "^".to_string(),
            Facing::Down => "v".to_string(),
            Facing::Left => "<".to_string(),
            Facing::Right => ">".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Obstacle {
    coordinate: Coordinate,
}

impl Obstacle {
    fn new(coordinate: Coordinate) -> Obstacle {
        Obstacle {
            coordinate: coordinate,
        }
    }

    fn as_string(&self) -> String {
        "#".to_string()
    }
}

#[derive(Debug, Clone)]
struct Tile {
    guard: Option<Guard>,
    obstacle: Option<Obstacle>,
    was_visited: bool,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            guard: None,
            obstacle: None,
            was_visited: false
        }
    }

    fn as_string(&self) -> String {
        match self.guard {
            Some(ref guard) => guard.as_string(),
            None => match self.obstacle {
                Some(ref obstacle) => obstacle.as_string(),
                None => ".".to_string(),
            },
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            tiles: vec![vec![Tile::new(); width]; height],
        }
    }

    fn as_string(&self) -> String {
        let mut string = String::new();
        for x in 0..self.height {
            for y in 0..self.width {
                string.push_str(&self.tiles[x][y].as_string());
            }
            string.push_str("\n");
        }
        string
    }

    fn get_tile(&self, coordinate: Coordinate) -> &Tile {
        &self.tiles[coordinate.x][coordinate.y]
    }

    fn get_tile_mut(&mut self, coordinate: Coordinate) -> &mut Tile {
        &mut self.tiles[coordinate.x][coordinate.y]
    }

    fn get_guard(&self) -> &Guard {
        self.tiles.iter()
            .flat_map(|x| x.iter())
            .find(|tile| tile.guard.is_some()).expect("No guard on map")
            .guard.as_ref().unwrap()
    }

    fn get_guard_mut(&mut self) -> &mut Guard {
        self.tiles.iter_mut()
            .flat_map(|x| x.iter_mut())
            .find(|tile| tile.guard.is_some()).expect("No guard on map")
            .guard.as_mut().unwrap()
    }

    fn is_guard_in_bounds(&self) -> bool {
        let guard = self.get_guard();
        guard.coordinate.x < self.height && guard.coordinate.y < self.width
    }

    fn tick(&mut self) {
        let peek_coordinate = self.get_guard().peek();
        let has_obstacle = self.tiles[peek_coordinate.x][peek_coordinate.y].obstacle.is_some();

        let guard = self.get_guard_mut();
        if has_obstacle {
            guard.turn_right();
        } else {
            guard.move_forward();
        }

    }
}

fn part1(input: &str) -> String {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut grid = Grid::new(width, height);

    // grid setup
    let _ = input.lines().enumerate().map(|(x, line)| {
        let _ = line.as_bytes().iter().enumerate().map(|(y, char)| {
            match char {
                b'^' => {
                    grid.tiles[x][y].guard = Some(Guard::new(Coordinate { x: x, y: y }, Facing::Up));
                },
                b'v' => {
                    grid.tiles[x][y].guard = Some(Guard::new(Coordinate { x: x, y: y }, Facing::Down));
                },
                b'<' => {
                    grid.tiles[x][y].guard = Some(Guard::new(Coordinate { x: x, y: y }, Facing::Left));
                },
                b'>' => {
                    grid.tiles[x][y].guard = Some(Guard::new(Coordinate { x: x, y: y }, Facing::Right));
                },
                b'#' => {
                    grid.tiles[x][y].obstacle = Some(Obstacle::new(Coordinate { x: x, y: y }));
                },
                _ => (),
            }
        }).collect::<Vec<_>>();
    }).collect::<Vec<_>>();

    let mut distinct_tiles = Vec::new();
    distinct_tiles.push(grid.get_guard().coordinate.clone());

    while grid.is_guard_in_bounds() {
        // The code WILL panic if the guard goes out of bounds
        // regardless, the last outputted value is correct.
        grid.tick();

        if !distinct_tiles.contains(&grid.get_guard().coordinate) {
            distinct_tiles.push(grid.get_guard().coordinate.clone());
        }

        eprintln!("{:?}", grid.get_guard().coordinate);
        eprintln!("{}", distinct_tiles.len());
    }

    distinct_tiles.len().to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
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