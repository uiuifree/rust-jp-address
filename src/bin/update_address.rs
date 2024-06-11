use csv::{ReaderBuilder, WriterBuilder};
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path("./storage/KEN_ALL_UTF8.csv")
        .expect("not open");
    let mut hash = HashMap::new();
    let mut address_list = VecDeque::new();
    let mut pairs: Vec<String> = vec![];
    for record in reader.records() {
        let record = record.unwrap();
        let base_city = &record[0];
        let zip = &record[2];
        let mut prefecture_id = "".to_string();
        for str in base_city.to_string().as_str().chars() {
            prefecture_id = format!("{}{}", prefecture_id, str.to_string());
            if 2 <= prefecture_id.chars().count() {
                break;
            }
        }
        let prefecture_id: i32 = prefecture_id.parse().unwrap();
        let city_id: i32 = base_city.to_string().parse().unwrap();
        let prefecture_name = &record[6];
        let city_name = &record[7];
        let is_street_no = &record[11];
        let mut street_address = (&record[8]).to_string();
        // let numbers = HashMap::from([
        //     ("０", "0"),
        //     ("１", "1"),
        //     ("２", "2"),
        //     ("３", "3"),
        //     ("４", "4"),
        //     ("５", "5"),
        //     ("６", "6"),
        //     ("７", "7"),
        //     ("８", "8"),
        //     ("９", "9"),
        // ]);
        street_address = street_address
            .chars()
            .map(|q| {
                let numbers = HashMap::from([
                    ("０", "0"),
                    ("１", "1"),
                    ("２", "2"),
                    ("３", "3"),
                    ("４", "4"),
                    ("５", "5"),
                    ("６", "6"),
                    ("７", "7"),
                    ("８", "8"),
                    ("９", "9"),
                ]);
                let mut q = q.to_string();
                for (k, v) in numbers {
                    q = q.replace(k, v);
                }
                return q;
            })
            .collect::<String>();

        if street_address == "以下に掲載がない場合" {
            street_address = "".to_string();
        }
        // if zip != "0893737" {
        //     continue;
        // }

        let value = street_address.to_string();
        let mut street_value = value.split('（');

        let street_value_base = street_value.next();
        let street_value_append = street_value.next();
        if street_value_base.is_some() {
            street_address = street_value_base.unwrap().to_string();
            // 番地以外
            if is_street_no != "1" {
                if street_value_append.is_some() {
                    let mut is_add_append = true;

                    let street_value_append = street_value_append.unwrap().to_string();
                    if street_value_append.contains("、") {
                        is_add_append = false;
                    }
                    for i in 0..10 {
                        if street_value_append.contains(&i.to_string()) {
                            is_add_append = false;
                        }
                    }

                    if is_add_append {
                        street_address += street_value_append.as_str();
                    }
                }
            }
        }
        street_address = street_address.replace("（", "").replace("）", "");
        street_address = street_address.replace("その他", "");
        if street_address.contains("、") {
            street_address = "".to_string();
        }

        let key = zip.to_string() + prefecture_name + city_name + street_address.as_str();
        if pairs.contains(&key) {
            continue;
        }
        pairs.push(key);
        // address_list.push(ZipAddress{
        // })
        // println!("{:?}", record);
        let i = hash.entry(street_address.to_string()).or_insert(0);
        *i += 1;
        address_list.push_back(ZipAddress {
            zip: zip.to_string(),
            prefecture_id,
            city_id,
            prefecture_name: prefecture_name.to_string(),
            city_name: city_name.to_string(),
            street_address: street_address.to_string(),
        });
    }
    for (k, i) in hash {
        if i >= 20 {
            println!("{} {}", i, k);
        }
        // 以下に掲載がない場合
    }

    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .from_path("src/data/zipcode.tsv")
        .expect("error write csv");

    // let mut wtr = ;
    // cities.sort_by(|a, b| a.id.cmp(&b.id));

    wtr.write_record(&[
        "zip",
        "prefecture_id",
        "city_id",
        "prefecture_name",
        "city_name",
        "street_address",
    ])
    .expect("error write csv");
    for address in address_list {
        // println!("{} {} {} {}", city.prefecture_id, city.id, city.major_city_id,city.name);
        wtr.write_record(&[
            address.zip,
            address.prefecture_id.to_string(),
            address.city_id.to_string(),
            address.prefecture_name,
            address.city_name,
            address.street_address,
        ])
        .expect("error write csv");
    }
    wtr.flush().expect("flush");
}

#[derive(Debug)]
struct ZipAddress {
    zip: String,
    prefecture_id: i32,
    city_id: i32,
    prefecture_name: String,
    city_name: String,
    street_address: String,
}
