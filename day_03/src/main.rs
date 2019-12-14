use std::collections::HashMap;
use std::fs;

const INTERSECTION_SYMBOL: i32 = std::i32::MAX;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
}

fn part_one(input: &String) -> i32 {
    let mut point_count_map: HashMap<(i32, i32), i32> = HashMap::new();
    let central_port = (1, 1);
    draw_wires(input, &mut point_count_map, central_port);

    point_count_map.iter()
        .filter(|(_, count)| **count == INTERSECTION_SYMBOL)
        .map(|(point, _)| *point)
        .map(|point| calc_manhattan_distance(central_port, point))
        .min()
        .expect("Could not compute minimum distance to central port.")
}

fn draw_wires(input: &String, mut point_count_map: &mut HashMap<(i32, i32), i32>, central_port: (i32, i32)) {
    let mut line_identity = 0;
    for line in input.lines() {
        line_identity += 1;
        draw_wire(&mut point_count_map, line_identity, line, central_port)
    }
}

fn draw_wire(mut point_count_map: &mut &mut HashMap<(i32, i32), i32>, line_identity: i32, line: &str, mut current_coordinate: (i32, i32)) -> () {
    for coordinate_str in line.split(',') {
        let direction = coordinate_str.get(..1).expect("Could not slice direction.");
        let length: i32 = coordinate_str.get(1..coordinate_str.len()).expect("Could not slice length.")
            .parse().expect("Could not parse length number.");
        for _ in 1..=length {
            match direction {
                "L" => current_coordinate.0 -= 1,
                "R" => current_coordinate.0 += 1,
                "D" => current_coordinate.1 -= 1,
                "U" => current_coordinate.1 += 1,
                _ => panic!("Invalid direction character."),
            }
            insert_or_increment_count(&mut point_count_map, current_coordinate, line_identity);
        }
    }
}

fn insert_or_increment_count(point_count_map: &mut HashMap<(i32, i32), i32>, coordinate: (i32, i32), line_identity: i32) {
    if let Some(count) = point_count_map.get_mut(&coordinate) {
        // check identity for every line to avoid intersecting with itself
        if *count != INTERSECTION_SYMBOL && *count != line_identity {
            *count = INTERSECTION_SYMBOL;
        }
    } else {
        point_count_map.insert(coordinate, line_identity);
    }
}

fn calc_manhattan_distance(central_port: (i32, i32), point: (i32, i32)) -> i32 {
    (central_port.0 - point.0).abs() + (central_port.1 - point.1).abs()
}
