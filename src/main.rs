extern crate html5ever;

mod fragment;
mod row;
mod table;
mod month;
mod error;

use soup::prelude::*;
use html5ever::rcdom::{Handle};
use itertools::{Itertools};

use crate::fragment::*;
use crate::row::*;
use crate::table::*;
use crate::error::*;

fn main() {
    for table in get_steam_videocard_tables() {
        println!("{:?}", table);
    }
}

fn get_steam_videocard_tables() -> Vec<Table> {
    let src = std::path::Path::new("videocard.html");

    if !src.exists() {
        let body = get_body()
            .expect("Unable to fetch HTML");

        std::fs::write(src, body).unwrap();
    }

    let body = std::fs::read_to_string(src)
        .expect("Unable to read cached HTML");

    let soup = Soup::new(&body);

    let root = soup.tag("div")
        .attr("id", "sub_stats")
        .find()
        .unwrap();

    let frags: Vec<Fragment> = root.children()
        .filter(|e| e.is_element())
        .filter(|e| e.name() == "div")
        .map(|e| {
                let class = get_class(&e);
                let value = get_text(&e);

                match class.as_str() {
                    "substats_col_left col_header"
                        => vec![Fragment::TableName(value)],
                    "substats_col_month_last_pct col_header"
                        => vec![Fragment::TableHeader(value)],
                    "substats_row row_0" | "substats_row row_1"
                        => get_row_parts(e),
                    _ => vec![],
                }
            })
        .flatten()
        .collect();

    let chunk_size = frags.iter()
        .take_while(|f| Fragment::is_table_fragment(&f))
        .count();

    let mut index: u8 = 0;
    let cs: Vec<(u8, &[Fragment])> = frags.chunks(chunk_size)
        .map(|chunk| {
            if chunk.iter().all(Fragment::is_table_fragment) {
                index += 1;
            }
            (index, chunk)
        })
        .collect();

    let mut tables = Vec::new();
        
    for (_, group) in &cs.into_iter().group_by(|t| t.0) {
        let values: Vec<&[Fragment]> = group.map(|t| t.1).collect();
        
        let (table_fragments, row_fragments): (Vec<&[Fragment]>, Vec<&[Fragment]>) = values.into_iter()
            .partition(|v| v.into_iter().all(Fragment::is_table_fragment));

        let (rows, row_errors): (Vec<Row>, Vec<Error>) = row_fragments.iter()
            .map(|fs| Row::from(fs))
            .partition_result();

        assert_eq!(0, row_errors.len(), "Error building rows. Errors: {:?}", row_errors);

        if let Some(fs) = table_fragments.first() {
            if let Ok(table) = Table::from(fs, rows) {
                tables.push(table);
            } // TODO print errors
        }
    };

    tables
}

fn get_text(e: &Handle) -> String {
    String::from(e.text().trim())
}

fn get_class(e: &Handle) -> String {
    e.attrs().get("class")
        .cloned()
        .unwrap_or(String::from(""))
}

fn get_row_parts(element: Handle) -> Vec<Fragment> {
    element.children()
        .filter(|e| e.is_element())
        .map(|e| {
            let class = get_class(&e);
            let value = String::from(e.text().trim());
            match class.as_str() {
                "substats_col_left" =>
                    Option::Some(Fragment::RowName(value)),
                "substats_col_month_last_pct" =>
                    Option::Some(Fragment::RowValue(value)),
                _ => Option::None,
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

fn get_body() -> Result<String, ureq::Error> {
    let body: String = ureq::get("https://web.archive.org/web/20220226092434/https://store.steampowered.com/hwsurvey/videocard/")
        .call()?
        .into_string()?;

    Ok(body)
}