use std::io::{ self, Write };
use std::process::exit;


enum GameResult {
    CrossWin, 
    CircleWin,
    Draw,
}

#[derive(PartialEq, Eq)]
enum CellType {
    Blank,
    Cross,
    Circle,
}

enum Command {
    Place { number: u8 },
    Quit,
    Help,
}

struct Cell {
    cell_type: CellType,
}

impl Cell {
    pub fn change_type(&mut self, new_type: CellType) {
        self.cell_type = new_type;
    }

    pub fn symbol(&self) -> char {
        match self.cell_type {
            CellType::Blank => ' ',
            CellType::Cross => 'X', 
            CellType::Circle => 'O',
        }
    }
}


fn print_help() {
    println!("Cell's numbers are:\n123\n456\n789");
    println!("Type number from 1 to 9 to place or \"quit\" to exit game");
}


fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render_cells(cells: &Vec<Cell>) {
    for i in 0..9 {
        print!("{}", cells[i].symbol());
        if i == 2 || i == 5 || i == 8 {
            println!();
            println!("―――――");
        } else {
            print!("|");
        }
    }
}

fn get_command() -> Result<Command, io::Error> {
    print!(">>> ");
    io::stdout().flush()?;

    let mut command = String::new();

    io::stdin().read_line(&mut command)?;

    command = command.trim().to_lowercase();

    match command.as_str() {
        "quit" | "exit" => return Ok(Command::Quit),
        "help" => return Ok(Command::Help),
        _ => {}
    }

    let number: u8 = match command.parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e))
        }
    };

    Ok(Command::Place { number })
}


fn main() {
    println!("Tic Tac Toe game in rust");
    print_help();
    println!();

    let mut cells: Vec<Cell> = Vec::new();
    let mut game_finished = false;
    let mut need_help = true;
    let mut current_player_is_cross = true;
    let mut winner = GameResult::Draw;
    let mut moves = 0;

    for _i in 0..9 {
        cells.push(Cell { cell_type: CellType::Blank });
    }

    while !game_finished {
        clear_screen();
        render_cells(&cells);

        if need_help {
            print_help();
            need_help = false;
        }

        let command = get_command();

        let command = match command {
            Ok(cmd) => cmd,
            Err(_) => continue
        };

        match command {
            Command::Quit => {
                clear_screen();
                exit(0)
            },
            Command::Help => need_help = true,
            Command::Place { number } => {
                let new_cell_type = if current_player_is_cross {
                    CellType::Cross
                } else {
                    CellType::Circle
                };
                if (number < 1) || (number > 9) {
                    continue
                }
                let ind = usize::from(number) - 1;
                if cells[ind].cell_type != CellType::Blank {
                    continue
                }
                cells[ind].change_type(new_cell_type);

            }
        }
        moves += 1;
        
        // ugly hardcode
        let indexes_to_check = [(1, 2, 3), (4, 5, 6), (7, 8, 9),
            (1, 4, 7), (2, 5, 8), (3, 6, 9),
            (1, 5, 9), (3, 5, 7)];

        for (i1, i2, i3) in indexes_to_check {
            let type1 = &cells[i1-1].cell_type;
            let type2 = &cells[i2-1].cell_type;
            let type3 = &cells[i3-1].cell_type;
            if type1 == type2 && type2 == type3 && type1 != &CellType::Blank {
                winner = if current_player_is_cross {
                    GameResult::CrossWin 
                } else {
                    GameResult::CircleWin
                };
                game_finished = true;
                break
            }
        }
        if !game_finished && moves == 9 {
            winner = GameResult::Draw;
            game_finished = true;
        }

        current_player_is_cross = !current_player_is_cross;
    }

    clear_screen();
    println!("Tic Tac Toe game in rust\n");
    render_cells(&cells);

    match winner {
        GameResult::Draw => println!("Draw"),
        GameResult::CrossWin => println!("Cross player win!"),
        GameResult::CircleWin => println!("Circle player win!"),
    }
}
