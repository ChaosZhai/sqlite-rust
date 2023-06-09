mod schema;

use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use crate::sqlitey::database_header::Header;
use crate::sqlitey::page_header::PageHeader;
use crate::schema::Schema;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    let mut database = Vec::new();
    file.read_to_end(&mut database)?;
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            #[allow(unused_variables)]
            let page_size = u16::from_be_bytes([header[16], header[17]]);

            // You can use print statements as follows for debugging, they'll be visible when running tests.
            // println!("Logs from your program will appear here!");

            // Uncomment this block to pass the first stage
            println!("database page size: {}", page_size);

            // second stage
            let mut b_tree_header = [0; 8];
            file.read_exact(&mut b_tree_header)?;
            let table_size = u16::from_be_bytes([b_tree_header[3], b_tree_header[4]]);
            println!("number of tables: {}", table_size);
        }
        ".tables" => {
            // third stage
            // Let K be the number of cells on the btree.
            // The cell pointer array consists of K 2-byte integer offsets to the cell contents.
            // let mut file = File::open(&args[1])?;
            // let mut header = [0; 108];
            // file.read_exact(&mut header)?;
            // let page_size = u16::from_be_bytes([header[16], header[17]]);
            // let table_size = u16::from_be_bytes([b_tree_header[103], b_tree_header[104]]);
            //
            // let mut names: Vec<String> = vec![];
            // for i in 0..table_size {
            //
            // }
            // println!("{}", names.join(" "));
            let page_header = PageHeader::parse(&database[100..108])?;

                       let cell_pointers = database[108..]
                                .chunks_exact(2)
                               .take(page_header.number_of_cells.into())
                              .map(|bytes| u16::from_be_bytes(bytes.try_into().unwrap()))
                              .collect::<Vec<_>>();
                        #[allow(unused_variables)]
                            let table_names = cell_pointers
                               .into_iter()
                               .map(|cell_pointer| {
                                   let stream = &database[cell_pointer as usize..];
                                  let (_, offset) = parse_varint(stream);
                                  let (_rowid, read_bytes) = parse_varint(&stream[offset..]);
                                 parse_record(&stream[offset + read_bytes..], 5)
                                         .map(|record| Schema::parse(record).expect("Invalid record").table_name)
                                })
                            .collect::<Result<Vec<_>>>()?;

                       for name in table_names {
                               print!("{} ", name)
                       }

        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
