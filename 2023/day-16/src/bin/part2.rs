use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part2(input));
}

fn part2(input: &str) -> usize {
    let contraption = Contraption::from_input(input);

    let mut possible_starts: Vec<(usize, usize, Direction)> = Vec::new();

    possible_starts.extend((0..contraption.width).map(|x| (x, 0, Direction::South)));
    possible_starts.extend((0..contraption.width).map(|x| (x, contraption.height - 1, Direction::North)));
    possible_starts.extend((0..contraption.height).map(|y| (0, y, Direction::East)));
    possible_starts.extend((0..contraption.height).map(|y| (contraption.width - 1, y, Direction::West)));

    possible_starts
        .into_iter()
        .map(|(x, y, direction)| {
            let mut beam_state = BeamState::new(&contraption);
            
            println!("running {}, {}, {:?}", x, y, direction);
            
            beam_state.set_start(x, y, direction);
            beam_state.run();

            beam_state.count_energised()
        })
        .max()
        .unwrap_or(0)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct BeamSegment {
    location: Point,
    direction: Direction,
}

impl BeamSegment {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { location: Point::new(x, y), direction }
    }
}

#[derive(Debug)]
struct Contraption {
    pub tiles: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Contraption {
    pub fn from_input(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input
            .split_terminator("\n")
            .map(|row| row.chars().collect())
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        Self { tiles, width, height }
    }

    pub fn get_tile(&self, point: &Point) -> char {
        assert!(
            point.x < self.width && point.y < self.height,
            "Point lies outside of tile set"
        );

        self.tiles[point.y][point.x].clone()
    }

    fn segment_next(&self, from: &Point, direction: Direction) -> Option<BeamSegment> {
        match direction {
            d @ Direction::North if from.y > 0 => {
                Some(BeamSegment::new(from.x, from.y - 1, d))
            },
            d @ Direction::East if from.x < (self.width - 1) => {
                Some(BeamSegment::new(from.x + 1, from.y, d))
            },
            d @ Direction::South if from.y < (self.height - 1) => {
                Some(BeamSegment::new(from.x, from.y + 1, d))
            },
            d @ Direction::West if from.x > 0 => {
                Some(BeamSegment::new(from.x - 1, from.y, d))
            },
            _ => None
        }
    }

    pub fn beam_next(&self, segment: &BeamSegment) -> Vec<BeamSegment> {
        let tile = self.get_tile(&segment.location);

        let next_directions = match tile {
            // Empty tiles allow the beam to continue in whatever its
            // current direction of travel is.
            '.' => vec![segment.direction.clone()],
            // Mirrors are double-sided, and reflect at 90 degree angles
            // (note the `\` character needs to be escaped).
            '\\' => match segment.direction {
                Direction::North => vec![Direction::West],
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
            },
            '/' => match segment.direction {
                Direction::North => vec![Direction::East],
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
            },
            // Splitters send the beam in two directions if entered from the
            // side, otherwise they allow the beam to continue.
            '|' => match segment.direction {
                Direction::East | Direction::West => vec![Direction::North, Direction::South],
                d => vec![d.clone()]
            },
            '-' => match segment.direction {
                Direction::North | Direction::South => vec![Direction::East, Direction::West],
                d => vec![d.clone()]
            },
            _ => panic!("Unrecognised tile")
        };

        next_directions
            .iter()
            .filter_map(|&direction| self.segment_next(&segment.location, direction))
            .collect()
    }
}

#[derive(Debug)]
struct BeamState<'a> {
    contraption: &'a Contraption,
    active: VecDeque<BeamSegment>,
    visited: Vec<BeamSegment>,
}

impl<'a> BeamState<'a> {
    pub fn new(contraption: &'a Contraption) -> Self {
        Self {
            contraption,
            active: VecDeque::new(),
            visited: Vec::new(),
        }
    }

    pub fn set_start(&mut self, x: usize, y: usize, direction: Direction) {
        self.active.push_back(BeamSegment::new(x, y, direction));
    }

    pub fn run(&mut self) {
        while !self.active.is_empty() {
            let current = self.active.pop_front().unwrap();

            let next = self.contraption.beam_next(&current);

            for segment in next.iter() {
                // Prevent loops
                if !self.visited.contains(&segment) {
                    self.active.push_back(segment.clone());
                }
            }

            self.visited.push(current.clone());
        }
    }

    fn get_energised(&self) -> Vec<Vec<usize>> {
        let mut energised: Vec<Vec<usize>> = (0..self.contraption.height)
            .map(|_| (0..self.contraption.width).map(|_| 0).collect())
            .collect();

        for segment in &self.visited {
            if energised[segment.location.y][segment.location.x] == 0 {
                energised[segment.location.y][segment.location.x] += 1;
            }
        }

        energised
    }

    pub fn render_energised(&self) -> String {
        let energised = self.get_energised();
        
        let mut output = String::new();
        
        for y in 0..self.contraption.height {
            for x in 0..self.contraption.width {
                if energised[y][x] > 0 {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }

            output.push('\n');
        }
        
        output
    }

    pub fn count_energised(&self) -> usize {
        self.get_energised()
            .into_iter()
            .flatten()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contraption_new() {
        let input = r"./-\|
-|\/.
/.-\.
";
        let contraption = Contraption::from_input(&input);

        assert_eq!(5, contraption.width);
        assert_eq!(3, contraption.height);
    }

    #[test]
    fn contraption_get_tile() {
        let input = r".|-/\
|-\/.
-/\-.
";
        let contraption = Contraption::from_input(&input);

        assert_eq!('.', contraption.get_tile(&Point::new(0, 0)));
        assert_eq!('|', contraption.get_tile(&Point::new(1, 0)));
        assert_eq!('|', contraption.get_tile(&Point::new(0, 1)));
        assert_eq!('-', contraption.get_tile(&Point::new(1, 1)));
        assert_eq!('/', contraption.get_tile(&Point::new(1, 2)));
        assert_eq!('\\', contraption.get_tile(&Point::new(2, 1)));
        assert_eq!('/', contraption.get_tile(&Point::new(3, 1)));
    }
    
    #[test]
    fn beam_state_render_energised_works() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
        let expected = r"######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
";

        let contraption = Contraption::from_input(&input);

        let mut beam_state = BeamState::new(&contraption);

        beam_state.set_start(0, 0, Direction::East);

        beam_state.run();

        assert_eq!(expected, beam_state.render_energised());
    }

    #[test]
    fn beam_state_count_energised_works() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

        let contraption = Contraption::from_input(&input);

        let mut beam_state = BeamState::new(&contraption);

        beam_state.set_start(0, 0, Direction::East);

        beam_state.run();

        assert_eq!(46, beam_state.count_energised());
    }

    #[test]
    fn part2_works() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

        assert_eq!(51, part2(&input));
    }
}
