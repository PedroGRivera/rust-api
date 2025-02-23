use serde::Deserialize;

//these types should be structs to account for CRUD data

#[derive(Debug, Deserialize)]
pub struct KeyVal {
    key: String,
    value: String,
}