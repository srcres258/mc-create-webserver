use std::fs;
use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::Path;
use serde_json::{Map, Value};
use crate::constants;
use crate::constants::VersionNum;
use crate::data::{JsonSerializable, TrainStation};

pub struct Database {
    version: VersionNum,
    train_stations: Vec<TrainStation>
}

impl Database {
    pub fn new(version: VersionNum, train_stations: Vec<TrainStation>) -> Self {
        Self {
            version,
            train_stations
        }
    }

    pub fn new_empty() -> Self {
        Self::new(constants::DATABASE_VERSION, Vec::new())
    }

    pub fn new_from_json(json_val: &Value) -> Option<Self> {
        let version = json_val.get("version")?.as_u64()?;
        let mut train_stations: Vec<TrainStation> = Vec::new();
        for train_station_json_val in json_val.get("train_stations")?.as_array()? {
            let train_station = TrainStation::new_from_json(train_station_json_val)?;
            train_stations.push(train_station);
        }
        Some(Self::new(version, train_stations))
    }

    pub fn load_from_file(file_path: String) -> Option<Self> {
        let mut content = String::new();
        let mut file = if Path::new(file_path.as_str()).exists() {
            File::open(file_path.clone()).ok()?
        } else {
            File::create(file_path.clone()).ok()?
        };
        let _ = file.read_to_string(&mut content); // Ignore the result intentionally.
        if content.len() == 0 {
            // If failed to read file or the file is empty, create a new database.
            Some(Self::new_empty())
        } else {
            // Process the file content otherwise.
            let json_val: Value = serde_json::from_str(content.as_str()).ok()?;
            Self::new_from_json(&json_val)
        }
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn train_stations(&self) -> &Vec<TrainStation> {
        &self.train_stations
    }

    pub fn train_stations_mut(&mut self) -> &mut Vec<TrainStation> {
        &mut self.train_stations
    }

    pub fn save_to_file(&self, file_path: String) -> Result<(), IOError> {
        let json_val = self.to_json();
        let json_val_str = json_val.to_string();
        fs::write(file_path, json_val_str)?;
        Ok(())
    }

    pub fn insert_or_modify_train_stations(&mut self, train_station: TrainStation) {
        let mut found = false;
        let mut index = 0usize;
        for (i, ts) in self.train_stations.iter().enumerate() {
            if ts.name() == train_station.name() {
                found = true;
                index = i;
                break;
            }
        }
        if found {
            self.train_stations.remove(index);
        }

        self.train_stations.push(train_station);
    }
}

impl JsonSerializable for Database {
    fn to_json(&self) -> Value {
        let mut json_val = Map::new();
        json_val.insert(String::from("version"), Value::Number(self.version.into()));

        let mut train_stations_json: Vec<Value> = Vec::new();
        for train_station in self.train_stations.iter() {
            let json_val = train_station.to_json();
            train_stations_json.push(json_val);
        }
        json_val.insert(String::from("train_stations"), Value::Array(train_stations_json));

        Value::Object(json_val)
    }
}
