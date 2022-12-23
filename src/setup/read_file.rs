use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::database::db::{Database, Table};

pub fn read_lines<P>(filename: P, db: &mut Database)
where
    P: AsRef<Path>,
{
    println!("Reading file: {}", filename.as_ref().display());
    let file = File::open(filename);
    let file = match file {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines().skip(1);
    for line in lines {
        let line = line.unwrap_or_else(|_|panic!("Line Invalid"));
        let table_info = line.split(',').collect::<Vec<&str>>();
        db.add_table_type(table_info[0].to_owned());
        db.add_surface_material(table_info[2].to_owned());
        db.add_structural_material(table_info[3].to_owned());
        db.add_table(Table::from(table_info));
    }
    db.cleanup();
}

