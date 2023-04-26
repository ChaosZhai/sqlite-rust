pub struct Schema {
    kind_: String,
    name_: String,
    table_name_: String,
    root_page_: u8,
    sql_: String,
}

impl Schema {
    pub fn parse(record: Vec<Vec<u8>>) -> Option<Self> {
    let mut items = record.into_iter();
    let kind = items.next()?;
    let name = items.next()?;
    let table_name = items.next()?;
    let root_page = *items.next()?.get(0)?;
    let sql = items.next()?;

    let schema = Self {
        kind_: String::from_utf8_lossy(&kind).to_string(),
        name_: String::from_utf8_lossy(&name).to_string(),
        table_name_: String::from_utf8_lossy(&table_name).to_string(),
        root_page_,
        sql_: String::from_utf8_lossy(&sql).to_string(),
    };
    Some(schema)
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name_
    }
}

