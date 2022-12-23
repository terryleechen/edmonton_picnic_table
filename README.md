This is my attempt to program my CMPT 201 C assignment in Rust.

The City of Edmonton maintains a dataset of Public Picnic Table Locations. Information about it can be found at
https://data.edmonton.ca/Facilities-and-Structures/Public-Picnic-Table-Locations/vk3s-q842

Database module:

- Tables: Vec<Table>
- Table_type_table: Vec<String>
- Surface_material_table: Vec<String>
- Structural_material_table: Vec<String>

methods

- New
- Get_tables_length
- Get_table_type_table_length
- Get_surface_material_table_length
- Get_structural_material_table_length
- Add_table
- Add_table_type
- Add_surface_material
- Add_structural_material
- Cleanup
  Get_table
  List_table_type
  List_surface_material
  List_structural_material
  Count_entries
  Edit_table_entry
  Report_by_ward
  Report_by_neighbourhood
  Sort_by_member - not implemented

Table module:
id: i32
table_type: String
surface_material: String
structural_material: String
street_aveenue: String
neighbourhood_id: i32
neighbourhood_name: String
ward: String
latitude: f64
longitude: f64
location_lat: String
location_long: String
geometry_point: String

Methods
New
Set_id
Set_table_type
Set_surface_material
Set_street_avenue
Set_neighbourhood_id
Set_neighbourhood_name
Set_ward
Set_latitude
Set_longitude
Set_location_lat
Set_location_long
Set_geometry_point
Get_id
get_table_type
