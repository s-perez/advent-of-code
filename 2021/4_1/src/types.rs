#[derive(Default)]
struct Cell {
    number: u8,
    marked: bool
}

impl Cell {
    pub fn new(number: u8) -> Cell {
        Cell {
            number: number,
            marked: false
        }
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn mark(&mut self, ball: u8) {
        if self.number == ball {
            self.marked = true
        }
    }
}


struct Board {
    numbers: [Cell;25]
}

impl Board {

    pub fn new(numbers: [u8;25]) -> Board {
        let board = numbers
            .map(|number| Cell::new(number))
            .collect();

        Board {
            numbers: board

        }
    }

    pub fn mark(&mut self, ball: u8) {
        self.numbers
            .iter_mut()
            .flatten()
            .map(|cell| cell.mark(ball))
            .collect::<()>();
    }

    fn check_rows_for_win(&self) -> bool {
        true
    }

    fn check_columns_for_win(&self) -> bool {
        false
    }
}


struct TestCase {
    pub boards: Vec<Board>,
    pub balls: Vec<u8>
}

impl TestCase {
    pub fn solve() {}
}
