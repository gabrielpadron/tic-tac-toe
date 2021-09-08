extern crate colored;
extern crate text_io;
use colored::*;
use text_io::read;

type Board = Vec<char>;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\n{}", " ##### Bem-vindo ao jogo da velha #####".green());
    game();
}

fn print_board(board: &mut Board) {
    println!("");
    println!("\t{} | {} | {}", board[0], board[1], board[2]);
    println!("\t--+---+--");
    println!("\t{} | {} | {}", board[3], board[4], board[5]);
    println!("\t--+---+--");
    println!("\t{} | {} | {}", board[6], board[7], board[8]);
    println!("");
}

fn is_empty_cell(board: &Board, index: usize) -> bool {
    board[index] != 'X' && board[index] != 'O'
}

fn game() {
    let mut board: Board = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut turn: char = 'X';
    let mut count: i32 = 0;
    let mut finished: bool = false;

    println!("{}", "Insira o nome do primeiro jogador: ".yellow());
    let player1: String = read!();

    println!("{}", "Insira o nome do segundo jogador: ".yellow());
    let player2: String = read!();
    let mut player_turn = &player1;

    while !finished {
        print_board(&mut board);

        println!(
            "{} ({}){}",
            player_turn.green(),
            turn,
            ", insira um inteiro entre 1 e 9:".green()
        );
        let mut choice: usize = read!();
        choice -= 1;

        if is_empty_cell(&board, choice) {
            board[choice] = turn;
            count += 1;
        } else {
            println!(
                "\n{}",
                "Este campo já está ocupado, selecione outro campo:".red()
            );
            continue;
        }

        if count >= 5 {
            // horizontal
            if board[6] == board[7] && board[7] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            } else if board[3] == board[4] && board[4] == board[5] && !is_empty_cell(&board, 5) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            } else if board[0] == board[1] && board[1] == board[2] && !is_empty_cell(&board, 2) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;

                // vertical
            } else if board[0] == board[3] && board[3] == board[6] && !is_empty_cell(&board, 6) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            } else if board[1] == board[4] && board[4] == board[7] && !is_empty_cell(&board, 7) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            } else if board[2] == board[5] && board[5] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;

                // diagonal
            } else if board[0] == board[4] && board[4] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            } else if board[2] == board[4] && board[4] == board[6] && !is_empty_cell(&board, 6) {
                print_board(&mut board);
                println!(
                    "{} {} {}",
                    " ###### Fim de jogo,".yellow(),
                    player_turn.yellow(),
                    "ganhou! ######".yellow()
                );
                finished = true;
            }
        }

        if count == 9 {
            println!("{}", " ###### Fim de jogo, foi um empate! #####".blue(),);
            finished = true;
        }

        if turn == 'X' {
            turn = 'O';
            player_turn = &player2;
        } else {
            turn = 'X';
            player_turn = &player1;
        }
    }

    println!("Você deseja jogar novamente? (s/n)");
    let restart: char = read!();
    if restart == 's' || restart == 'S' {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        game();
    }
}
