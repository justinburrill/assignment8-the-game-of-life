// Justin Burrill
// apr 26 2023

mod utils;
use std::{cmp, io::Write};
use colored::*;
use read_input::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use crate::utils::find_number_of_digits;


struct Board
{
    board:Vec<Vec<bool>>,
    generation_count:u32
}

fn toggle_cell(mut board:Board, coordinates:[usize;2]) -> Board
{
    board.board[coordinates[0]][coordinates[1]] = !board.board[coordinates[0]][coordinates[1]];
    board
}

fn print_board(board:&Vec<Vec<bool>>, with_axis:bool, highlight_x:Option<i32>)
{

    let alive_char:char = 'X';
    let dead_char:char = 'X';
    let axis_colour = 150;

    let board_size = board[0].len();
    let num_of_digits = utils::find_number_of_digits(board_size);

    // this chunk makes the top numbers if drawing board with numbered axis
    if with_axis
    {        
        // print numbers on top axis
        for digits_place_index in (0..num_of_digits).rev()
        {
            
            let mut str:String = String::from("");
            for num in 0..board_size
            {
                if find_number_of_digits(num) > digits_place_index
                {
                    // convert number to string, then to list of chars, then reverse it, then convert to string
                    let num_str:String = num.to_string().chars().rev().collect();
                    // weird workaround because no string indexing??
                    let byte:u8 = num_str.as_bytes()[digits_place_index as usize];
                    str.push(byte as char);
                }
                else {
                    str.push(' ');
                }
            }
            let mut spaces = String::from(" ");
            for _spaces_count in 0..num_of_digits
            {
                spaces.push(' ');
            }
            println!("{}{}", spaces, str.truecolor(axis_colour, axis_colour, axis_colour));
        }
        println!();
    }
    

    for y_line_index in 0..board.len()
    {
        let mut spaces = String::from(" ");
        
        let y_index_num_of_digits = find_number_of_digits(y_line_index);
        for _spaces_count in 0..num_of_digits-y_index_num_of_digits
        {
            spaces.push(' ');
        }
        
        if with_axis {print!("{}{}", String::from(y_line_index.to_string()).truecolor(axis_colour, axis_colour, axis_colour), spaces);}

        for cell_x in 0..board[y_line_index].len()
        {
            let cell:bool = board[y_line_index][cell_x];
            let mut alive_print_colour:u8 = 255;
            let mut dead_print_colour:u8 = 170;
            
            // if there is an input to highlight a certain line of cells, set print colour on other lines to a darker colour
            if highlight_x != None && highlight_x.unwrap() >= 0 && cell_x != highlight_x.unwrap().try_into().unwrap()
            {
                alive_print_colour = 120;
                dead_print_colour = 20;
            }

            if cell
            {
                print!("{}", String::from(alive_char).truecolor(alive_print_colour-20, alive_print_colour, alive_print_colour-20).bold());
            }
            else
            {
                print!("{}", String::from(dead_char).truecolor(dead_print_colour+60, dead_print_colour, dead_print_colour));
            }

        }
        // flush stdout
        std::io::stdout().flush().unwrap();
        println!();
    }
}

fn input_coordinate_pair(board:&Board) -> [i32;2]
{
    let mut out:[i32;2] = [-1,-1];
    let letters:[char;2] = ['y', 'x'];
    let min: i32 = 0;
    let max: i32 = (board.board.len() as i32) - 1;

    for count in (0..=1).rev()
    {
        utils::clear_screen();
        print_board(&board.board, true ,Some(out[1]));
        println!("input {} coordinate of desired cell, or press enter to continue", String::from(letters[count]).bold());
        // let num:i32 = utils::input_in_range(0, board.len().try_into().unwrap());
        let num:i32;

        loop
        {
            let input:String = input().get();

            // end loop if blank input
            if input == ""
            {
                return [-1,-1];
            }

            let input_num:i32 = input.parse::<i32>().unwrap();

            if min <= input_num && input_num <= max
            {
                num = input_num;
                break;
            }

            println!("Invalid input");

        }

        out[count] = num;
    }    

    // return as [y, x]
    out
}

