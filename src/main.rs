#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::io;

fn main() {
    println!("Bem-vindo ao jogo da velha");
    game();
}

fn print_board(board: &mut HashMap<i32, &str>) {
    println!("");
    println!("{} | {} | {}", board[&1], board[&2], board[&3]);
    println!("--+---+--");
    println!("{} | {} | {}", board[&4], board[&5], board[&6]);
    println!("--+---+--");
    println!("{} | {} | {}", board[&7], board[&8], board[&9]);
    println!("");
}

fn is_empty() -> bool {
    true
}

fn game() {
    let mut board = HashMap::new();
    board.insert(1, "1");
    board.insert(2, "2");
    board.insert(3, "3");
    board.insert(4, "4");
    board.insert(5, "5");
    board.insert(6, "6");
    board.insert(7, "7");
    board.insert(8, "8");
    board.insert(9, "9");

    let turn = 'X';
    let mut _count: i32;
    let finished = false;

    while !finished {
        print_board(&mut board);
        println!("Insira um inteiro entre 1 e 9:");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read stdin");
        let choice = line.trim().parse::<i32>().expect("Failed to parse input");

        if board[&choice].is_empty() {

        }
    }

}
