use rand::seq::SliceRandom;
use rand::RngExt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Master,
}

impl Difficulty {
    fn get_remove_count(&self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 50,
            Difficulty::Hard => 60,
            Difficulty::Master => 70,
        }
    }

    pub fn next(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Master,
            Difficulty::Master => Difficulty::Easy,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Sudoku {
    pub grid: [[Option<u8>; 9]; 9],
    pub fixed: [[bool; 9]; 9],
}

type SudokuGrid = [[Option<u8>; 9]; 9];

//TODO: Per i livelli Hard e Master, implementare un algoritmo di generazione più sofisticato che garantisca un'unica soluzione e una maggiore difficoltà.
//TODO: Oppure implementare un resolver prima di darlo al giocatore, e se il resolver impiega troppo tempo, rigenerare un nuovo puzzle.
impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            grid: [[None; 9]; 9],
            fixed: [[false; 9]; 9],
        }
    }

    pub fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        Self::is_valid_pos(&self.grid, row, col, num)
    }

    fn is_valid_pos(puzzle: &SudokuGrid, row: usize, col: usize, num: u8) -> bool {
        let num_opt = Some(num);

        if (0..9).any(|i| {
            (i != col && puzzle[row][i] == num_opt) || (i != row && puzzle[i][col] == num_opt)
        }) {
            return false;
        }

        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                if puzzle[start_row + r][start_col + c] == num_opt {
                    return false;
                }
            }
        }
        true
    }

    fn fill_grid(puzzle: &mut SudokuGrid) -> bool {
        let mut rng = rand::rng();

        for r in 0..9 {
            for c in 0..9 {
                if puzzle[r][c].is_none() {
                    let mut numbers: Vec<u8> = (1..10).collect();
                    numbers.shuffle(&mut rng);

                    for num in numbers {
                        if Self::is_valid_pos(puzzle, r, c, num) {
                            puzzle[r][c] = Some(num);

                            if Self::fill_grid(puzzle) {
                                return true;
                            }

                            puzzle[r][c] = None; // Backtrack
                        }
                    }
                    return false; // Vicolo cieco
                }
            }
        }
        true
    }

    pub fn generation(&self) -> SudokuGrid {
        let mut puzzle: SudokuGrid = [[None; 9]; 9];

        Self::fill_grid(&mut puzzle);
        puzzle
    }

    pub fn create_puzzle(mut puzzle: SudokuGrid, difficulty: Difficulty) -> SudokuGrid {
        let mut rng = rand::rng();
        let mut removed = 0;
        let target = difficulty.get_remove_count();

        while removed < target {
            let r = rng.random_range(0..9);
            let c = rng.random_range(0..9);

            if puzzle[r][c].is_some() {
                puzzle[r][c] = None;
                removed += 1;
            }
        }
        puzzle
    }

    pub fn set(&mut self, row: usize, col: usize, num: u8) -> bool {
        if self.fixed[row][col] {
            return false;
        }
        if self.is_valid(row, col, num) {
            self.grid[row][col] = Some(num);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self, row: usize, col: usize) {
        if !self.fixed[row][col] {
            self.grid[row][col] = None;
        }
    }

    pub fn is_fixed(&self, row: usize, col: usize) -> bool {
        self.fixed[row][col]
    }

    pub fn is_complete(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
    }

    pub fn generate_puzzle(difficulty: Option<Difficulty>) -> Self {
        let mut sudoku = Sudoku::new();

        sudoku.grid = sudoku.generation();

        let selected_diff = difficulty.unwrap_or(Difficulty::Easy);

        sudoku.grid = Sudoku::create_puzzle(sudoku.grid, selected_diff);

        for r in 0..9 {
            for c in 0..9 {
                if sudoku.grid[r][c].is_some() {
                    sudoku.fixed[r][c] = true;
                }
            }
        }

        sudoku
    }
}
