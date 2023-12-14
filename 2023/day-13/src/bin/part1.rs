fn main() {
    let input = include_str!("input.txt");

    let patterns: Vec<(Vec<usize>, Vec<usize>)> = input
        .split_terminator("\n\n")
        .map(|i| Pattern::from_input(i))
        .map(|p| get_reflections(&p))
        .collect();

    println!("{:?}", patterns);
}

fn str_as_bin_int(string: &str) -> usize {
    usize::from_str_radix(string, 2).expect("Not a valid binary number")
}

#[derive(Debug)]
pub struct Pattern {
    width: usize,
    height: usize,
    columns: Vec<usize>,
    rows: Vec<usize>,
}

impl Pattern {
    pub fn from_input(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input
            .split_terminator("\n")
            .map(|row| row
                 .chars()
                 .map(|c| match c {
                     '#' => '1',
                     _ => '0'
                 })
                 .collect()
            )
            .collect();
        
        let width = tiles[0].len();
        let height = tiles.len();

        let mut columns = vec![];
        
        for x in 0..width {
            let mut column = String::new();
            
            for y in 0..height {
                column.push(tiles[y][x]);
            }

            columns.push(str_as_bin_int(&column));
        }

        let mut rows = vec![];
        
        for y in 0..height {
            let mut row = String::new();

            for x in 0..width {
                row.push(tiles[y][x]);
            }

            rows.push(str_as_bin_int(&row));
        }
        
        Pattern {
            width,
            height,
            columns,
            rows,
        }
    }
}
 
fn is_reflection_before_point(line: &Vec<usize>, point: usize) -> bool {
    if point == 0 || point >= line.len() {
        return false;
    }
    
    let mut left = point.clone() - 1;
    let mut right = point.clone();

    loop {
        if line[left] != line[right] {
            // This can't be a reflection if the elements are different.
            break false;
        } else if left == 0 || right == line.len() - 1 {
            // We have reached either the start or end of the line without
            // failing equality so this can be considered a reflection.
            break true;
        } else {
            // Otherwise, expand the test window equally either side of
            // the reflection point.
            left -= 1;
            right += 1;
        }
    }
}

fn get_reflections(pattern: &Pattern) -> (Vec<usize>, Vec<usize>) {
    let mut column_reflections = vec![];

    for x in 0..pattern.columns.len() {
        if is_reflection_before_point(&pattern.columns, x) {
           column_reflections.push(x);
        }
    }

    let mut row_reflections = vec![];

    for y in 0..pattern.rows.len() {
        if is_reflection_before_point(&pattern.rows, y) {
            row_reflections.push(y);
        }
    }

    println!("columns: {:?}, ref: {:?} -- rows: {:?}, ref: {:?}", pattern.columns, column_reflections, pattern.rows, row_reflections);

    (column_reflections, row_reflections)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_reflection_before_point_returns_false_at_ends() {
        //              0><1  2  3  4  5  6  7><8
        let line = vec![2, 2, 1, 1, 1, 1, 1, 3, 3];

        assert_eq!(false, is_reflection_before_point(&line, 0));
        assert_eq!(true, is_reflection_before_point(&line, 1));
        assert_eq!(false, is_reflection_before_point(&line, 7));
        assert_eq!(true, is_reflection_before_point(&line, 8));
    }
    
    #[test]
    fn is_reflection_before_point_low_match() {
        // Reflection occurs:
        //              0  1  2><3  4  5  6  7  8
        let line = vec![1, 2, 3, 3, 2, 1, 7, 8, 9];

        assert_eq!(false, is_reflection_before_point(&line, 2));
        assert_eq!(true, is_reflection_before_point(&line, 3));
        assert_eq!(false, is_reflection_before_point(&line, 4));
    }

    #[test]
    fn is_reflection_before_point_high_match() {
        // Reflection occurs:
        //              0  1  2  3  4  5><6  7  8
        let line = vec![1, 2, 3, 4, 5, 6, 6, 5, 4];

        assert_eq!(false, is_reflection_before_point(&line, 5));
        assert_eq!(true, is_reflection_before_point(&line, 6));
        assert_eq!(false, is_reflection_before_point(&line, 7));
    }
}
