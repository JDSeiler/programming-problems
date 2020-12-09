use std::io::{self};

enum DFSError {
    NoValidOrientations,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PipeShape {
    Null,
    Straight,
    Elbow,
    Junction,
}

#[derive(Debug, Copy, Clone)]
enum Orientation {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Vertical,
    Horizontal,
    Universal, // reserved for Junctions and Nulls and unplaced pipes
}

// Clockwise starting with North: N E S W
// 1 for open (valid), 0 for closed (invalid)
type OpenSides = (bool, bool, bool, bool);
type PipeGrid = Vec<Vec<Pipe>>;
type DoneGrid = Vec<Vec<bool>>;

#[derive(Debug, Copy, Clone)]
struct Pipe {
    shape: PipeShape,
    orientation: Orientation,
}

impl Pipe {
    fn new(shape_code: char) -> Pipe {
        match shape_code {
            'A' => Pipe {
                shape: PipeShape::Null,
                orientation: Orientation::Universal,
            },
            'B' => Pipe {
                shape: PipeShape::Straight,
                orientation: Orientation::Universal,
            },
            'C' => Pipe {
                shape: PipeShape::Elbow,
                orientation: Orientation::Universal,
            },
            'D' => Pipe {
                shape: PipeShape::Junction,
                orientation: Orientation::Universal,
            },
            _ => panic!("Unknown shape code!"),
        }
    }

    fn get_open_sides(&self) -> OpenSides {
        match self.shape {
            PipeShape::Null => (false, false, false, false),
            PipeShape::Straight => match self.orientation {
                Orientation::Vertical => (true, false, true, false),
                Orientation::Horizontal => (false, true, false, true),
                Orientation::Universal => (true, true, true, true),
                _ => panic!("Straight pipe had invalid orientation!"),
            },
            PipeShape::Elbow => match self.orientation {
                Orientation::UpRight => (true, true, false, false),
                Orientation::UpLeft => (true, false, false, true),
                Orientation::DownRight => (false, true, true, false),
                Orientation::DownLeft => (false, false, true, true),
                Orientation::Universal => (true, true, true, true),
                _ => panic!("Elbow pipe had invalid orientation!"),
            },
            PipeShape::Junction => (true, true, true, true),
        }
    }
}

fn get_size() -> Option<(u32, u32)> {
    let mut buf = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buf) {
        Ok(_) => {
            let mut nums = buf.split_whitespace();
            let rows: u32 = nums.next()?.parse().unwrap();
            let cols: u32 = nums.next()?.parse().unwrap();
            Some((rows, cols))
        }
        Err(_) => None,
    }
}

fn read_grid(dimensions: (u32, u32)) -> PipeGrid {
    let (rows, cols) = dimensions;
    let mut grid: PipeGrid = Vec::with_capacity(rows as usize);

    let mut buf = String::new();
    let stdin = io::stdin();

    for _i in 0..rows {
        let mut row: Vec<Pipe> = Vec::with_capacity(cols as usize);
        match stdin.read_line(&mut buf) {
            Ok(_) => {
                for chr in buf.trim().chars() {
                    let pipe: Pipe = Pipe::new(chr);
                    row.push(pipe);
                }
                grid.push(row);
            }
            Err(_) => {}
        }
        buf.clear();
    }
    grid
}

fn init_done_grid(dimensions: (u32, u32)) -> DoneGrid {
    let (rows, cols) = dimensions;
    let mut grid: DoneGrid = Vec::with_capacity(rows as usize);

    for _i in 0..rows {
        let row = vec![false; cols as usize];
        grid.push(row);
    }
    grid
}

fn check_north(row: &usize, col: &usize, pipe_grid: &PipeGrid) -> bool {
    return if *row < 1 {
        false
    } else {
        let north_neighbor = pipe_grid.get(*row - 1).unwrap().get(*col).unwrap();
        let neighbor_oriented_well = north_neighbor.get_open_sides().2;
        neighbor_oriented_well
    };
}

fn check_east(row: &usize, col: &usize, max_col: &usize, pipe_grid: &PipeGrid) -> bool {
    return if *col == (*max_col - 1) {
        false
    } else {
        let east_neighbor = pipe_grid.get(*row).unwrap().get(*col + 1).unwrap();
        let neighbor_oriented_well = east_neighbor.get_open_sides().3;
        neighbor_oriented_well
    };
}

fn check_south(row: &usize, col: &usize, max_row: &usize, pipe_grid: &PipeGrid) -> bool {
    return if *row == (*max_row - 1) {
        false
    } else {
        let south_neighbor = pipe_grid.get(*row + 1).unwrap().get(*col).unwrap();
        let neighbor_oriented_well = south_neighbor.get_open_sides().0;
        neighbor_oriented_well
    };
}

fn check_west(row: &usize, col: &usize, pipe_grid: &PipeGrid) -> bool {
    return if *col < 1 {
        false
    } else {
        let west_neighbor = pipe_grid.get(*row).unwrap().get(*col - 1).unwrap();
        let neighbor_oriented_well = west_neighbor.get_open_sides().1;
        neighbor_oriented_well
    };
}

