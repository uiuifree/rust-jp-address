use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Prefecture {
    pub id: i32,
    pub name: String,
    pub short_name: String,
    pub en_name: String,
}

impl Prefecture {
    pub fn get_prefectures() -> HashMap<i32, Prefecture> {
        return GET_PREFECTURES.clone();
    }
    pub fn find_by_id(id: i32) -> Option<Prefecture> {
        return GET_PREFECTURES.get(&id).cloned();
    }
    pub fn find_by_name(name: &str) -> Option<Prefecture> {
        let mut prefectures = GET_PREFECTURES.clone();
        for (_, value) in prefectures.iter_mut() {
            if value.name == name {
                return Some(value.clone());
            }
        }
        return None;
    }
}

lazy_static! {
    static ref GET_PREFECTURES: HashMap<i32, Prefecture> = load_prefectures();
}

fn load_prefectures() -> HashMap<i32, Prefecture> {
    return include_str!("data/prefectures.tsv")
        .lines()
        // .skip(1)
        .map(|line| {
            let mut cols = line.split("\t");
            let prefecture_id = cols.next().unwrap().parse::<i32>().unwrap();
            let prefecture_short_name = cols.next().unwrap().to_string();
            let prefecture_name = cols.next().unwrap().to_string();
            let prefecture_en = cols.next().unwrap().to_string();
            Prefecture {
                id: prefecture_id,
                short_name: prefecture_short_name,
                name: prefecture_name,
                en_name: prefecture_en,
            }
        })
        .map(|prefecture| {
            return (prefecture.id, prefecture);
        })
        .collect::<HashMap<i32, Prefecture>>();
}

#[cfg(test)]
mod tests {
    use crate::prefecture::Prefecture;

    #[test]
    fn test_load_prefectures() {
        let prefectures = Prefecture::get_prefectures();
        assert_eq!(prefectures.len(), 47);
        let prefecture = Prefecture::find_by_id(1).unwrap();
        assert_eq!(prefecture.id, 1);
        let prefecture = Prefecture::find_by_id(0).unwrap_or(Prefecture::default());
        assert_eq!(prefecture.id, 0);
        let prefecture = Prefecture::find_by_name("東京都").unwrap_or(Prefecture::default());
        assert_eq!(prefecture.id, 13);
        assert_eq!(prefecture.name, "東京都");
        assert_eq!(prefecture.short_name, "東京");
        assert_eq!(prefecture.en_name, "tokyo");
    }
}
