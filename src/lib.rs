pub mod weather {
    use std::fs;
    use std::error::Error;
    use csv::Reader;
    use serde::{Deserialize, Serialize};


    #[derive(Debug, Deserialize, Serialize)]
    pub struct Record {
        pub timestamp: String, 
        pub interval: i64, 
        pub humidity_inside: f64, 
        pub temperature_inside: f64, 
        pub humidity_out: f64, 
        pub temperature_out: f64, 
        pub pressure_abs: f64, 
        pub wind_avg: f64, 
        pub wind_gust: f64, 
        pub wind_dir: f64, 
        pub rain_hour: f64, 
        pub rain_year: f64
    }


    pub fn test() -> String {
        return "jippi".to_string();
    }

    pub fn read_single() {
        let contents = fs::read_to_string("./raw/2024/2024-06/2024-06-07.txt")
            .expect("Should have been able to read the file");
    }

    pub fn example() -> Result<Vec<Record>, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("./raw/2024/2024-06/2024-06-07.txt");

        let headers = csv::StringRecord::from(vec![
            "timestamp", 
            "interval", 
            "humidity_inside", 
            "temperature_inside", 
            "humidity_out", 
            "temperature_out", 
            "pressure_abs", 
            "wind_avg",
            "wind_gust", 
            "wind_dir", 
            "rain_hour", 
            "rain_year"
        ]);

        let mut list: Vec<Record> = Vec::new();

        let mut rdr = Reader::from_path("./raw/2024/2024-06/2024-06-07.txt")?;
        for result in rdr.records() {
            match result.unwrap().deserialize(Some(&headers)) {
                Ok(record) => list.push(record),
                Err(_) => continue,
            };
        }
        Ok(list)
    }

    pub fn read_files() -> Vec<std::path::PathBuf>{
        let paths = fs::read_dir("./raw/").unwrap();
        let mut list = Vec::new();

        for path in paths {
            let file_name = path.unwrap().path();
            list.push(file_name.to_owned());
        }
        return list;
    }

}