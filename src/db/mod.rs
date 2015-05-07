// Module to handle db functionality for the server.
mod txtdb;

pub mod db {
    pub fn open_table(table_name: &str) -> TxtTable {
        return TxtTable::new(table_name);
    }
}