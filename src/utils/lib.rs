use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}


#[derive(Deserialize)]
pub struct Item_Id {
    pub id: u32,
}

