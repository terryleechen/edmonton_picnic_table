use std::collections::HashSet;

pub struct Database {
    tables: Vec<Table>,
    table_type_table: Vec<String>,
    surface_material_table: Vec<String>,
    structural_material_table: Vec<String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            tables: Vec::new(),
            table_type_table: Vec::new(),
            surface_material_table: Vec::new(),
            structural_material_table: Vec::new(),
        }
    }

    pub fn get_tables_length(&self) -> usize {
        self.tables.len()
    }

    pub fn get_table_type_table_length(&self) -> usize {
        self.table_type_table.len()
    }

    pub fn get_surface_material_table_length(&self) -> usize {
        self.surface_material_table.len()
    }

    pub fn get_structural_material_table_length(&self) -> usize {
        self.structural_material_table.len()
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn add_table_type(&mut self, table_type: String) {
        self.table_type_table.push(table_type);
    }

    pub fn add_surface_material(&mut self, surface_material: String) {
        self.surface_material_table.push(surface_material);
    }

    pub fn add_structural_material(&mut self, structural_material: String) {
        self.structural_material_table.push(structural_material);
    }

    pub fn cleanup(&mut self) {
        self.table_type_table.sort();
        self.table_type_table.dedup();
        self.surface_material_table.sort();
        self.surface_material_table.dedup();
        self.structural_material_table.sort();
        self.structural_material_table.dedup();
    }

    pub fn get_table(&mut self, index: usize) -> &Table {
        &self.tables[index]
    }

    pub fn list_table_type(&mut self) {
        for table_type in &self.table_type_table {
            println!("table_type: {}", table_type);
        }
    }

    pub fn list_surface_material(&mut self) {
        for surface_material in &self.surface_material_table {
            println!("surface_material: {}", surface_material);
        }
    }

    pub fn list_structural_material(&mut self) {
        for structural_material in &self.structural_material_table {
            println!("structural_material: {}", structural_material);
        }
    }

    pub fn count_entries(&self, table_type: String) -> i32 {
        let mut count = 0;
        for table in &self.tables {
            if table.table_type == table_type {
                count += 1;
            }
        }
        count
    }

    // sort_by_member??

    pub fn edit_table_entry(&mut self, table_id: i32, table_type: String, member_name: String) {
        for table in &mut self.tables {
            if table.id == table_id {
                match table_type.as_ref() {
                    "Square Picnic Table" => table.set_table_type(member_name.to_string()),
                    "surface_material" => table.set_surface_material(member_name.to_string()),
                    "structural_material" => table.set_structural_material(member_name.to_string()),
                    _ => println!("No match"),
                }
            }
        }
    }

    pub fn report_by_ward(&mut self) {
        self.tables.sort_by(|a, b| a.ward.cmp(&b.ward));
        let mut seen = HashSet::new();
        for table in &self.tables {
            if !seen.contains(&table.ward) {
                println!("Ward: {}", table.ward);
                seen.insert(&table.ward);
            }
        }
    }

    pub fn report_by_neighbourhood(&self) {
        let mut neighbourhoods: Vec<String> = Vec::new();
        for table in &self.tables {
            neighbourhoods.push(table.neighbourhood_name.to_string());
        }
        neighbourhoods.sort();
        neighbourhoods.dedup();
        for neighbourhood in neighbourhoods {
            println!("Neighbourhood: {}", neighbourhood);
        }
    }
}

pub struct Table {
    id: i32,
    table_type: String,
    surface_material: String,
    structural_material: String,
    street_avenue: String,
    neighbourhood_id: i32,
    neighbourhood_name: String,
    ward: String,
    latitude: f64,
    longitude: f64,
    location_lat: String,
    location_long: String,
    geometry_point: String,
}

impl Table {
    pub fn new() -> Table {
        Table {
            id: 0,
            table_type: String::new(),
            surface_material: String::new(),
            structural_material: String::new(),
            street_avenue: String::new(),
            neighbourhood_id: 0,
            neighbourhood_name: String::new(),
            ward: String::new(),
            latitude: 0.0,
            location_lat: String::new(),
            location_long: String::new(),
            geometry_point: String::new(),
            longitude: 0.0,
        }
    }
    pub fn from(table_info: Vec<&str>) -> Table {
        Table {
            id: table_info[0].parse().unwrap(),
            table_type: table_info[1].to_owned(),
            surface_material: table_info[2].to_owned(),
            structural_material: table_info[3].to_owned(),
            street_avenue: table_info[4].to_owned(),
            neighbourhood_id: table_info[5].parse().unwrap(),
            neighbourhood_name: table_info[6].to_owned(),
            ward: table_info[7].to_owned(),
            latitude: table_info[8].parse().unwrap(),
            longitude: table_info[9].parse().unwrap(),
            location_lat: table_info[10].to_owned(),
            location_long: table_info[11].to_owned(),
            geometry_point: table_info[12].to_owned(),
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_table_type(&mut self, table_type: String) {
        self.table_type = table_type;
    }

    pub fn set_surface_material(&mut self, surface_material: String) {
        self.surface_material = surface_material;
    }

    pub fn set_structural_material(&mut self, structural_material: String) {
        self.structural_material = structural_material;
    }

    pub fn set_street_avenue(&mut self, street_aveenue: String) {
        self.street_avenue = street_aveenue;
    }

    pub fn set_neighbourhood_id(&mut self, neighbourhood_id: i32) {
        self.neighbourhood_id = neighbourhood_id;
    }

    pub fn set_neighbourhood_name(&mut self, neighbourhood_name: String) {
        self.neighbourhood_name = neighbourhood_name;
    }

    pub fn set_ward(&mut self, ward: String) {
        self.ward = ward;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn set_location_lat(&mut self, location_lat: String) {
        self.location_lat = location_lat;
    }

    pub fn set_location_long(&mut self, location_long: String) {
        self.location_long = location_long;
    }

    pub fn set_geometry_point(&mut self, geometry_point: String) {
        self.geometry_point = geometry_point;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_table_type(&self) -> &String {
        &self.table_type
    }
}

