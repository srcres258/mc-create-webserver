use askama::Template;
use crate::app;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    train_stations: Vec<(String, Vec<(String, String, String)>)>
}

pub fn generate_homepage() -> String {
    let mut temp_train_stations: Vec<(String, Vec<(String, String, String)>)> = Vec::new();

    let train_stations = app::app_instance().database().train_stations();
    for train_station in train_stations.iter() {
        let mut temp_schedules: Vec<(String, String, String)> = Vec::new();
        for entry in train_station.schedule().entries().iter() {
            temp_schedules.push((
                format!("{}", entry.time_left().unwrap_or(0)),
                entry.train_name().to_string(),
                entry.train_destination().to_string()
            ));
        }
        temp_train_stations.push((
            train_station.name(),
            temp_schedules
        ));
    }

    let temp = IndexTemplate {
        train_stations: temp_train_stations
    };
    let document = temp.render().unwrap();

    document
}