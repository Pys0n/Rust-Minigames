struct FourInARow {
    board: [[char; 7]; 6],
    turn: char,
}

impl FourInARow {
    fn new() -> Self {
        Self {
            board: [
                [' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' '], 
                [' ', ' ', ' ', ' ', ' ', ' ', ' '], 
                [' ', ' ', ' ', ' ', ' ', ' ', ' '], 
                [' ', ' ', ' ', ' ', ' ', ' ', ' '], 
                [' ', ' ', ' ', ' ', ' ', ' ', ' '], 
            ],
            turn: 'X',
        }
    }
    
    fn switch_turn(&mut self) {
        match self.turn {
            'X' => self.turn = 'O',
            'O' => self.turn = 'X',
            _ => (),
        }
    }

    fn choose_column(&mut self, column: usize) {
        if self.board[0][column] == ' ' {
            for i in (0..6).rev() {
                if self.board[i][column] == ' ' {
                    self.board[i][column] = self.turn;
                    self.switch_turn();
                    return;
                }
            }
        }
    }
    
    fn has_winner(&self) -> (bool, char) {
        // vertically
        for col in 0..7 {
            for row in 0..3 {
                if self.board[row][col] == self.board[row+1][col] &&
                   self.board[row+1][col] == self.board[row+2][col] &&
                   self.board[row+2][col] == self.board[row+3][col] &&
                   self.board[row][col] != ' ' {
                    return (true, self.board[row][col]);
                   }
            }
        }

        // horizontally
        for col in 0..4 {
            for row in 0..6 {
                if self.board[row][col] == self.board[row][col+1] &&
                   self.board[row][col+1] == self.board[row][col+2] &&
                   self.board[row][col+2] == self.board[row][col+3] &&
                   self.board[row][col] != ' ' {
                    return (true, self.board[row][col]);
                   }
            }
        }

        // diagonally (top-left to bottom-right)
        for col in 0..4 {
            for row in 0..3 {
                if self.board[row][col] == self.board[row+1][col+1] &&
                   self.board[row+1][col+1] == self.board[row+2][col+2] &&
                   self.board[row+2][col+2] == self.board[row+3][col+3] &&
                   self.board[row][col] != ' ' {
                    return (true, self.board[row][col]);
                   }
            }
        }

        // diagonally (top-right to bottom-left)
        for col in 3..7 {
            for row in 0..3 {
                if self.board[row][col] == self.board[row+1][col-1] &&
                   self.board[row+1][col-1] == self.board[row+2][col-2] &&
                   self.board[row+2][col-2] == self.board[row+3][col-3] &&
                   self.board[row][col] != ' ' {
                    return (true, self.board[row][col]);
                   }
            }
        }

        return (false, ' ');
    }
    
    fn print_board(&self) {
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[0][0], self.board[0][1], self.board[0][2], self.board[0][3], self.board[0][4], self.board[0][5], self.board[0][6]);
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[1][0], self.board[1][1], self.board[1][2], self.board[1][3], self.board[1][4], self.board[1][5], self.board[1][6]);
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[2][0], self.board[2][1], self.board[2][2], self.board[2][3], self.board[2][4], self.board[2][5], self.board[2][6]);
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[3][0], self.board[3][1], self.board[3][2], self.board[3][3], self.board[3][4], self.board[3][5], self.board[3][6]);
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[4][0], self.board[4][1], self.board[4][2], self.board[4][3], self.board[4][4], self.board[4][5], self.board[4][6]);
        println!("{}|{}|{}|{}|{}|{}|{}", self.board[5][0], self.board[5][1], self.board[5][2], self.board[5][3], self.board[5][4], self.board[5][5], self.board[5][6]);
        println!("0|1|2|3|4|5|6")
    }
}

fn main() {
    let mut fiar = FourInARow::new();

    while !fiar.has_winner().0 {
        fiar.print_board();

        let mut column_str = String::new();

        println!("\nPlayer {}: Enter a number between 0 and 6: ", fiar.turn);

        std::io::stdin()
            .read_line(&mut column_str)
            .expect("Invalid input!");

        let column: usize = column_str.trim().parse().expect("Could not convert input into usize!");

        fiar.choose_column(column);
    }

    println!("");

    fiar.print_board();

    let winner = fiar.has_winner().1;
    if winner == ' ' {
        println!("\nDraw! Nobody won!");
    } else {
        println!("\n{} won!", winner);
    }

}