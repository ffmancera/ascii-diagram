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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for line in self.canvas.iter() {
            let mut parsed_line = line.iter().collect::<String>();
            parsed_line.push_str("\n");
            result.push_str(parsed_line.as_str());
        }

        return result;
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_canvas() {
        let mut canvas = crate::Canvas::new(1, 5);
        let expected_res = String::from("     \n");
        let result = canvas.to_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_with_blank_chars() {
        let mut canvas = crate::Canvas::new(1, 10);
        let expected_res = String::from("    +---+ \n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 4), 5);
        let result = canvas.to_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_with_empty_line() {
        let mut canvas = crate::Canvas::new(2, 5);
        let expected_res = String::from("+---+\n     \n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 0), 5);
        let result = canvas.to_string();

        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_horizontal_line() {
        let mut canvas = crate::Canvas::new(1, 5);
        let expected_res = String::from("+---+\n");

        canvas.draw_horizontal_line('+', '-', '+', (0, 0), 5);
        let result = canvas.to_string();

        assert_eq!(result, expected_res);
    }
}
