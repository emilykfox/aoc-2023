#[derive(Default, Clone, Copy, Debug)]
struct Cell {
    number: u8,
    marked: bool,
}

#[derive(Clone, Copy, Debug)]
struct Board {
    cells: [[Cell; 5]; 5],
    won: bool,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/2021-day04.txt").unwrap();

    let draws = lines[0].split(',').map(|draw| draw.parse::<u8>().unwrap());
    let num_boards = lines.len() / 6;
    let mut boards: Vec<Board> = Vec::new();
    for board_index in 0..num_boards {
        let mut board = Board {
            cells: [[Cell::default(); 5]; 5],
            won: false,
        };
        for row_index in 0..5 {
            for (column_index, number) in lines[2 + 6 * board_index + row_index]
                .split(' ')
                .filter(|number| !number.is_empty())
                .enumerate()
            {
                board.cells[row_index][column_index].number = number.parse().unwrap();
            }
        }
        boards.push(board);
    }

    let mut first_score = 0;
    let mut last_score = 0;
    for draw in draws {
        for board in boards.iter_mut().filter(|board| !board.won) {
            // mark cell
            for row in board.cells.iter_mut() {
                for cell in row.iter_mut() {
                    if cell.number == draw {
                        cell.marked = true;
                    }
                }
            }

            // check for win
            let mut win = false;
            for row in board.cells.iter() {
                if row.iter().filter(|cell| cell.marked).count() == 5 {
                    win = true;
                }
            }
            for column_index in 0..5 {
                if board
                    .cells
                    .iter()
                    .filter(|row| row[column_index].marked)
                    .count()
                    == 5
                {
                    win = true;
                }
            }
            if win {
                board.won = true;
                let mut unmarked_total: u32 = 0;
                for row in board.cells.iter() {
                    for cell in row.iter() {
                        if !cell.marked {
                            unmarked_total += cell.number as u32;
                        }
                    }
                }
                let score = unmarked_total * (draw as u32);
                if first_score == 0 {
                    first_score = score;
                }
                last_score = score;
            }
        }
    }

    println!("Part A: {}", first_score);
    println!("Part B: {}", last_score);
}
