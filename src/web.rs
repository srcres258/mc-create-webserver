use crate::app;

pub fn generate_homepage() -> String {
    use rtml::*;

    let mut body_str = String::from("<ul>");
    let train_stations = app::app_instance().database().train_stations();
    for train_station in train_stations.iter() {
        body_str.push_str(format!("<li>{}<ul>", train_station.name()).as_str());
        for (index, entry) in train_station.schedule().entries().iter().enumerate() {
            body_str.push_str(format!(
                "<li>Schedule {}<ul><li>Time left: {}</li><li>Train name: {}</li><li>Train destination: {}</li></ul></li>",
                index,
                entry.time_left().unwrap_or_else(|| 0),
                entry.train_name(),
                entry.train_destination()).as_str());
        }
        body_str.push_str("</ul></li>")
    }
    body_str.push_str("</ul>");

    // Use the macros to generate some HTML
    let mut document: String = html!{
        .lang = "en",
            head!{
                title!{
                    "Title of the document"
                }
            },
            body!("$$$TEMPLATE$$$")
    }.render();
    document = document.replace("$$$TEMPLATE$$$", body_str.as_str());

    document
}