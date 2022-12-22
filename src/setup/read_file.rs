use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::database::db::{Table, Database};

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

    let mut i = 0;
    for line in lines {
        if i == 12 {
            i = 0;
        }

        let line = match line {
            Err(why) => panic!("couldn't read line: {}", why),
            Ok(line) => line,
        };
        let line = line.split(',');
        let mut table = Table::new();

        let mut _j = 0;
        for (j,info) in line.enumerate() {
            //println!("info: {}", info);

            match j {
                0 => table.set_id(info.parse::<i32>().unwrap()),
                1 => table.set_table_type(info.to_string()),
                2 => table.set_surface_material(info.to_string()),
                3 => table.set_structural_material(info.to_string()),
                4 => table.set_street_aveenue(info.to_string()),
                5 => table.set_neighbourhood_id(info.parse::<i32>().unwrap()),
                6 => table.set_neighbourhood_name(info.to_string()),
                7 => table.set_ward(info.to_string()),
                8 => table.set_latitude(info.parse::<f64>().unwrap()),
                9 => table.set_longitude(info.parse::<f64>().unwrap()),
                10 => table.set_location_lat(info.to_string()),
                11 => table.set_location_long(info.to_string()),
                12 => table.set_geometry_point(info.to_string()),
                _ => println!("Error setting table info"),
            }
       }
       db.add_table(table);
    }
}
