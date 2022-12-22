mod setup;
mod database;

fn main() {
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // let something = match setup::read_file::read_lines("setup/simple.csv") {
    //     Err(why) => panic!("couldn't open file: {}", why),
    //     Ok(something) => something,
    // };
    // println!("something: {:?}", something);
    
    let mut db = database::db::Database::new();
    
    setup::read_file::read_lines("setup/simple.csv",&mut db);
    
   
    println!(" db len {}", &db.get_length());
    println!(" id {}", &db.get_table(0).get_id());
    println!("{}", &db.count_entries("Square Picnic Table".to_string()));
    db.editTableEntry(10846, "Square Picnic Table".to_string(), "new_value".to_string())
    //let table = database::db::Table::new("test".to_string());
    
}
