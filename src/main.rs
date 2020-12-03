use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum Terrain {
    Slope,
    Tree,
}

struct Piste {
    model: Vec<Vec<Terrain>>,
}

impl Piste {
    fn new_from_path<P: AsRef<Path>>(filename: P) -> Piste {
        let model = lines_to_piste_model(line_reader_to_lines(open_line_reader(filename)));
        Piste { model }
    }

    fn get_terrain(&self, top: usize, left: usize) -> Option<Terrain> {
        self.model.get(top).map(|row| row[left % row.len()])
    }
}

struct Vector {
    top: usize,
    right: usize,
}

impl Vector {
    fn new(top: usize, right: usize) -> Vector {
        Vector { top, right }
    }
}

fn open_line_reader<P: AsRef<Path>>(filename: P) -> io::Lines<io::BufReader<File>> {
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(e) => panic!("{}", e), // error is not recoverable
    }
}

fn line_reader_to_lines(
    reader: impl Iterator<Item = io::Result<String>>,
) -> impl Iterator<Item = String> {
    reader.map(|line_res| match line_res {
        Ok(line) => line,
        // Error is unrecoverable so OK to panic
        Err(e) => panic!("{}", e),
    })
}

fn lines_to_piste_model(lines: impl Iterator<Item = String>) -> Vec<Vec<Terrain>> {
    lines.map(line_to_terrain_vec).collect()
}

fn line_to_terrain_vec(line: String) -> Vec<Terrain> {
    line.chars()
        .map(|c| match c {
            '.' => Terrain::Slope,
            '#' => Terrain::Tree,
            _ => panic!("{} is not a valid terrain marker", c), // This is not recoverable
        })
        .collect()
}

fn count_trees(piste: &Piste, vector: &Vector) -> usize {
    let mut left: usize = 0;
    (0usize..piste.model.len())
        .step_by(vector.top)
        .map(|top: usize| {
            let terrain = piste.get_terrain(top, left);
            left += vector.right;
            terrain.expect("empty terrain")
        })
        .filter(|terrain: &Terrain| matches!(*terrain, Terrain::Tree))
        .count()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let piste = Piste::new_from_path(&args[1]);
        let first_result: usize = count_trees(&piste, &Vector::new(1usize, 3usize));

        let second_result: usize = vec![
            Vector::new(1, 1),
            Vector::new(1, 3),
            Vector::new(1, 5),
            Vector::new(1, 7),
            Vector::new(2, 1),
        ]
        .iter()
        .map(|v: &Vector| count_trees(&piste, v))
        .product();
        println!("You encounter {} trees", first_result);
        println!("The product of encountered trees is {}", second_result);
    } else {
        println!("input filename is required");
    }
}