fn north_neighbor_is_placed(row: &usize, col: &usize, done_grid: &DoneGrid) -> bool {
    *done_grid.get(*row - 1).unwrap().get(*col).unwrap()
}
fn east_neighbor_is_placed(row: &usize, col: &usize, done_grid: &DoneGrid) -> bool {
    *done_grid.get(*row).unwrap().get(*col + 1).unwrap()
}
fn south_neighbor_is_placed(row: &usize, col: &usize, done_grid: &DoneGrid) -> bool {
    *done_grid.get(*row + 1).unwrap().get(*col).unwrap()
}
fn west_neighbor_is_placed(row: &usize, col: &usize, done_grid: &DoneGrid) -> bool {
    *done_grid.get(*row).unwrap().get(*col - 1).unwrap()
}

fn get_valid_configurations(
    row: usize,
    col: usize,
    shape_type: &PipeShape,
    pipe_grid: &PipeGrid,
    done_grid: &DoneGrid,
    new_path: bool,
) -> Option<Vec<Orientation>> {
    let max_row = pipe_grid.len();
    let max_col = pipe_grid.get(0).unwrap().len();

    let open_directions: OpenSides = (
        check_north(&row, &col, pipe_grid),
        check_east(&row, &col, &max_col, pipe_grid),
        check_south(&row, &col, &max_row, pipe_grid),
        check_west(&row, &col, pipe_grid),
    );

    let mut orientations: Vec<Orientation> = Vec::new();
    // The extra and conditions about neighbors being placed is to ensure
    // that the chosen orientation connects to previous cells.
    // This can be overridden with a boolean flag, like in the case of starting
    // a new dfs probe in the flood fill. A straight pipe cannot possibly be
    // the first cell of a new probe (always invalid) so it doesn't get the flag.
    match shape_type {
        PipeShape::Null => orientations.push(Orientation::Universal),
        PipeShape::Straight => {
            if open_directions.0
                && open_directions.2
                && (north_neighbor_is_placed(&row, &col, &done_grid)
                    || south_neighbor_is_placed(&row, &col, &done_grid))
            {
                orientations.push(Orientation::Vertical);
            }
            if open_directions.1
                && open_directions.3
                && (east_neighbor_is_placed(&row, &col, &done_grid)
                    || west_neighbor_is_placed(&row, &col, &done_grid))
            {
                orientations.push(Orientation::Horizontal);
            }
        }
        PipeShape::Elbow => {
            if open_directions.0
                && open_directions.1
                && (north_neighbor_is_placed(&row, &col, &done_grid)
                    || east_neighbor_is_placed(&row, &col, &done_grid)
                    || new_path)
            {
                orientations.push(Orientation::UpRight);
            }
            if open_directions.1
                && open_directions.2
                && (east_neighbor_is_placed(&row, &col, &done_grid)
                    || south_neighbor_is_placed(&row, &col, &done_grid)
                    || new_path)
            {
                orientations.push(Orientation::DownRight);
            }
            if open_directions.2
                && open_directions.3
                && (south_neighbor_is_placed(&row, &col, &done_grid)
                    || west_neighbor_is_placed(&row, &col, &done_grid)
                    || new_path)
            {
                orientations.push(Orientation::DownLeft);
            }
            if open_directions.3
                && open_directions.0
                && (west_neighbor_is_placed(&row, &col, &done_grid)
                    || north_neighbor_is_placed(&row, &col, &done_grid)
                    || new_path)
            {
                orientations.push(Orientation::UpLeft);
            }
        }
        PipeShape::Junction => {
            if open_directions == (true, true, true, true) {
                orientations.push(Orientation::Universal)
            }
        }
    }
    return if orientations.len() == 0 {
        None
    } else {
        Some(orientations)
    };
}

fn scout_north(row: &usize, col: &usize, done_grid: &mut DoneGrid) -> bool {
    return if *row < 1 {
        false
    } else {
        return !*done_grid.get(*row - 1).unwrap().get(*col).unwrap();
    };
}

fn scout_east(row: &usize, col: &usize, max_col: &usize, done_grid: &mut DoneGrid) -> bool {
    return if *col == (*max_col - 1) {
        false
    } else {
        return !*done_grid.get(*row).unwrap().get(*col + 1).unwrap();
    };
}

fn scout_south(row: &usize, col: &usize, max_row: &usize, done_grid: &mut DoneGrid) -> bool {
    return if *row == (*max_row - 1) {
        false
    } else {
        return !*done_grid.get(*row + 1).unwrap().get(*col).unwrap();
    };
}
fn scout_west(row: &usize, col: &usize, done_grid: &mut DoneGrid) -> bool {
    return if *col < 1 {
        false
    } else {
        return !*done_grid.get(*row).unwrap().get(*col - 1).unwrap();
    };
}

