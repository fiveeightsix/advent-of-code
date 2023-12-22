use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    todo!()
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

#[derive(Debug)]
struct CityMap {
    blocks: Vec<u32>,
    width: usize,
    height: usize,
}

impl CityMap {
    pub fn from_input(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        
        let blocks: Vec<u32> = input
            .split_terminator("\n")
            .flat_map(|row| {
                width = row.len();
                height += 1;

                row.chars().filter_map(|c| c.to_digit(10))
            })
            .collect();

        Self { width, height, blocks }
    }

    pub fn get(&self, p: &Point) -> u32 {
        self.blocks[p.y * self.width + p.x]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city_map_from_input() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let city_map = CityMap::from_input(&input);
        
        println!("{:?}", city_map);

        assert_eq!(13, city_map.width);
        assert_eq!(13, city_map.height);
    }
}
