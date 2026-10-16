use crate::y2015::io::get_contents;

fn parse_grid(s: &str, day_two: bool) -> [[bool; 100]; 100] {
    let mut grid = [[false; 100]; 100];

    for (row, line) in s.lines().enumerate() {
        for (column, light) in line.chars().enumerate() {
            if light == '#' {
                grid[row][column] = true;
            }
        }
    }

    if day_two {
        grid[0][0] = true;
        grid[0][99] = true;
        grid[99][0] = true;
        grid[99][99] = true;
    }

    grid
}

fn update_grid(grid: &[[bool; 100]; 100], day_two: bool) -> [[bool; 100]; 100] {
    let mut new_grid = *grid;
    let corners = [(0, 0), (0, 99), (99, 0), (99, 99)];

    for row in 0..100 {
        for column in 0..100 {
            if day_two && corners.contains(&(row, column)) {
                continue;
            }

            let mut lit_lights = 0;

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let r = row + dr;
                    let c = column + dc;

                    if (0..100).contains(&r)
                        && (0..100).contains(&c)
                        && grid[r as usize][c as usize]
                    {
                        lit_lights += 1;
                    }
                }
            }

            let row_index = row as usize;
            let column_index = column as usize;

            match grid[row_index][column_index] {
                true => {
                    if lit_lights == 2 || lit_lights == 3 {
                        continue;
                    }
                    new_grid[row_index][column_index] = false;
                }
                false => {
                    if lit_lights == 3 {
                        new_grid[row_index][column_index] = true;
                    }
                }
            }
        }
    }

    new_grid
}

fn count_lights(grid: &[[bool; 100]; 100]) -> usize {
    grid.iter().flatten().filter(|&&cell| cell).count()
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d18.txt");
    let mut grid = parse_grid(&contents, false);

    for _ in 0..100 {
        grid = update_grid(&grid, false);
    }

    let lit_lights = count_lights(&grid);
    println!("Amount of lights on after 100 steps: {lit_lights}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d18.txt");
    let mut grid = parse_grid(&contents, true);

    for _ in 0..100 {
        grid = update_grid(&grid, true);
    }

    let lit_lights = count_lights(&grid);
    println!("Amount of lights on after 100 steps: {lit_lights}");
}
