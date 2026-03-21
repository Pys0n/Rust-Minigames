struct TicTacToe {
    board: [[char; 3]; 3],
    turn: char,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            board: [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']],
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

    fn has_winner(&self) -> (bool, char) {
        // check horizontally
        if self.board[0][0] == self.board[0][1] && 
           self.board[0][1] == self.board[0][2] && 
           self.board[0][0] != ' ' {
            return (true, self.board[0][0]);
        } else if self.board[1][0] == self.board[1][1] && 
                  self.board[1][1] == self.board[1][2] && 
                  self.board[1][0] != ' ' {
            return (true, self.board[1][0]);
        } else if self.board[2][0] == self.board[2][1] && 
                  self.board[2][1] == self.board[2][2] && 
                  self.board[2][0] != ' ' {
            return (true, self.board[2][0]);
        }

        // check vertically
        if self.board[0][0] == self.board[1][0] && 
           self.board[1][0] == self.board[2][0] && 
           self.board[0][0] != ' ' {
            return (true, self.board[0][0]);
        } else if self.board[0][1] == self.board[1][1] && 
                  self.board[1][1] == self.board[2][1] && 
                  self.board[0][1] != ' ' {
            return (true, self.board[0][1]);
        } else if self.board[0][2] == self.board[1][2] && 
                  self.board[1][2] == self.board[2][2] && 
                  self.board[0][2] != ' ' {
            return (true, self.board[0][2]);
        }

        // check diagonally
        if self.board[0][0] == self.board[1][1] &&
           self.board[1][1] == self.board[2][2] &&
           self.board[0][0] != ' ' {
            return (true, self.board[1][1]);
        } else if self.board[0][2] == self.board[1][1] &&
                  self.board[1][1] == self.board[2][0] &&
                  self.board[0][2] != ' ' {
            return (true, self.board[1][1]);
        }

        // check remis
        if self.board[0][0] != ' ' &&
           self.board[0][1] != ' ' &&
           self.board[0][2] != ' ' &&
           self.board[1][0] != ' ' &&
           self.board[1][1] != ' ' &&
           self.board[1][2] != ' ' &&
           self.board[2][0] != ' ' &&
           self.board[2][1] != ' ' &&
           self.board[2][2] != ' ' {
            return (true, ' ');
        }

        return (false, ' ');
    }

    fn print_board(&self) {
        println!("{}|{}|{}", self.board[0][0], self.board[0][1], self.board[0][2]);
        println!("—|—|—");
        println!("{}|{}|{}", self.board[1][0], self.board[1][1], self.board[1][2]);
        println!("—|—|—");
        println!("{}|{}|{}", self.board[2][0], self.board[2][1], self.board[2][2]);
    }
}

fn main() {
    let mut ttt = TicTacToe::new();

    while !ttt.has_winner().0 {
        ttt.print_board();

        println!("Please enter a number between 0 and 8: ");

        let mut field = String::new();

        std::io::stdin()
            .read_line(&mut field)
            .expect("Invalid Input!");

        let field_id: usize = field.trim().parse().expect("Could not convert input into usize!");

        if ttt.board[field_id/3][field_id%3] == ' ' {
            ttt.board[field_id/3][field_id%3] = ttt.turn;
            ttt.switch_turn();
        } else {
            println!("Invalid field!");
        }
    }

    ttt.print_board();

    let winner = ttt.has_winner().1;

    if winner == ' ' {
        println!("Nobody won!");
    } else {
        println!("{winner} won!");
    }

}