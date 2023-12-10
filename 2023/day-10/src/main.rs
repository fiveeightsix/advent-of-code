fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");

    let grid = Grid::from_input(input);

    let total = find_distance(grid)?;
    
    println!("{}", total / 2);

    Ok(())
}

pub fn find_distance(grid: Grid) -> Result<i32, Box<dyn std::error::Error>> {
    let starting = grid.start_tile_connections()?;
    
    assert!(starting.len() == 2);

    let mut prev = starting[0].clone();

    let mut distance: i32 = 1;
    
    let total = loop {
        println!("prev: {:?}", prev);
        
        let prev_tile = grid.get_tile(&prev.point);

        println!("prev tile: {:?}", prev_tile);
        
        let next_direction = tile_connections(prev_tile)
            .into_iter()
            .filter(|d: &Direction| *d != prev.from)
            .next()
            .unwrap();

        let next_point = grid.get_neighbour(&prev.point, next_direction)
            .ok_or(format!("Can't find next point from {:?}", prev))?;

        let next_tile = grid.get_tile(&next_point);

        println!("next point: {:?}", next_point);
        println!("next tile: {:?}", next_tile);

        distance += 1;
        
        if next_tile == 'S' {
            break distance
        } else {
            prev = PointRecord::new(next_point, next_direction.opposite());
        }
    };

    Ok(total)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointRecord {
    point: Point,
    from: Direction,
}

impl PointRecord {
    pub fn new(point: Point, from: Direction) -> PointRecord {
        PointRecord { point, from }
    }
}

#[derive(Debug)]
pub struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input.split("\n")
            .map(|s| s.chars().collect())
            .filter(|v: &Vec<char>| v.len() > 0)
            .collect();

        // Assume this is going to be a regular size
        let width = tiles[0].len();
        let height = tiles.len();

        Grid {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile(&self, point: &Point) -> char {
        self.tiles[point.y][point.x]
    }

    pub fn get_neighbour(&self, tile: &Point, direction: Direction) -> Option<Point> {
        assert!(tile.x < self.width, "x not in grid");
        assert!(tile.y < self.height, "y not in grid");

        match direction {
            Direction::North => if tile.y == 0 { None } else { Some(Point::new(tile.x, tile.y - 1)) },
            Direction::East => if tile.x == self.width - 1 { None } else { Some(Point::new(tile.x + 1, tile.y)) },
            Direction::South => if tile.y == self.height - 1 { None } else { Some(Point::new(tile.x, tile.y + 1)) }
            Direction::West => if tile.x == 0 { None } else { Some(Point::new(tile.x - 1, tile.y)) }
        }
    }

    pub fn find_start(&self) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y][x] == 'S' {
                    return Some(Point::new(x, y));
                }
            }
        }

        None
    }

    pub fn start_tile_connections(&self) -> Result<Vec<PointRecord>, Box<dyn std::error::Error>> {
        let mut connections = Vec::new();

        let start = self.find_start().ok_or("Couldn't find start tile")?;

        match self.get_neighbour(&start, Direction::North) {
            Some(p) => {
                if tile_connections(self.get_tile(&p)).contains(&Direction::South) {
                    connections.push(PointRecord::new(p, Direction::South));
                }
            },
            _ => {}
        }

        match self.get_neighbour(&start, Direction::East) {
            Some(p) => {
                if tile_connections(self.get_tile(&p)).contains(&Direction::West) {
                    connections.push(PointRecord::new(p, Direction::West));
                }
            },
            _ => {}
        }

        match self.get_neighbour(&start, Direction::South) {
            Some(p) => {
                if tile_connections(self.get_tile(&p)).contains(&Direction::North) {
                    connections.push(PointRecord::new(p, Direction::North));
                }
            },
            _ => {}
        }

        match self.get_neighbour(&start, Direction::West) {
            Some(p) => {
                if tile_connections(self.get_tile(&p)).contains(&Direction::East) {
                    connections.push(PointRecord::new(p, Direction::East));
                }
            },
            _ => {}
        }

        Ok(connections)
    }
}

pub fn tile_connections(tile: char) -> Vec<Direction> {
    match tile {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::East, Direction::South],
        '.' => vec![],
        _ => panic!("{}", format!("Unrecognised character: {:?}", tile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creates() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....";
        
        let grid = Grid::from_input(input);

        assert!(grid.width == 5);
        assert!(grid.height == 5);
        
        assert_eq!(grid.find_start(), Some(Point::new(1, 1)));
    }

    #[test]
    fn get_neighbour() {
        let input = r"...
...
...";
        
        let grid = Grid::from_input(input);

        let top_mid = Point::new(1, 0);
        
        assert_eq!(grid.get_neighbour(&top_mid, Direction::North), None);
        assert_eq!(grid.get_neighbour(&top_mid, Direction::East), Some(Point::new(2, 0)));
        assert_eq!(grid.get_neighbour(&top_mid, Direction::South), Some(Point::new(1, 1)));
        assert_eq!(grid.get_neighbour(&top_mid, Direction::West), Some(Point::new(0, 0)));

        let bottom_left = Point::new(0, 2);

        assert_eq!(grid.get_neighbour(&bottom_left, Direction::North), Some(Point::new(0, 1)));
        assert_eq!(grid.get_neighbour(&bottom_left, Direction::East), Some(Point::new(1, 2)));
        assert_eq!(grid.get_neighbour(&bottom_left, Direction::South), None);
        assert_eq!(grid.get_neighbour(&bottom_left, Direction::West), None);
    }

    #[test]
    fn start_tile_connections() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....";
        
        let grid = Grid::from_input(input);

        let expected = vec![
            PointRecord::new(Point::new(2, 1), Direction::West),
            PointRecord::new(Point::new(1, 2), Direction::North)
        ];

        let result = grid.start_tile_connections();

        assert!(result.is_ok());

        let connections = result.unwrap();
        
        assert_eq!(expected.len(), connections.len());

        println!("{:?}", connections);
        
        assert!(expected.iter().all(|item| connections.contains(item)));
    }
    
    #[test]
    fn start_tile_connections_more() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let grid = Grid::from_input(input);

        let expected = vec![
            PointRecord::new(Point::new(1, 2), Direction::West),
            PointRecord::new(Point::new(0, 3), Direction::North)
        ];

        let result = grid.start_tile_connections();

        assert!(result.is_ok());

        let connections = result.unwrap();
        
        assert_eq!(expected.len(), connections.len());

        println!("{:?}", connections);
        
        assert!(expected.iter().all(|item| connections.contains(item)));
    }

    
    #[test]
    fn find_distance_works() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....";
        
        let grid = Grid::from_input(input);

        let result = find_distance(grid);

        assert!(result.is_ok());

        let distance = result.unwrap();

        assert_eq!(4, distance / 2);
    }

    #[test]
    fn find_distance_works_odd() {
        let input = r".....
.S--7
.|..|
.L--J
.....";
        
        let grid = Grid::from_input(input);

        let result = find_distance(grid);

        assert!(result.is_ok());

        let distance = result.unwrap();

        assert_eq!(5, distance / 2);
    }

    #[test]
    fn find_distance_works_more() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let grid = Grid::from_input(input);

        let result = find_distance(grid);

        assert!(result.is_ok());

        let distance = result.unwrap();

        assert_eq!(8, distance / 2);
    }
}
