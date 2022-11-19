use std::io; 
use rand::Rng; 

fn visualize_board(board: [char; 9]) {
    // Visualizes the board. 
    println!(" -------------"); 
    for index in 1..10 {
        print!(" | {}", board[index-1]); 
        if index % 3 == 0{
            println!(" |"); 
            println!(" -------------"); 
        }
    }
}

fn board_is_full(board: [char; 9]) -> bool {
    // Checks if board is full or not. 
    for element in board {
        if element == ' ' {
            return false;  
        }
    }
    true   
}

fn check_winner(board: [char; 9]) -> bool {
    // Checks if someone won or not.
    for i in 0..3 {
        let h_i = i * 3; 
        if board[i] == ' ' || board[h_i] == ' '{
            continue; 
        } 
        if board[i] == board[i+3] && board[i+3] == board[i+6] || 
            board[h_i] == board[h_i+1] && board[h_i+1] == board[h_i+2] {
                return true; 
            }
    }

    if board[0] == ' ' || board[2] == ' ' {
        return false;   
    } else if board[0] == board[4] && board[4] == board[8] || 
        board[2] == board[4] && board[4] == board[6] {
            return true;  
    }
    false 
}

fn select_symbol() -> (char, char) {
    // Makes the player select a symbol 
    loop {
        println!("Wanna be X or O? Choose: "); 
        let mut choice = String::new(); 

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");  

        let choice = choice.trim().to_string(); 

        if choice == "X" {
            return ('X', 'O'); 
        } else if choice == "O"{
            return ('O', 'X'); 
        } else {
            println!("That's not a valid choice! Make sure to remove any whitespaces if you added one!"); 
            continue; 
        }
    }
}

fn select_index() -> usize {
    // Makes the player select an index. 
    loop {
        println!("Choose positions 1-9: "); 
        let mut index_input = String::new();

        io::stdin()
            .read_line(&mut index_input)
            .expect("Failed to read line."); 
        
        let index: usize = match index_input.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, 
        }; 

        if index >= 1 && index <= 9 {
            return index-1; 
        } else {
            println!("Must be from 1-9."); 
        }
    }
}

fn player_place_sym(board: &mut [char; 9], player_sym: char) {
    // The player place symbol function. 
    loop {
        let board_index = select_index();
        if board[board_index] == ' ' {
            board[board_index] = player_sym;
            break; 
        } else {
            let board_index = board_index + 1; 
            println!("Position {board_index} is not valid!"); 
        }
    }
}

fn computer_place_sym(board: &mut [char;9], computer_sym: char) {
    // The computer place symbol function 
    loop {
        let computer_index = rand::thread_rng().gen_range(0..=8); 
        if board[computer_index] == ' ' {
            board[computer_index] = computer_sym; 
            let computer_index = computer_index + 1; 
            println!("I pick position {computer_index}");
            break; 
        }
    }
}

fn main() {
    println!("Scripto's Tictactoe Game");
    println!("Board positions in case you don't know or your mind actually forgot it."); 
    println!("-------------"); 
    println!("| 1 | 2 | 3 |"); 
    println!("-------------"); 
    println!("| 4 | 5 | 6 |"); 
    println!("-------------");
    println!("| 7 | 8 | 9 |"); 
    println!("-------------"); 
    println!(""); 

    let mut player_turn =  rand::thread_rng().gen_range(0..=1) != 0; 
    let mut board: [char; 9] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']; 
    let (player_sym, computer_sym) = select_symbol(); 

    println!(""); 

    loop {
        if player_turn {
            player_place_sym(&mut board, player_sym); 
            player_turn = false;
            
            if check_winner(board) {
                println!("Player wins!");
                visualize_board(board); 
                break; 
            }

        } else {
            computer_place_sym(&mut board, computer_sym); 
            player_turn = true;  

            if check_winner(board) {
                println!("Computer wins!"); 
                visualize_board(board);
                break; 
            }
        }

        visualize_board(board); 

        println!(""); 

        if board_is_full(board) {
            println!("The board is full. The game ends with no one winning or losing :((((("); 
            break; 
        } 
    }
}