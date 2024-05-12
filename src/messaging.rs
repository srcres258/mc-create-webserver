use std::ops::Index;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::data;

pub enum PackType {
    TrainStationScheduleUpdate,
    TrainStationRemoval
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainStationScheduleUpdate {
    name: String,
    entries: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainStationRemoval {
    name: String
}

impl PackType {
    pub fn of(name: &str) -> Option<PackType> {
        match name {
            "TrainStationScheduleUpdate" => Some(Self::TrainStationScheduleUpdate),
            "TrainStationRemoval" => Some(Self::TrainStationRemoval),
            _ => None
        }
    }
}

impl TrainStationScheduleUpdate {
    pub fn new(name: String, entries: Vec<String>) -> Self {
        Self { name, entries }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn entries(&self) -> &Vec<String> {
        &self.entries
    }

    pub fn parse_to_data(&self) -> Option<data::TrainStation> {
        let mut result = data::TrainStation::new_with_name(self.name.clone());

        // We assume that each schedule line is arranged as the following:
        //
        //     ttAA BB CC
        //
        // where tt represents an integer, AA, BB and CC represent strings.
        for entry in self.entries.iter() {
            let parts: Vec<_> = entry.split(' ').collect();
            if parts.len() < 3 {
                return None;
            }

            let time_left_str = *parts.index(0);
            let train_name_str = *parts.index(1);
            let train_destination_str = *parts.index(2);

            let mut time_left_num_str = String::new();
            for c in time_left_str.chars() {
                if c >= '0' && c <= '9' {
                    time_left_num_str.push(c);
                } else {
                    break;
                }
            }

            let time_left = match u32::from_str(time_left_num_str.as_str()) {
                Ok(val) => Some(val),
                Err(_) => None
            };
            let train_name = String::from(train_name_str);
            let train_destination = String::from(train_destination_str);
            let entry = data::TrainStationScheduleEntry::new(
                time_left, train_name, train_destination);

            result.schedule_mut().entries_mut().push(entry);
        }

        Some(result)
    }
}

impl TrainStationRemoval {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
