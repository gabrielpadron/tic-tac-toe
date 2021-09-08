#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
type Board = Vec<Vec<char>>;

fn main() {
    println!("Bem-vindo ao jogo da velha");
    game();
}

fn print_board(board: Board) {
    println!("");
    println!("{} | {} | {}", board[0][0], board[0][1], board[0][2]);
    println!("--+---+--");
    println!("{} | {} | {}", board[1][0], board[1][1], board[1][2]);
    println!("--+---+--");
    println!("{} | {} | {}", board[2][0], board[2][1], board[2][2]);
    println!("");
}

fn game() {
    let mut board: Board = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let turn = "X";
    let mut count: i32 = 0;
    let finished = false;

    while !finished {
        print_board(board.clone());
        println!("Insira um inteiro entre 1 e 9:");

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let choice = line.trim().parse::<usize>().unwrap();
        let (row, col) = match choice {
            1 => (0, 0),
            2 => (0, 1),
            3 => (0, 2),
            4 => (1, 0),
            5 => (1, 1),
            6 => (1, 2),
            7 => (2, 0),
            8 => (2, 1),
            _ => (2, 2),
        };

        if board[row][col] != 'X' && board[row][col] != 'O' {
            board[row][col] = 'X';
            count += 1;
        } else {
            println!("Este campo já está ocupado, \nSelecione outro campo:");
            continue;
        }
    }
}
