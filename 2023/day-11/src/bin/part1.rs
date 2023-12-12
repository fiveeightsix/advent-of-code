use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let mut image = Image::from_input(input);

    println!("total galaxies: {}", image.galaxies.len());
    println!("dimensions: {} x {}", image.width, image.height);

    image.expand();

    println!("total galaxies after expansion: {}", image.galaxies.len());
    println!("dimensions after expansion: {} x {}", image.width, image.height);

    println!("{}", image.get_total_distance());
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

    pub fn distance_to(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Path {
    from: Point,
    to: Point,
    distance: usize,
}

impl Path {
    pub fn new(from: Point, to: Point) -> Self {
        let distance = from.distance_to(&to);
        
        Path { from, to, distance }
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    galaxies: Vec<Point>
}

impl Image {
    pub fn from_input(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut galaxies = vec![];

        for (y, row) in input.split_terminator("\n").enumerate() {
            height = y + 1;
            
            for (x, c) in row.chars().enumerate() {
                width = x + 1;
                
                match c {
                    '#' => galaxies.push(Point::new(x, y)),
                    _ => {}
                }
            }
        }

        Self { width, height, galaxies }
    }

    fn is_empty_column(&self, x: usize) -> bool {
        self.galaxies.iter().all(|g| g.x != x)
    }

    fn is_empty_row(&self, y: usize) -> bool {
        self.galaxies.iter().all(|g| g.y != y)
    }

    fn expand_column(&mut self, x: usize) {
        self.width += 1;
            
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.x > x {
                galaxy.x += 1;
            }
        }
    }

    fn expand_row(&mut self, y: usize) {
        self.height += 1;
        
        for galaxy in self.galaxies.iter_mut() {
            if galaxy.y > y {
                galaxy.y += 1;
            }
        }
    }

    pub fn expand(&mut self) {
        let mut x = 0;
        
        while x < self.width {
            if self.is_empty_column(x) {
                self.expand_column(x);
                x += 2; // skip over expanded column
            } else {
                x += 1;
            }
        }

        let mut y = 0;
        
        while y < self.height {
            if self.is_empty_row(y) {
                self.expand_row(y);
                y += 2; // skip over expanded row
            } else {
                y += 1;
            }
        }
    }

    pub fn get_all_paths(&self) -> Vec<Path> {
        let mut paths = Vec::new();
        
        for (from, to) in self.galaxies.iter().tuple_combinations() {
            paths.push(Path::new(from.clone(), to.clone()));
        }

        paths
    }

    pub fn get_total_distance(&self) -> usize {
        self.get_all_paths()
            .iter()
            .map(|path| path.distance)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance_to() {
        let p1 = Point::new(4, 0);
        let p7 = Point::new(9, 10);

        assert_eq!(15, p1.distance_to(&p7));
        assert_eq!(15, p7.distance_to(&p1));

        let p3 = Point::new(0, 2);
        let p6 = Point::new(12, 7);

        assert_eq!(17, p3.distance_to(&p6));
        assert_eq!(17, p6.distance_to(&p3));

        let p8 = Point::new(0, 11);
        let p9 = Point::new(5, 11);

        assert_eq!(5, p8.distance_to(&p9));
        assert_eq!(5, p9.distance_to(&p8));
    }

    #[test]
    fn image_from_input() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let image = Image::from_input(input);

        assert_eq!(10, image.width);
        assert_eq!(10, image.height);

        let expected_galaxies = vec![
            Point::new(3, 0),
            Point::new(7, 1),
            Point::new(0, 2),
            Point::new(6, 4),
            Point::new(1, 5),
            Point::new(9, 6),
            Point::new(7, 8),
            Point::new(0, 9),
            Point::new(4, 9),
        ];

        assert_eq!(expected_galaxies, image.galaxies);
    }

    #[test]
    fn image_expand() {
        let expected_input = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

        let expected_image = Image::from_input(expected_input);

        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let mut image = Image::from_input(input);

        image.expand();

        assert_eq!(expected_image, image);        
    }

    #[test]
    fn image_get_all_paths() {
        let input = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

        let image = Image::from_input(input);

        assert_eq!(36, image.get_all_paths().len());

        assert_eq!(374, image.get_total_distance());
    }
}
