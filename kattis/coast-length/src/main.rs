use std::io::{self};

/*
 * Something is horribly wrong with my floodfill implementation (it's doing some funky DFS ordering
 * instead of BFS. But I can't be bothered to go back and fix it right now.
 *
 * The core of the problem IS just BFS, and then count the number of `ocean -> land` edges while
 * doing the floodfill. Nothing crazy.
 * */


#[derive(Copy, Clone, Debug)]
enum TerrainType {
    Water, // might be ocean, might be not
    Ocean,
    Land
}

fn read_ints() -> (u32, u32) {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf).expect("IO Error");

    let ints: Vec<u32> = buf.split_ascii_whitespace()
        .map(|c| c.parse::<u32>().expect("Failed int conversion"))
        .collect();

    // Lol this is gross
    (*ints.get(0).unwrap(), *ints.get(1).unwrap())
}

fn read_map(rows: u32, cols: u32) -> Vec<Vec<TerrainType>> {
    let mut buf = String::with_capacity(cols as usize);
    let stdin = io::stdin();

    let mut map: Vec<Vec<TerrainType>> = Vec::with_capacity(rows as usize);

    let mut first_row = Vec::with_capacity(cols as usize);
    for _i in 0..cols+2 {
        first_row.push(TerrainType::Water)
    }
    map.push(first_row);

    for _i in 0..rows {
        stdin.read_line(&mut buf).expect("IO Error");
        let mut row: Vec<TerrainType> = buf.chars().filter_map(|c| {
            match c {
                '0' => Some(TerrainType::Water),
                '1' => Some(TerrainType::Land),
                _ => None
            }
        }).collect();

        row.insert(0, TerrainType::Water);
        row.push(TerrainType::Water);

        map.push(row);
        buf.clear();
    }

    let mut last_row = Vec::with_capacity(cols as usize);
    for _i in 0..cols+2 {
        last_row.push(TerrainType::Water)
    }
    map.push(last_row);

    return map;
}

fn flood_fill(map: &mut Vec<Vec<TerrainType>>, pos: (i32, i32)) {
    // Have to use i32s so that if I subtract from 0 I get a negative number.
    let (r, c) = pos;
    println!("Marking {} {}", r, c);
    map[r as usize][c as usize] = TerrainType::Ocean;
    // TODO - Check the neighbors, recurse to cells that are Water and with valid indices
    let neighbors: [(i32, i32); 4] = [(r+1,c), (r-1,c), (r,c+1), (r,c-1)];
    // This is doing a strange and silly version of DFS. I need to BFS instead?
    for i in 0..4 {
        let (next_r, next_c) = neighbors[i];
        if next_r >= 0 && next_r < map.len() as i32 && next_c >= 0 && next_c < map[0].len() as i32 {
            match map[next_r as usize][next_c as usize] {
                TerrainType::Water => flood_fill(map, (next_r, next_c)),
                TerrainType::Ocean => return,
                TerrainType::Land => return
            }
        }
    }
}

fn main() {
    let (rows, cols) = read_ints();
    let mut map = read_map(rows, cols);
    flood_fill(&mut map, (0, 0));
    println!("{:#?}", map);
}
