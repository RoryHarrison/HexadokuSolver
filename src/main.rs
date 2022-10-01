use std::fs;

type Board = Vec<Vec<Option<u32>>>;

fn main() {
    let grid = parse_input("input.txt");
    let solution = solve(grid);
    print_grid(solution);
}


fn solve(grid: Board) -> Board {
    let mut result = grid.clone();
    let (y, x) = next_empty(&grid);

    if solved(&grid) {
        return result;
    }

    for guess in all_possible(&grid, y, x) {
        result[y][x] = Some(guess);
        result = solve(result);
        if solved(&result) {
            return result;
        }
    }
    result = grid;
    result
}

fn print_grid(grid: Board) {
    //Convert from Option<u32> -> char
    let mut board = Vec::new();
    for row in grid {
        let r = row.iter().map(|x| char::from_digit(x.unwrap_or(0), 16).unwrap()).collect::<Vec<char>>();
        board.push(r);
    }
    
    for y in 0..16 {
        let mut tmp = String::new();
        for x in 0..16 {
            if x%4 == 0 && x!= 0 {tmp.push('|')}
            tmp.push_str(board[y][x].to_string().as_str());
            tmp.push(' ');
        }
        println!("{tmp}");
        if (y+1)%4 == 0 && y != 0 && y != 15 {
            println!("--------+--------+--------+--------");
        }
    }
}

fn next_empty(grid: &Board) -> (usize, usize){
    for y in 0..16 {
        for x in 0..16 {
            if grid[y][x] == None {
                return (y, x);
            }
        }
    }
    (16,16)
}

fn solved(grid: &Board) -> bool {
    next_empty(grid) == (16, 16)
}

fn all_possible(grid: &Board, y: usize, x: usize) -> Vec<u32> {
    let mut result = Vec::new();
    for guess in 0..16 {
        if possible(y, x, grid, guess) {
            result.push(guess);
        }
    }
    result
}

fn possible(y: usize, x: usize, grid: &Board, n: u32) -> bool {
    //Check for value in rows/columns
    for i in  0..16 {
        if grid[y as usize][i] == Some(n) {
            return false
        }
        if grid[i][x as usize] == Some(n) {
            return false
        }
    }

    //Check for n in 'square'
    let x0 = (x/4)*4;
    let y0 = (y/4)*4;
    for i in 0..4 {
        for j in 0..4 {
            if grid[(y0+i) as usize][(x0+j) as usize] == Some(n) {
                return false
            }
        }
    }
    true
}

fn parse_input(file: &str) -> Board {
    let mut grid: Board = Vec::new();
    if let Ok(input) = fs::read_to_string(file) {
        let input = input.replace(' ', "");
        for line in input.lines() {
            let row = line.chars().map(|x| x.to_digit(16)).collect();
            grid.push(row);
       }
    }
    grid
}
