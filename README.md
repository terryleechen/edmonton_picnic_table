This is my attempt to program my CMPT 201 C assignment in Rust.

The City of Edmonton maintains a dataset of Public Picnic Table Locations. Information about it can be found at
https://data.edmonton.ca/Facilities-and-Structures/Public-Picnic-Table-Locations/vk3s-q842

Database module:

- Tables: Vec<Table>
- Table_type_table: Vec<String>
- Surface_material_table: Vec<String>
- Structural_material_table: Vec<String>

methods

- new
- get_tables_length
- get_table_type_table_length
- get_surface_material_table_length
- get_structural_material_table_length
- add_table
- add_table_type
- add_surface_material
- add_structural_material
- cleanup
- get_table
- list_table_type
- list_surface_material
- list_structural_material
- count_entries
- edit_table_entry
- report_by_ward
- report_by_neighbourhood
- sort_by_member - not implemented

Table module:

- id: i32
- table_type: String
- surface_material: String
- structural_material: String
- street_aveenue: String
- neighbourhood_id: i32
- neighbourhood_name: String
- ward: String
- latitude: f64
- longitude: f64
- location_lat: String
- location_long: String
- geometry_point: String

methods

- new
- set_id
- set_table_type
- set_surface_material
- set_street_avenue
- set_neighbourhood_id
- set_neighbourhood_name
- set_ward
- set_latitude
- set_longitude
- set_location_lat
- set_location_long
- set_geometry_point
- get_id
- get_table_type
