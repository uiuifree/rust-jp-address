extern crate core;

use calamine::{Reader, Xlsx, open_workbook, DataType};
use csv::{ WriterBuilder};
use jp_address_search::city::City;
fn main() {
    let mut excel: Xlsx<_> = open_workbook("./storage/000730858.xlsx").unwrap();
    let mut cities = vec![];
    let mut ignore_city = vec![];
    if let Some(Ok(r)) = excel.worksheet_range("H30.10.1政令指定都市") {
        let mut is_header = true;
        for row in r.rows() {
            if is_header {
                is_header = false;
                continue;
            }
            let city = row_to_city(row);
            if city.is_none() {
                continue;
            }
            let city = city.unwrap();
            if city.name.ends_with("市") {
                ignore_city.push(city.name);
                continue;
            }
            cities.push(city);
        }
    }
    if let Some(Ok(r)) = excel.worksheet_range("R1.5.1現在の団体") {
        let mut is_header = true;
        for row in r.rows() {
            if is_header {
                is_header = false;
                continue;
            }
            let city = row_to_city(row);
            if city.is_none() {
                continue;
            }
            let city = city.unwrap();
            if ignore_city.contains(&city.name){
                continue;
            }
            cities.push(city);
        }
    }
    // println!("{:?}",vec);
    let mut wtr  =WriterBuilder::new()
        .delimiter(b'\t')
        .from_path("src/data/cities.tsv")
        .expect("error write csv");

    // let mut wtr = ;
    cities.sort_by(|a, b| a.id.cmp(&b.id));
    for city in cities {
        println!("{} {} {}", city.prefecture_id, city.id, city.name);
        wtr.write_record(&[city.prefecture_id.to_string(), city.id.to_string(), city.name]).expect("error write csv");
    }
    wtr.flush().expect("flush");
}
fn row_to_city(row: &[DataType]) -> Option<City> {
    let mut prefecture_id = "".to_string();
    for str in row[0].to_string().as_str().chars() {
        prefecture_id = format!("{}{}", prefecture_id, str.to_string());
        if 2 <= prefecture_id.chars().count() {
            break;
        }
    }
    let prefecture_id: i32 = prefecture_id.parse().unwrap();
    let city_id: i32 = row[0].to_string().parse().unwrap();
    let city_id: i32 = city_id / 10;
    let city_name = row[2].to_string();
    if city_name.is_empty() {
        return None;
    }

    Some(City {
        id: city_id,
        prefecture_id: prefecture_id,
        name: city_name,
    })
}



