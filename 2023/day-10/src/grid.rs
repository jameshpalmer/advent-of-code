pub struct Grid {
    pipes: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input: String) -> Grid {
        let mut pipes: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            pipes.push(row);
        }
        Grid { pipes }
    }

    fn get_starting_point(&self) -> (usize, usize) {
        for (row, line) in self.pipes.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return (row, col);
                }
            }
        }
        panic!("No starting point found");
    }

    fn get_next_point(
        &self,
        curr_point: (usize, usize),
        prev_point: (usize, usize),
    ) -> (usize, usize) {
        let (row_usize, col_usize) = curr_point;
        let (row, col) = (row_usize as i32, col_usize as i32);

        let curr_pipe = self.pipes[curr_point.0][curr_point.1];
        let moves: Vec<((i32, i32), char)> = [
            ((row, col + 1), ['-', 'F', 'L'], 'S'),
            ((row, col - 1), ['-', 'J', '7'], 'S'),
            ((row + 1, col), ['|', 'F', '7'], 'S'),
            ((row - 1, col), ['|', 'J', 'L'], 'S'),
            ((row, col + 1), ['-', 'F', 'L'], '-'),
            ((row, col - 1), ['-', 'J', '7'], '-'),
            ((row + 1, col), ['|', 'F', '7'], '|'),
            ((row - 1, col), ['|', 'J', 'L'], '|'),
            ((row - 1, col), ['|', 'J', 'L'], 'F'),
            ((row, col - 1), ['-', 'J', '7'], 'F'),
            ((row - 1, col), ['|', 'J', 'L'], '7'),
            ((row, col + 1), ['-', 'F', 'L'], '7'),
            ((row + 1, col), ['|', 'F', '7'], 'L'),
            ((row, col - 1), ['-', 'J', '7'], 'L'),
            ((row + 1, col), ['|', 'F', '7'], 'J'),
            ((row, col + 1), ['-', 'F', 'L'], 'J'),
        ]
        .iter()
        .filter(|&&((next_row, next_col), source_symbols, _pipe)| {
            next_row >= 0
                && next_col >= 0
                && next_row < self.pipes.len() as i32
                && next_col < self.pipes[0].len() as i32
                && (source_symbols.contains(&curr_pipe) || curr_pipe == 'S')
        })
        .map(|&(next_point, _source_symbols, pipe)| (next_point, pipe))
        .collect();

        for &(next_point, next_pipe) in &moves {
            if self
                .pipes
                .get(next_point.0 as usize)
                .and_then(|row: &Vec<char>| row.get(next_point.1 as usize))
                == Some(&next_pipe)
                && (next_point.0 != (prev_point.0 as i32) || next_point.1 != (prev_point.1 as i32))
            {
                return (next_point.0 as usize, next_point.1 as usize);
            }
        }

        panic!("No next point found");
    }

    fn get_loop_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        points.push(self.get_starting_point());
        points.push(self.get_next_point(points[0], points[0]));

        while *points.last().unwrap() != points[0] {
            points.push(self.get_next_point(*points.last().unwrap(), points[points.len() - 2]));
        }

        points
    }

    pub fn get_furthest_point(&self) -> usize {
        let points = self.get_loop_points();
        return points.len() / 2;
    }

    pub fn count_interior_points(&self) -> usize {
        // remove last point, which is the same as the first point
        let mut loop_points = self.get_loop_points();
        loop_points.pop();
        for row in 0..self.pipes.len() {
            let row_loop_points = loop_points
                .iter()
                .filter(|(loop_row, _loop_col)| *loop_row == row)
                .map(|(_loop_row, loop_col)| *loop_col)
                .collect::<Vec<usize>>();

            println!("{:?}", row_loop_points);
        }

        0
    }
}
