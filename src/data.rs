use serde_json::{Map, Value};

#[derive(Clone)]
pub struct TrainStationScheduleEntry {
    time_left: Option<u32>, // Unit: second, None if unknown.
    train_name: String,
    train_destination: String
}

#[derive(Clone)]
pub struct TrainStationSchedule {
    entries: Vec<TrainStationScheduleEntry>
}

#[derive(Clone)]
pub struct TrainStation {
    name: String,
    schedule: TrainStationSchedule
}

pub trait JsonSerializable {
    fn to_json(&self) -> Value;
}

impl TrainStationScheduleEntry {
    pub fn new(
        time_left: Option<u32>,
        train_name: String,
        train_destination: String
    ) -> Self {
        Self {
            time_left,
            train_name,
            train_destination
        }
    }

    pub fn new_empty() -> Self {
        Self::new(None, String::from(""), String::from(""))
    }

    pub fn new_from_json(json_val: &Value) -> Option<Self> {
        let time_left = Some(json_val.get("time_left")?.as_u64()? as u32);
        let train_name = json_val.get("train_name")?.as_str()?.to_string();
        let train_destination = json_val.get("train_destination")?.as_str()?.to_string();
        Some(Self::new(time_left, train_name, train_destination))
    }

    pub fn time_left(&self) -> Option<u32> {
        self.time_left
    }

    pub fn train_name(&self) -> &str {
        &self.train_name
    }

    pub fn train_destination(&self) -> &str {
        &self.train_destination
    }

    pub fn set_time_left(&mut self, time_left: Option<u32>) {
        self.time_left = time_left;
    }

    pub fn set_train_name(&mut self, train_name: String) {
        self.train_name = train_name;
    }

    pub fn set_train_destination(&mut self, train_destination: String) {
        self.train_destination = train_destination;
    }
}

impl JsonSerializable for TrainStationScheduleEntry {
    fn to_json(&self) -> Value {
        serde_json::json!({
            "time_left": self.time_left,
            "train_name": self.train_name,
            "train_destination": self.train_destination
        })
    }
}

impl TrainStationSchedule {
    pub fn new(entries: Vec<TrainStationScheduleEntry>) -> Self {
        Self { entries }
    }

    pub fn new_empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn new_from_json(json_val: &Value) -> Option<Self> {
        let mut entries: Vec<TrainStationScheduleEntry> = Vec::new();
        for entries_json_val in json_val.get("entries")?.as_array()? {
            let entry = TrainStationScheduleEntry::new_from_json(entries_json_val)?;
            entries.push(entry);
        }
        Some(Self::new(entries))
    }

    pub fn entries(&self) -> &Vec<TrainStationScheduleEntry> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut Vec<TrainStationScheduleEntry> {
        &mut self.entries
    }
}

impl JsonSerializable for TrainStationSchedule {
    fn to_json(&self) -> Value {
        let mut entries_json: Vec<Value> = Vec::new();
        for entry in self.entries.iter() {
            let json = entry.to_json();
            entries_json.push(json);
        }
        let mut json_val = Map::new();
        json_val.insert(String::from("entries"), Value::Array(entries_json));
        Value::Object(json_val)
    }
}

impl TrainStation {
    pub fn new(name: String, schedule: TrainStationSchedule) -> Self {
        Self { name, schedule }
    }

    pub fn new_with_name(name: String) -> Self {
        Self::new(name, TrainStationSchedule::new_empty())
    }

    pub fn new_empty() -> Self {
        Self::new_with_name(String::from(""))
    }

    pub fn new_from_json(json_val: &Value) -> Option<Self> {
        let name = json_val.get("name")?.as_str()?.to_string();
        let schedule = TrainStationSchedule::new_from_json(json_val.get("schedule")?)?;
        Some(Self::new(name, schedule))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn schedule(&self) -> &TrainStationSchedule {
        &self.schedule
    }

    pub fn schedule_mut(&mut self) -> &mut TrainStationSchedule {
        &mut self.schedule
    }
}

impl JsonSerializable for TrainStation {
    fn to_json(&self) -> Value {
        let mut json_val = Map::new();
        json_val.insert(String::from("name"), Value::String(self.name.clone()));
        json_val.insert(String::from("schedule"), self.schedule.to_json());
        Value::Object(json_val)
    }
}
