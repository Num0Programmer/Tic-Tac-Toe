use std::io::Write;

const FOR_BOARD: [u8; 9] = [0, 0, 1, 0, 1, 0, 1, 0, 0]; // for case '/'
const BACK_BOARD: [u8; 9] = [1, 0, 0, 0, 1, 0, 0, 0, 1]; // for case '\'

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

    println!("");
    println!("Tic Tac Toe");
    println!("-----------");
    println!("1) Play");
    println!("2) Credits");
    println!("3) Quit\n");
    print!("> ");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut selection).unwrap();
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
    let winner: u8;
    let mut turns: u8 = 0;
    let mut result = false;

    let mut slot: (usize, usize);
    let mut board = [' '; 9];

    while !result && turns < 10
    {
        turns += 1;
        display_board(board);
        slot = get_slot();
        add_to_board(&mut board, slot, turns);
        result = check_board(board, turns);
    }
    display_board(board);

    winner = turns % 2;
    if winner == 1
    {
        println!("Congratulations player 1, you win!\n");
    }
    else if winner == 0
    {
        println!("Congratulations player 2, you win!\n");
    }
    else
    {
        println!("Tie!\n");
    }
}

fn get_slot() -> (usize, usize)
{
    let mut row_idx = String::new();
    let mut col_idx = String::new();

    print!("Please enter a row index (0 - 2): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut row_idx).unwrap();
    
    print!("Please enter a column index (0 - 2): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut col_idx).unwrap();

    let row: usize = row_idx.trim().parse::<usize>().unwrap();
    let col: usize = col_idx.trim().parse::<usize>().unwrap();

    (row, col)
}

// structs, enums, and functions dedicated to implementing the board
/// representation of a tic-tac-toe board
fn add_to_board(board: &mut [char; 9], slot: (usize, usize), turns: u8) -> ()
{
    if turns % 2 == 1
    {
        board[slot.0 * 3 + slot.1] = 'O';
    }
    else
    {
        board[slot.0 * 3 + slot.1] = 'X';
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

