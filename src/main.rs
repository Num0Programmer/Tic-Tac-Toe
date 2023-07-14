use std::io::Write;

const FOR_BOARD: [u8; 9] = [0, 0, 1, 0, 1, 0, 1, 0, 0]; // for case '/'
const BACK_BOARD: [u8; 9] = [1, 0, 0, 0, 1, 0, 0, 0, 1]; // for case '\'
const EMPTY_SLOT: char = ' ';

fn main() -> ()
{
    let mut running = main_menu();

    while running
    {
        game();
        running = main_menu();
    }
    println!("Thanks for playing!");
}

// structs, enums, and functions dedicated to the main menu
fn main_menu() -> bool
{
    let mut selection = String::new();
    let keep_running: bool;

    print_menu();
    get_user_menu_input(&mut selection);
    keep_running = input_user_selection(&mut selection);

    return keep_running
}

fn get_user_menu_input(selection: &mut String) -> ()
{
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line( selection).unwrap();
}

fn print_menu() -> ()
{
    println!("");
    println!("Tic Tac Toe");
    println!("-----------");
    println!("1) Play");
    println!("2) Credits");
    println!("3) Quit\n");
    print!("> ");
}

fn input_user_selection(selection: &mut String) -> bool 
{
    match selection.trim()
    {
        "2" => {
            println!("Credit to Num0Programmer");
            return false
        },
        "3" => { return false },
        &_ => {}
    }
    println!(""); // create space

    true
}

// structs, enums, and functions dedicated to implementing the game
fn game() -> ()
{
    let mut turns: u8 = 0;
    let mut result = false;

    let mut slot: (usize, usize);
    let mut board = [EMPTY_SLOT; 9];

    while !result && turns < 9
    {
        turns += 1;
        display_board(board);

        slot = slot_selection(&mut board);
        add_to_board(&mut board, slot, turns);

        result = check_board(board, turns);
    }
    display_board(board);
    check_winner(turns, result);
}

fn check_winner(turns: u8, not_a_tie: bool) ->()
{
    let winner: u8;

    if !not_a_tie 
    {
        println!("Tie!\n")
    }

    else 
    {
        winner = turns % 2;
        if winner == 1
        {
            println!("Congratulations player 1, you win!\n");
        }
        else if winner == 0
        {
            println!("Congratulations player 2, you win!\n");
        }
    }
}

fn get_slot() -> (usize, usize)
{
    let mut valid_slot: bool = false;
    let mut row_is_num: bool;
    let mut col_is_num: bool;

    let mut row_idx = String::new();
    let mut col_idx = String::new();

    while !valid_slot
    {
        row_idx.clear();
        col_idx.clear();

        print!("Please enter a row index (0 - 2): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut row_idx).unwrap();
        
        print!("Please enter a column index (0 - 2): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut col_idx).unwrap();

        row_is_num = str_is_number(&row_idx);
        col_is_num = str_is_number(&col_idx);

        if col_is_num && row_is_num
        {
            valid_slot = is_valid_slot(&row_idx, &col_idx);
        }

        if !valid_slot
        {
            println!();
        }

    }

    let row: usize = row_idx.trim().parse::<usize>().unwrap();
    let col: usize = col_idx.trim().parse::<usize>().unwrap();

    (row, col)
}

fn is_valid_slot(row_idx: &String, col_idx: &String) -> bool
{

    let row_index: usize = row_idx.trim().parse::<usize>().unwrap();
    let col_index: usize = col_idx.trim().parse::<usize>().unwrap();

    let valid_col_idx: bool = is_valid_idx(row_index);
    let valid_row_idx: bool = is_valid_idx(col_index);

    return valid_col_idx && valid_row_idx;
}

fn is_valid_idx(index: usize) -> bool
{
    // check if index is 0,1 or 2
    for number in 0..3
    {
        if number == index
        {
            return true;
        }
    }
    return false
}

fn str_is_number(str: &String) -> bool
{
    let mut index: usize = 0;
    let mut is_num: bool;
    let mut str_chars = str.chars();


    while index < str.len() - 1
    {
        is_num = str_chars.next().unwrap().is_numeric();
        
        if !is_num 
        {
            return false;
        }

        index += 1;
    }
    return true;
}

// structs, enums, and functions dedicated to implementing the board
/// representation of a tic-tac-toe board
fn add_to_board(board: &mut [char; 9], slot: (usize, usize), turns: u8) -> ()
{
    let selected_slot: usize = slot.0 * 3 + slot.1;
    let player_1_turn: bool = turns % 2 == 1;

    if player_1_turn
    {
        board[selected_slot] = 'O';
    }
    else
    {
        board[selected_slot] = 'X';
    }
}

fn check_board(board: [char; 9], p_check: u8) -> bool
{
    let mut symbol = 'O';
    let mut slot;
    let mut p_sum: [u8; 6] = [0; 6];
    let mut for_diag_score: u8 = 0;
    let mut back_diag_score: u8 = 0;

    if p_check % 2 == 0
    {
        symbol = 'X';
    }

    for row in 0..3
    {
        for col in 0..3
        {
            slot = row * 3 + col;

            if board[slot] == symbol
            {
                p_sum[row] += 1;
                p_sum[col + 3] += 1;

                for_diag_score += FOR_BOARD[slot];
                back_diag_score += BACK_BOARD[slot];
            }
        }
    }

    for row in 0..3
    {
        if p_sum[row] == 3
        {
            return true; // player filled a row
        }

        for col in 0..3
        {
            if p_sum[col + 3] == 3
            {
                return true; // player filled a column
            }
        }
    }

    if for_diag_score == 3 || back_diag_score == 3
    {
        return true; // play filled diagonal
    }

    false // tie or board is not full
}

fn display_board(board: [char; 9]) -> ()
{
    println!("\nBoard");
    println!("-------------");
    for row in 0..3
    {
        print!("|");
        std::io::stdout().flush().unwrap();
        for col in 0..3
        {
            print!(" {} |", board[row * 3 + col]);
            std::io::stdout().flush().unwrap();
        }
        println!("\n-------------");
    }
    println!(""); // create space
}

fn is_slot_empty(board: &mut [char; 9], slot: (usize, usize)) -> bool
{
    let selected_slot: usize = slot.0 * 3 + slot.1;

    if board[selected_slot] == EMPTY_SLOT
    {
        return true;
    }
    return false;
}

fn slot_selection(board: &mut [char; 9]) -> (usize, usize)
{
    let mut slot: (usize, usize) = get_slot();
    let mut is_empty: bool = is_slot_empty(board, slot);

    while !is_empty
    {
        println!("");
        println!("This space has already been used!\n");
        slot = get_slot();
        is_empty = is_slot_empty(board, slot);
    }
    return slot;
}