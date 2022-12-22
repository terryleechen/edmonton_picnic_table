
pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Database {
        Database {tables: Vec::new() }
    }

    pub fn get_length(&self) -> usize {
        self.tables.len()
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_table(&mut self, index: usize) -> &Table {
        &self.tables[index]
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

    //editTableEntry(Db,tableID,memberName, newValue)&
    pub fn editTableEntry(&self, table_id: i32, member_name: String, new_value: String) {
        for table in &self.tables {
            if table.get_id() == table_id {
                
                //println!("{}", table.get_table_type());
            }
        }
    }
}
    

pub struct Table {
    id: i32,
    table_type: String,
    surface_material: String,
    structural_material: String,
    street_aveenue: String,
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
            street_aveenue: String::new(),
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

    pub fn set_street_aveenue(&mut self, street_aveenue: String) {
        self.street_aveenue = street_aveenue;
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

    pub fn  set_latitude(&mut self, latitude: f64) {
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