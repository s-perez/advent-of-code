use arrayvec::ArrayVec;


#[derive(Default, Copy, Clone)]
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

    pub fn mark(&mut self, ball: &u8) {
        if self.number == *ball {
            self.marked = true
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    numbers: [Cell;25]
}

impl Board {

    pub fn new(numbers: Vec<u8>) -> Board {
        let board: ArrayVec<Cell, 25> = 
            IntoIterator::into_iter(numbers)
            .map(|number| Cell::new(number))
            .collect();

        Board {
            numbers: board.into_inner().unwrap_or_else(|_| panic!("Array not completely filled"))

        }
    }

    pub fn mark(&mut self, ball: &u8) {
        self.numbers
            .iter_mut()
            .map(|cell| cell.mark(ball))
            .for_each(drop);
    }

    pub fn did_already_win(&self) -> bool {
        self.check_rows_for_win() || self.check_columns_for_win()
    }

    pub fn calculate_points(&self, last_ball: u32) -> u32 {
        self.numbers
            .iter()
            .filter(|cell| !cell.is_marked())
            .fold(0 as u32, |acc, cell| acc + cell.get_number() as u32) * last_ball
    }

    fn get_rows(&self) -> Vec<[Cell; 5]> {
        (0..5)
            .map(|row| [self.numbers[row * 5], self.numbers[row * 5 + 1], self.numbers[row * 5 + 2], self.numbers[row * 5 + 3], self.numbers[row * 5 + 4]])
            .collect()
    }

    fn get_columns(&self) -> Vec<[Cell; 5]> {
        (0..5)
            .map(|col| [self.numbers[col], self.numbers[col + 5], self.numbers[col + 10], self.numbers[col + 15], self.numbers[col + 20]])
            .collect()
    }

    fn check_rows_for_win(&self) -> bool {
        self.get_rows()
            .iter()
            .any(|row| row.iter().all(|cell| cell.is_marked()))
    }

    fn check_columns_for_win(&self) -> bool {
        self.get_columns()
            .iter()
            .any(|column| column.iter().all(|cell| cell.is_marked()))
    }
}


pub struct TestCase {
    pub boards: Vec<Board>,
    pub balls: Vec<u8>
}

impl TestCase {
    pub fn new(numbers: Vec<Vec<u8>>, balls: Vec<u8>) -> TestCase{
        TestCase {
            boards: numbers.into_iter().map(Board::new).collect(),
            balls: balls
        }
    }

    pub fn solve(&mut self) -> u32 {
        let mut last_ball: u8 = 0;
        self.balls
            .clone()
            .into_iter()
            .take_while(|x| {
                self.boards.iter_mut().for_each(|board| board.mark(x));
                self.boards.iter().filter(|board| !board.did_already_win()).count() > 1
            })
            .for_each(drop);

        let last_board: &mut Board = self.boards.iter_mut().find(|board| !board.did_already_win()).unwrap();
        self.balls
            .clone()
            .into_iter()
            .take_while(|x| {
                last_board.mark(x);
                last_ball = *x;
                !last_board.did_already_win()
            })
            .for_each(drop);
        last_board.calculate_points(last_ball as u32)
    }
}
