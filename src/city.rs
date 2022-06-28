use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct City {
    pub prefecture_id: i32,
    pub id: i32,
    pub name: String,
}

impl City {
    pub fn get_cities() -> HashMap<i32, City> {
        return GET_CITIES.clone();
    }
    pub fn find_by_id(id: i32) -> Option<City> {
        return GET_CITIES.get(&id).cloned();
    }
    pub fn find_by_name(name: &str) -> Option<City> {
        let cities = &GET_CITIES;
        for (_, value) in cities.iter() {
            if value.name == name {
                return Some(value.clone());
            }
        }
        return None;
    }
}

lazy_static! {
    static ref GET_CITIES: HashMap<i32, City> = load_cities();
}

fn load_cities() -> HashMap<i32, City> {
    return include_str!("data/cities.tsv")
        .lines()
        // .skip(1)
        .map(|line| {
            let mut cols = line.split("\t");
            let prefecture_id = cols.next().unwrap().parse::<i32>().unwrap();
            let city_id = cols.next().unwrap().parse::<i32>().unwrap();
            let city_name = cols.next().unwrap().to_string();
            City {
                id: city_id,
                prefecture_id,
                name: city_name,
            }
        })
        .map(|city| {
            return (city.id, city);
        })
        .collect::<HashMap<i32, City>>();
}

#[cfg(test)]
mod tests {
    use crate::city::City;

    #[test]
    fn test_load_cities() {
        let cities = City::get_cities();
        assert_eq!(cities.len(), 1896);
        let city = City::find_by_id(13101).unwrap();
        assert_eq!(city.prefecture_id, 13);
        assert_eq!(city.id, 13101);
        let prefecture = City::find_by_id(0).unwrap_or(City::default());
        assert_eq!(prefecture.id, 0);
        let city = City::find_by_name("堺市堺区").unwrap_or(City::default());
        assert_eq!(city.id, 27141);
        assert_eq!(city.name, "堺市堺区");
    }
}
