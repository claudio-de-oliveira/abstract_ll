#[allow(dead_code)]
pub struct ScannerEnv<'a> {
    current_row: usize,
    current_col: usize,
    vtext: Vec<&'a str>,
}

#[allow(dead_code)]
impl<'a> ScannerEnv<'a> {
    pub fn new(vtext: Vec<&'a str>) -> Self {
        ScannerEnv {
            current_row: 0,
            current_col: 0,
            vtext,
        }
    }

    #[inline]
    pub fn get_current_row(&self) -> usize {
        self.current_row
    }

    #[inline]
    pub fn get_current_col(&self) -> usize {
        self.current_col
    }

    #[inline]
    pub fn end_of_text(&self) -> bool {
        self.current_row == self.vtext.len() - 1
            && self.current_col == self.vtext[self.current_row].len()
    }

    pub fn retract(&mut self) {
        if self.current_col > 0 {
            self.current_col -= 1;
        } else if self.current_row > 0 {
            self.current_row -= 1;
            self.current_col = self.vtext[self.current_row].len() - 1;
        }
    }

    pub fn next_char(&mut self) -> char {
        if self.current_col >= self.vtext[self.current_row].len() {
            self.current_row += 1;
            self.current_col = 0;
            return '\n';
        }

        let ch = self.vtext[self.current_row].chars().nth(self.current_col).unwrap();
        self.current_col += 1;

        ch
    }

}
