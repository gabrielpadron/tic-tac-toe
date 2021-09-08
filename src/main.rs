#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
type Board = Vec<char>;

fn main() {
    println!("Bem-vindo ao jogo da velha");
    game();
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

fn valid_move(board: &Board, index: usize) -> bool {
    board[index] != 'X' && board[index] != 'O'
}

fn game() {
    let mut board: Board = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut turn: char = 'X';
    let mut count: i32 = 0;
    let mut finished: bool = false;

    while !finished {
        print_board(&mut board);
        println!("{}, insira um inteiro entre 1 e 9:", turn);

        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Escolha inválida");
        let mut choice: usize = line.trim().parse::<usize>().unwrap();
        choice -= 1;

        if valid_move(&board, choice) {
            board[choice] = turn;
            count += 1;
        } else {
            println!("\nEste campo já está ocupado, selecione outro campo:");
            continue;
        }

        if count >= 5 {
            // horizontal
            if board[6] == board[7] && board[7] == board[8] && !valid_move(&board, 8) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            } else if board[3] == board[4] && board[4] == board[5] && !valid_move(&board, 5) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            } else if board[0] == board[1] && board[1] == board[2] && !valid_move(&board, 2) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;

            // vertical
            } else if board[0] == board[3] && board[3] == board[6] && !valid_move(&board, 6) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            } else if board[1] == board[4] && board[4] == board[7] && !valid_move(&board, 7) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            } else if board[2] == board[5] && board[5] == board[8] && !valid_move(&board, 8) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;

            // diagonal
            } else if board[0] == board[4] && board[4] == board[8] && !valid_move(&board, 8) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            } else if board[2] == board[4] && board[4] == board[6] && !valid_move(&board, 6) {
                print_board(&mut board);
                println!("Fim de jogo");
                println!(" ### {} ganhou ###", turn);
                finished = true;
            }
        }

        if count == 9 {
            println!("Fim de jogo\nEmpate!");
        }

        if turn == 'X' {
            turn = 'O';
        } else {
            turn = 'X';
        }
    }
}
