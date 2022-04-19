use rand::{thread_rng, Rng};
use std::io;
use clearscreen::clear;

fn main() {
    let mut board = [['*'; 11]; 11];
    //randomly create destination
    let (x, y) = (thread_rng().gen_range(1..10), thread_rng().gen_range(0..10));
    //initial position of player
    board[0][0] = 'X';
    board[x as usize][y as usize] = 'Y';
    let mut input = String::new();
    let (mut x1, mut y1) = (0 as i32, 0 as i32);
    println!("****************************************");
    println!("You are 'X' and you have to reach 'Y'");
    println!("Use W A S D to move your 'X'");
    println!("Press 'X' to quit the game");
    println!("Press any key to start the game .... ");
    println!("****************************************");
    io::stdin().read_line(&mut &mut input).unwrap();
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    clear().unwrap();
    loop {
        if x1 < 0 || x1 > 10 || y1 < 0 || y1 > 10 {
            println!("Out of bounds. You lose :/");
            break;
        }
        if x1 == x && y1 == y {
            println!("You win :D");
            break;
        }
        board[x1 as usize][y1 as usize] = 'X';
        print(&board);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "w" => {
                board[x1 as usize][y1 as usize] = '*';
                x1 = x1- 1;
            }
            "a" => {
                board[x1 as usize][y1 as usize] = '*';
                y1 = y1 - 1;
            }
            "s" => {
                board[x1 as usize][y1 as usize] = '*';
                x1 = x1 +1;
            }
            "d" => {
                board[x1 as usize][y1 as usize] = '*';
                y1 = y1 + 1;
            }
            "x" => {
                println!("Game Exit. BYE");
                break;
            }
            _ => {
                continue;
            }
        }
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        clear().unwrap();
    }
    println!("press any key to continue");
    io::stdin().read_line(&mut input).unwrap();
}

fn print(board: &[[char; 11]; 11]) {
    for i in board {
        for j in i {
            print!("{j} ");
        }
        println!();
    }
}