fn check_cell_life(board:&Board, x:usize, y:usize) -> bool
{
    let mut neighbour_count:u8 = 0;

    // iterate in a 3x3 cube around the given cell
    // max/min func used to avoid out-of-bounds index
    // ..= is inclusive range 
    if [x, y] == [2,2]
    {
        let _dsafdsa = 5;
    }

    for neighbour_y in cmp::max(y as isize -1, 0) as usize ..= cmp::min(y+1, board.board.len()-1)
    {
        for neighbour_x in cmp::max(x as isize -1,0) as usize..= cmp::min(x+1, board.board[neighbour_y].len()-1)
        {
            if board.board[neighbour_y][neighbour_x] && [neighbour_y, neighbour_x] != [y, x]
            {
                neighbour_count+=1;
            }
        }
        
    }

    if neighbour_count >= 2 && neighbour_count <= 3 && board.board[y][x] || neighbour_count == 3
    {
        return true;
    }
    else
    {
        return false;
    }
    
}

fn gen_next_board(old_board:&Board) -> Board
{
    let mut new_board:Board = Board {
        generation_count: old_board.generation_count,
        board: vec![vec![false; old_board.board[0].len()];old_board.board.len()],
    };

    for y in 0..old_board.board.len()
    {
        for x in 0..old_board.board[y].len()
        {
            if check_cell_life(old_board, x, y)
            {
                new_board.board[y][x] = true;
            }
        }
    }

    new_board.generation_count += 1;
    new_board
}

fn display_board_and_info(board:&Board, auto_on:bool)
{
    utils::clear_screen();

    print_board(&board.board, false, None);
    println!("generation number {}", String::from(board.generation_count.to_string()).bold());

    if auto_on
    {
        println!("stop auto generate - {}:\n", String::from("enter").bold());

    }
    else {
        println!("generate frame - {}, auto generate frames - {}, generate multiple frames - {}:\n", String::from("enter").bold(), String::from("a").bold(), String::from("input # of frames").bold());

    }
}

fn main() {
    utils::clear_screen();
    
    
    // user inputs size of board
    let mut input_length:usize = 0;
    while input_length == 0
    {
        println!("Input board size:\n");
        // loop{
        input_length = input().get();
        // println!("{}", find_number_of_digits(input_length));
        // }
    };


    // init "Board" to hold info about the cells
    let mut board = Board {
        generation_count: 0,
        board: vec![vec![false; input_length];input_length],
    };


    // user sets starting cells
    loop
    {
        utils::clear_screen();
        let pair:[i32;2] = input_coordinate_pair(&board);
        if pair[0] == -1 || pair[1] == -1
        {
            break;
        }
        else {
            board = toggle_cell(board, [pair[0].try_into().unwrap(), pair[1].try_into().unwrap()]);
        }
    }

    utils::clear_screen();

    let mut auto:bool = false;

    loop
    {

        // different prompts for if auto generate is on
        if auto
        {
            // create send/receive channel
            let (tx, rx) = mpsc::channel::<bool>();

            // create thread to watch for user input while the main program is running the game
            thread::spawn(move || {
                let input:String = input().get();
                if input.trim() == ""
                {
                    // send value down receiver
                    tx.send(false).ok();
                }
        
            });

            loop
            {
                // attempt to receive value
                let receive = rx.try_recv();
                // error if empty, if a value is sent, this bit runs
                if !receive.is_err()
                {
                    // end loop
                    auto = false;
                    break;
                }

                display_board_and_info(&board, true);
                board = gen_next_board(&board);
                thread::sleep(Duration::from_millis(500));
            }
            continue;
        }
        else
        {
            display_board_and_info(&board, false);
        }
        
        let input_str = input::<String>().get();

        if input_str.trim().to_uppercase() == "A"
        {
            auto = true;
        }
        else if input_str.trim().to_uppercase() == ""
        {
            board = gen_next_board(&board);
        }
        else if input_str.trim().parse::<u32>().is_ok()
        {
            for _count in 0..input_str.trim().parse::<u32>().unwrap()
            {
                board = gen_next_board(&board);
            }
        }

        utils::clear_screen();
    }


}
