use rand::seq::SliceRandom;

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
    pub hint_filled: [[bool; 9]; 9],
}

type SudokuGrid = [[Option<u8>; 9]; 9];

impl Sudoku {
    fn get_candidates(&self, row: usize, col: usize) -> Vec<u8> {
        let mut candidates: Vec<u8> = (1..10).collect();

        for c in 0..9 {
            if let Some(n) = self.grid[row][c] {
                candidates.retain(|&x| x != n);
            }
        }
        for r in 0..9 {
            if let Some(n) = self.grid[r][col] {
                candidates.retain(|&x| x != n);
            }
        }
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for r in start_row..start_row + 3 {
            for c in start_col..start_col + 3 {
                if let Some(n) = self.grid[r][c] {
                    candidates.retain(|&x| x != n);
                }
            }
        }
        candidates
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c].is_none() {
                    return Some((r, c));
                }
            }
        }
        None
    }

    fn solve_with_logic(&mut self) -> bool {
        let mut changed = true;
        let mut attempts = 0;
        let max_attempts = 100;

        while changed && attempts < max_attempts {
            changed = false;
            attempts += 1;

            for r in 0..9 {
                for c in 0..9 {
                    if self.grid[r][c].is_none() {
                        let candidates = self.get_candidates(r, c);
                        if candidates.len() == 1 {
                            self.grid[r][c] = Some(candidates[0]);
                            changed = true;
                        }
                    }
                }
            }

            for region in 0..9 {
                for num in 1..10 {
                    let mut count = 0;
                    let mut pos = (0, 0);

                    for i in 0..9 {
                        let (r, c) = match region < 3 {
                            true => (region, i),
                            false if region < 6 => (i, region - 3),
                            false => (i / 3 + (region - 6) * 3, i % 3 + (region - 6) * 3),
                        };
                        if self.grid[r][c].is_none() && self.get_candidates(r, c).contains(&num) {
                            count += 1;
                            pos = (r, c);
                        }
                    }

                    if count == 1 {
                        self.grid[pos.0][pos.1] = Some(num);
                        changed = true;
                    }
                }
            }
        }

        self.find_empty().is_none()
    }

    fn solve_with_backtracking(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty() {
            let candidates = self.get_candidates(row, col);
            for num in candidates {
                self.grid[row][col] = Some(num);
                if self.solve_with_backtracking() {
                    return true;
                }
                self.grid[row][col] = None;
            }
            return false;
        }
        true
    }

    fn count_solutions(&mut self, limit: usize, depth: usize) -> usize {
        if depth > 200 {
            return limit;
        }
        if let Some((row, col)) = self.find_empty() {
            let candidates = self.get_candidates(row, col);
            let mut count = 0;
            for num in candidates {
                self.grid[row][col] = Some(num);
                count += self.count_solutions(limit, depth + 1);
                if count >= limit {
                    self.grid[row][col] = None;
                    return count;
                }
                self.grid[row][col] = None;
            }
            return count;
        }
        1
    }

    pub fn is_solvable(&self) -> bool {
        let mut test_grid = self.clone();
        if test_grid.solve_with_logic() {
            return true;
        }
        test_grid.grid = self.grid;
        test_grid.solve_with_backtracking()
    }

    fn has_unique_solution(&self) -> bool {
        let mut test_sudoku = Sudoku::new();
        test_sudoku.grid = self.grid;
        test_sudoku.solve_with_logic();
        test_sudoku.grid = self.grid;
        test_sudoku.count_solutions(2, 0) == 1
    }

    pub fn new() -> Self {
        Sudoku {
            grid: [[None; 9]; 9],
            fixed: [[false; 9]; 9],
            hint_filled: [[false; 9]; 9],
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

    pub fn generation() -> SudokuGrid {
        let mut puzzle: SudokuGrid = [[None; 9]; 9];

        Self::fill_grid(&mut puzzle);
        puzzle
    }

    pub fn create_puzzle(mut puzzle: SudokuGrid, difficulty: Difficulty) -> SudokuGrid {
        let mut rng = rand::rng();
        let target = difficulty.get_remove_count();

        let mut cells: Vec<(usize, usize)> =
            (0..9).flat_map(|r| (0..9).map(move |c| (r, c))).collect();
        cells.shuffle(&mut rng);

        let mut removed = 0;
        let mut checked = 0;
        let max_check = 200;

        for (r, c) in cells {
            if removed >= target || checked >= max_check {
                break;
            }

            if puzzle[r][c].is_some() {
                checked += 1;
                let backup = puzzle[r][c];
                puzzle[r][c] = None;

                let mut test_sudoku = Sudoku::new();
                test_sudoku.grid = puzzle;

                if test_sudoku.has_unique_solution() {
                    removed += 1;
                } else {
                    puzzle[r][c] = backup;
                }
            }
        }

        puzzle
    }

    pub fn set(&mut self, row: usize, col: usize, num: u8) -> bool {
        if self.fixed[row][col] || self.hint_filled[row][col] {
            return false;
        }
        if self.is_valid(row, col, num) {
            self.grid[row][col] = Some(num);
            true
        } else {
            false
        }
    }

    pub fn set_hint(&mut self, row: usize, col: usize, num: u8) -> bool {
        if self.fixed[row][col] {
            return false;
        }
        if self.is_valid(row, col, num) {
            self.grid[row][col] = Some(num);
            self.hint_filled[row][col] = true;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self, row: usize, col: usize) {
        if !self.fixed[row][col] && !self.hint_filled[row][col] {
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

    pub fn get_hint(&self, row: usize, col: usize) -> Option<u8> {
        if self.grid[row][col].is_some() {
            return None;
        }
        let candidates = self.get_candidates(row, col);
        if candidates.len() == 1 {
            Some(candidates[0])
        } else {
            None
        }
    }

    pub fn generate_puzzle(difficulty: Option<Difficulty>) -> Self {
        let selected_diff = difficulty.unwrap_or(Difficulty::Easy);
        let mut sudoku;
        let mut attempts = 0;
        let max_attempts = 100;

        loop {
            sudoku = Sudoku::new();
            sudoku.grid = Self::generation();
            sudoku.grid = Self::create_puzzle(sudoku.grid, selected_diff);

            if sudoku.is_solvable() {
                break;
            }

            attempts += 1;
            if attempts >= max_attempts {
                sudoku.grid = Self::generation();
                sudoku.grid = Self::create_puzzle(sudoku.grid, Difficulty::Easy);
                break;
            }
        }

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
