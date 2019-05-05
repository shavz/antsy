extern crate serde;
extern crate reqwest;

use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Serialize, Deserialize};

// Trips struct

#[derive(Serialize, Deserialize)]
struct TripsRoot {
    version: String,
    #[serde(rename = "systemMessages")]
    system_messages: Vec<String>,
    journeys: Vec<Journeys>,
}

#[derive(Serialize, Deserialize)]
struct Destination {
id: String,
name: String,
#[serde(rename = "type")]
_type: String,
coord: Vec<f64>,
parent: Parent,
#[serde(rename = "arrivalTimePlanned")]
arrival_time_planned: String,
#[serde(rename = "arrivalTimeEstimated")]
arrival_time_estimated: String,
properties: Properties,
}

#[derive(Serialize, Deserialize)]
struct Downloads {
#[serde(rename = "type")]
_type: String,
url: String,
}

#[derive(Serialize, Deserialize)]
struct Fare {
tickets: Vec<String>,
zones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Infos {
timestamps: Timestamps,
priority: String,
id: String,
version: String,
#[serde(rename = "urlText")]
url_text: String,
url: String,
content: String,
subtitle: String,
properties: Properties1,
}

#[derive(Serialize, Deserialize)]
struct Journeys {
#[serde(rename = "isAdditional")]
is_additional: bool,
interchanges: i64,
legs: Vec<Legs>,
fare: Fare,
}

#[derive(Serialize, Deserialize)]
struct Legs {
duration: i64,
distance: i64,
origin: Origin,
destination: Destination,
transportation: Transportation,
infos: Vec<Infos>,
coords: Vec<Coords>,
#[serde(rename = "pathDescriptions")]
path_descriptions: Vec<PathDescriptions>,
}

#[derive(Serialize, Deserialize)]
struct Origin {
id: String,
name: String,
#[serde(rename = "type")]
_type: String,
coord: Vec<f64>,
parent: Parent,
#[serde(rename = "departureTimePlanned")]
departure_time_planned: String,
#[serde(rename = "departureTimeEstimated")]
departure_time_estimated: String,
properties: Properties,
}

#[derive(Serialize, Deserialize)]
struct TripsParent {
id: String,
name: String,
#[serde(rename = "type")]
_type: String,
}

#[derive(Serialize, Deserialize)]
struct PathDescriptions {
#[serde(rename = "turnDirection")]
turn_direction: String,
manoeuvre: String,
name: String,
coord: Vec<f64>,
duration: i64,
#[serde(rename = "cumDuration")]
cum_duration: i64,
distance: i64,
#[serde(rename = "cumDistance")]
cum_distance: i64,
#[serde(rename = "fromCoordsIndex")]
from_coords_index: i64,
#[serde(rename = "toCoordsIndex")]
to_coords_index: i64,
#[serde(rename = "skyDirection")]
sky_direction: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct Product {
class: i64,
name: String,
#[serde(rename = "iconId")]
icon_id: i64,
}

#[derive(Serialize, Deserialize)]
struct Properties {
downloads: Vec<Downloads>,
}

#[derive(Serialize, Deserialize)]
struct Properties1 {
#[serde(rename = "appliesTo")]
applies_to: String,
#[serde(rename = "stopIDglobalID")]
stop_i_dglobal_id: String,
}

#[derive(Serialize, Deserialize)]
struct Timestamps {
creation: String,
#[serde(rename = "lastModification")]
last_modification: String,
}

#[derive(Serialize, Deserialize)]
struct Transportation {
product: Product,
}


// Stops tsruct

#[derive(Serialize, Deserialize, Debug)]
struct StopsRoot {
    version: String,
    locations: Vec<Locations>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssignedStops {
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
    coord: Vec<f64>,
    parent: Parent1,
    modes: Vec<i64>,
    #[serde(rename = "connectingMode")]
    connecting_mode: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Locations {
    id: String,
    name: String,
    #[serde(rename = "disassembledName")]
    disassembled_name: String,
    coord: Vec<f64>,
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "matchQuality")]
    match_quality: i64,
    #[serde(rename = "isBest")]
    is_best: bool,
    modes: Vec<i64>,
    parent: Parent,
    #[serde(rename = "assignedStops")]
    assigned_stops: Vec<AssignedStops>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopsParent {
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopsParent1 {
    name: String,
    #[serde(rename = "type")]
    _type: String,
}


fn get_stops_for_stop_key(stop_key: &str) {
    let client = reqwest::Client::new();
    let api_key = "apikey 66oiEpcdH8zrdwW9YzJnTIlnTK7VKcmCHsdH";
    let request_url = format!("https://api.transport.nsw.gov.au/v1/tp/stop_finder?outputFormat=rapidJSON\
    &type_sf=stop&name_sf={stop}&coordOutputFormat=EPSG%3A4326&TfNSWSF=true&version=10.2.1.42", stop = stop_key);
    println!("request url is {:?}", request_url);
    let mut response = client
        .get(&request_url)
        .header(AUTHORIZATION, api_key)
        .send()
        .unwrap();

    let root: StopsRoot = response.json().unwrap();
    println!("response is {:?}", root);
}


fn main() {
    get_stops_for_stop_key("Wynyard")
}
