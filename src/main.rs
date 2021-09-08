#![allow(dead_code)]
#![allow(unused_variables)]

extern crate text_io;
use text_io::{read, try_read};

type Board = Vec<char>;

fn main() {
    println!("Bem-vindo ao jogo da velha");
    game();

    println!("Você deseja jogar novamente? (s/n)");
    let restart: char = read!();
    if restart == 's' || restart == 'S' {
        game();
    }
}

fn print_board(board: &mut Board) {
    println!("");
    println!("{} | {} | {}", board[0], board[1], board[2]);
    println!("--+---+--");
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("--+---+--");
    println!("{} | {} | {}", board[6], board[7], board[8]);
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

    println!("Insira o nome do primeiro jogador: ");
    let player1: String = read!();

    println!("Insira o nome do segundo jogador: ");
    let player2: String = read!();
    let mut player_turn = &player1;

    while !finished {
        print_board(&mut board);

        println!("{} ({}), insira um inteiro entre 1 e 9:", player_turn, turn);
        let mut choice: usize = try_read!().unwrap();
        choice -= 1;

        if is_empty_cell(&board, choice) {
            board[choice] = turn;
            count += 1;
        } else {
            println!("\nEste campo já está ocupado, selecione outro campo:");
            continue;
        }

        if count >= 5 {
            // horizontal
            if board[6] == board[7] && board[7] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            } else if board[3] == board[4] && board[4] == board[5] && !is_empty_cell(&board, 5) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            } else if board[0] == board[1] && board[1] == board[2] && !is_empty_cell(&board, 2) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;

                // vertical
            } else if board[0] == board[3] && board[3] == board[6] && !is_empty_cell(&board, 6) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            } else if board[1] == board[4] && board[4] == board[7] && !is_empty_cell(&board, 7) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            } else if board[2] == board[5] && board[5] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;

                // diagonal
            } else if board[0] == board[4] && board[4] == board[8] && !is_empty_cell(&board, 8) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            } else if board[2] == board[4] && board[4] == board[6] && !is_empty_cell(&board, 6) {
                print_board(&mut board);
                println!(" ###### Fim de jogo, {} ganhou! ######", player_turn);
                finished = true;
            }
        }

        if count == 9 {
            println!(" ###### Fim de jogo, foi um empate! ######");
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
}