fn get_next_cell(
    row: usize,
    col: usize,
    orientation: Orientation,
    done_grid: &mut DoneGrid,
) -> Option<(usize, usize)> {
    let max_row = done_grid.len();
    let max_col = done_grid.get(0).unwrap().len();

    match orientation {
        Orientation::UpRight => {
            if scout_north(&row, &col, done_grid) {
                return Some((row - 1, col));
            }
            if scout_east(&row, &col, &max_col, done_grid) {
                return Some((row, col + 1));
            }
            return None;
        }
        Orientation::UpLeft => {
            if scout_north(&row, &col, done_grid) {
                return Some((row - 1, col));
            }
            if scout_west(&row, &col, done_grid) {
                return Some((row, col - 1));
            }
            return None;
        }
        Orientation::DownRight => {
            if scout_east(&row, &col, &max_col, done_grid) {
                return Some((row, col + 1));
            }
            if scout_south(&row, &col, &max_row, done_grid) {
                return Some((row + 1, col));
            }
            return None;
        }
        Orientation::DownLeft => {
            if scout_south(&row, &col, &max_row, done_grid) {
                return Some((row + 1, col));
            }
            if scout_west(&row, &col, done_grid) {
                return Some((row, col - 1));
            }
            return None;
        }
        Orientation::Vertical => {
            if scout_north(&row, &col, done_grid) {
                return Some((row - 1, col));
            }
            if scout_south(&row, &col, &max_row, done_grid) {
                return Some((row + 1, col));
            }
            return None;
        }
        Orientation::Horizontal => {
            if scout_east(&row, &col, &max_col, done_grid) {
                return Some((row, col + 1));
            }
            if scout_west(&row, &col, done_grid) {
                return Some((row, col - 1));
            }
            return None;
        }
        Orientation::Universal => {
            if scout_north(&row, &col, done_grid) {
                return Some((row - 1, col));
            }
            if scout_east(&row, &col, &max_col, done_grid) {
                return Some((row, col + 1));
            }
            if scout_south(&row, &col, &max_row, done_grid) {
                return Some((row + 1, col));
            }
            if scout_west(&row, &col, done_grid) {
                return Some((row, col - 1));
            }
            return None;
        }
    };
}

fn dfs(
    row: usize,
    col: usize,
    pipe_grid: &mut PipeGrid,
    done_grid: &mut DoneGrid,
    new_path: bool,
) -> Result<(), DFSError> {
    // Get the shape of the current pipe, if we're a Null shape, mark as done immediately and return.
    let shape = pipe_grid.get(row).unwrap().get(col).unwrap().shape;
    if shape == PipeShape::Null {
        *done_grid.get_mut(row).unwrap().get_mut(col).unwrap() = true;
        return Ok(());
    }
    // Get a list of possible orientations for this cell, returns None if no
    // valid placements found
    let maybe_orientations = get_valid_configurations(
        row,
        col,
        &shape,
        pipe_grid.as_ref(),
        done_grid.as_ref(),
        new_path,
    );

    if let Some(orientations) = maybe_orientations {
        for orientation in orientations {
            // Place the pipe in the first valid orientation, then mark ourselves as done.
            let pipe = pipe_grid.get_mut(row).unwrap().get_mut(col).unwrap();
            pipe.orientation = orientation;
            *done_grid.get_mut(row).unwrap().get_mut(col).unwrap() = true;

            // Scout the neighboring cells for possible candidates
            if let Some((next_row, next_col)) = get_next_cell(row, col, orientation, done_grid) {
                // Recurse into the candidate, if it returns an error try the next orientation
                match dfs(next_row, next_col, pipe_grid, done_grid, false) {
                    Ok(_) => return Ok(()),
                    Err(_) => {
                        continue;
                    }
                }
            } else {
                // Scouting returns None if and only if all neighboring cells are done.
                // This is our base case, we've created a cycle and can continue the flood
                // fill later.

                // possible source of a bug? Maybe my invariant (None iff all neighbors are done)
                // is wrong?
                return Ok(());
            }
        }

        // If none of the orientations eventually return Ok then perhaps the problem
        // was a choice earlier in the call stack, so backtrack with an error.
        // Also reset the orientation back to default and then mark as not done.
        let pipe = pipe_grid.get_mut(row).unwrap().get_mut(col).unwrap();
        pipe.orientation = Orientation::Universal;
        *done_grid.get_mut(row).unwrap().get_mut(col).unwrap() = false;
        return Err(DFSError::NoValidOrientations);
    } else {
        // No valid placements? Obvious problem. Bubble up an error.
        return Err(DFSError::NoValidOrientations);
    }
}

fn main() {
    let dimensions = get_size().unwrap();
    let mut pipe_grid = read_grid(dimensions);
    let mut done_grid = init_done_grid(dimensions);

    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            if !done_grid
                .get(row as usize)
                .unwrap()
                .get(col as usize)
                .unwrap()
            {
                // This crazy looking thing is just to use up the Result returned by dfs.
                // Purely to make the compiler happy in case Kattis has strict settings.
                match dfs(
                    row as usize,
                    col as usize,
                    &mut pipe_grid,
                    &mut done_grid,
                    true,
                ) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
    let mut valid = true;
    for row in done_grid {
        for val in row {
            if !val {
                valid = false;
            }
        }
    }
    if valid {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
