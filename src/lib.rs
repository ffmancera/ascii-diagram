#[derive(Debug)]
pub struct Canvas {
    canvas: Vec<Vec<char>>,
}

impl Canvas {
    pub fn new(line_num: usize, line_len: usize) -> Canvas {
        Canvas {
            canvas: vec![vec![' '; line_len]; line_num],
        }
    }

    pub fn get_string(&self) -> String {
        let mut result = String::new();
        for line in self.canvas.iter() {
            let mut parsed_line = line.iter().collect::<String>();
            parsed_line.push('\n');
            result.push_str(parsed_line.as_str());
        }

        result
    }

    pub fn draw_horizontal_line(
        &mut self,
        init_char: char,
        middle_char: char,
        last_char: char,
        start_position: (usize, usize),
        len: usize,
    ) {
        self.canvas[start_position.0][start_position.1] = init_char;
        for i in start_position.1 + 1..start_position.1 + len - 1 {
            self.canvas[start_position.0][i] = middle_char;
        }
        self.canvas[start_position.0][start_position.1 + len - 1] = last_char;
    }

    pub fn draw_vertical_line(
        &mut self,
        init_char: char,
        middle_char: char,
        last_char: char,
        start_position: (usize, usize),
        len: usize,
    ) {
        self.canvas[start_position.0][start_position.1] = init_char;
        for i in start_position.0 + 1..start_position.0 + len - 1 {
            self.canvas[i][start_position.1] = middle_char;
        }
        self.canvas[start_position.0 + len - 1][start_position.1] = last_char;
    }

    pub fn draw_square(&mut self, start_position: (usize, usize), len: usize) {
        let last_h_pos = (start_position.0 + len - 1, start_position.1);
        let last_v_pos = (start_position.0, start_position.1 + len - 1);

        self.draw_horizontal_line('+', '-', '+', start_position, len);
        self.draw_vertical_line('+', '|', '+', start_position, len);
        self.draw_horizontal_line('+', '-', '+', last_h_pos, len);
        self.draw_vertical_line('+', '|', '+', last_v_pos, len);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_canvas() {
        let canvas = crate::Canvas::new(1, 5);
        let expected_res = String::from("     \n");
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_with_blank_chars() {
        let mut canvas = crate::Canvas::new(1, 10);
        let expected_res = String::from("    +---+ \n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 4), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_with_empty_line() {
        let mut canvas = crate::Canvas::new(2, 5);
        let expected_res = String::from("+---+\n     \n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 0), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_line() {
        let mut canvas = crate::Canvas::new(1, 5);
        let expected_res = String::from("+---+\n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 0), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_vertical_with_blank_chars() {
        let mut canvas = crate::Canvas::new(10, 1);
        let expected_res = String::from(" \n \n \n \n+\n|\n|\n|\n+\n \n");

        canvas.draw_vertical_line('+', '|', '+', (4, 0), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_vertical_with_empty_column() {
        let mut canvas = crate::Canvas::new(10, 2);
        let expected_res = String::from("  \n  \n  \n  \n+ \n| \n| \n| \n+ \n  \n");

        canvas.draw_vertical_line('+', '|', '+', (4, 0), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_vertical_line() {
        let mut canvas = crate::Canvas::new(5, 1);
        let expected_res = String::from("+\n|\n|\n|\n+\n");

        canvas.draw_vertical_line('+', '|', '+', (0, 0), 5);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_square() {
        let mut canvas = crate::Canvas::new(4, 4);
        let expected_res = String::from("+--+\n|  |\n|  |\n+--+\n");

        canvas.draw_square((0, 0), 4);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_square_empty_lines() {
        let mut canvas = crate::Canvas::new(6, 4);
        let expected_res = String::from("    \n+--+\n|  |\n|  |\n+--+\n    \n");

        canvas.draw_square((1, 0), 4);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_square_empty_lines_columns() {
        let mut canvas = crate::Canvas::new(6, 6);
        let expected_res = String::from("      \n +--+ \n |  | \n |  | \n +--+ \n      \n");

        canvas.draw_square((1, 1), 4);
        let result = canvas.get_string();

        assert_eq!(result, expected_res);
    }
}
