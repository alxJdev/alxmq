use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouterMessage {
    pub route: u16
}

pub enum Routes {
    Index,
    RetrieveFromMainQue,
    RetrieveFromResponseQue,
    ToMainQue,
    ToResponseQue,
}

impl Routes {
    pub fn get_u16_value(self) -> u16 {
        match self {
            Routes::RetrieveFromMainQue => 1,
            Routes::RetrieveFromResponseQue => 2,
            Routes::ToMainQue => 3,
            Routes::ToResponseQue => 4,
            _ => 0,
        }
    }

    pub fn get_enum_by_u16(value: u16) -> Routes {
        match value {
            1 => Routes::RetrieveFromMainQue,
            2 => Routes::RetrieveFromResponseQue,
            3 => Routes::ToMainQue,
            4 => Routes::ToResponseQue,
            _ => Routes::Index,
        }
    }
}