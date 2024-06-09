pub mod weather {
    use std::fs;
    use std::error::Error;
    use std::path::PathBuf;
    use csv::Reader;
    use serde::{Deserialize, Serialize};
    use chrono::{Datelike, Duration, Utc};


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

    fn generate_paths() -> Vec<String> {
        let mut paths = Vec::new();
        let now = Utc::now();

        for i in 0..30 {
            let date = now - Duration::days(i);
            let path = format!(
                "./raw/{}/{:04}-{:02}/{:04}-{:02}-{:02}.txt",
                date.year(),
                date.year(),
                date.month(),
                date.year(),
                date.month(),
                date.day()
            );
            if fs::metadata(&path).is_ok() {
                paths.push(path);
            }
        }

        paths
    }

    pub fn example() -> Result<Vec<Record>, Box<dyn Error>> {
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
        let paths = generate_paths();
        //let path: PathBuf = "./raw/2023/2023-06/2023-06-07.txt".into();
        //let mut rdr = Reader::from_path("./raw/2023/2023-06/2023-06-07.txt")?;
        for path in paths {
            println!("{}", path);
            let mut rdr = Reader::from_path(path)?;
            for result in rdr.records() {
                match result.unwrap().deserialize(Some(&headers)) {
                    //Ok(record) => list.push(record),
                    Ok(record) => list.insert(0, record),
                    Err(_) => continue,
                };
            }
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