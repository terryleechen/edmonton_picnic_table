use crate::setup::read_file::read_lines;

mod setup;
mod database;

fn main() {
    // create a new database
    let mut db = database::db::Database::new();

    if let Err(err) = read_lines("setup/full.csv",&mut db){
        panic!("Error occurred on read: {}", err);
    }

    println!("There are {} picnic tables!", &db.get_tables_length());
    db.edit_table_entry(10846, "Square Picnic Table".to_string(), "new_value".to_string());
    println!("The id {} table is a {}", &db.get_table(0).get_id(), &db.get_table(0).get_table_type());
    println!("There are {} Square Picnic Table", db.count_entries("Square Picnic Table".to_string()));
    println!("There are {} structural material types", db.get_structural_material_table_length());
    println!("There are {} surface material types", db.get_surface_material_table_length());
    println!("There are {} table types", db.get_table_type_table_length());
    db.report_by_ward();
    db.report_by_neighbourhood();
    db.list_table_type();
    db.list_surface_material();
    db.list_structural_material();

}
