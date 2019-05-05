extern crate reqwest;
extern crate serde;

use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};

// Trips struct
#[derive(Serialize, Deserialize, Debug)]
struct Destination {
    #[serde(rename = "isGlobalId")]
    is_global_id: bool,
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

#[derive(Serialize, Deserialize, Debug)]
struct Destination1 {
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Downloads {
    #[serde(rename = "type")]
    _type: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Fare {
    tickets: Vec<Tickets>,
    zones: Vec<Zones>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    properties: Properties3,
}

#[derive(Serialize, Deserialize, Debug)]
struct Journeys {
    rating: i64,
    #[serde(rename = "isAdditional")]
    is_additional: bool,
    interchanges: i64,
    legs: Vec<Legs>,
    fare: Fare,
}

#[derive(Serialize, Deserialize, Debug)]
struct Legs {
    duration: i64,
    origin: Origin,
    destination: Destination,
    transportation: Transportation,
    #[serde(rename = "stopSequence")]
    stop_sequence: Vec<StopSequence>,
    infos: Vec<Infos>,
    properties: Properties4,
}

#[derive(Serialize, Deserialize, Debug)]
struct Operator {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Origin {
    #[serde(rename = "isGlobalId")]
    is_global_id: bool,
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

#[derive(Serialize, Deserialize, Debug)]
struct Parent {
    id: String,
    name: String,
    #[serde(rename = "disassembledName")]
    disassembled_name: String,
    #[serde(rename = "type")]
    _type: String,
    parent: Parent1,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parent1 {
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    class: i64,
    name: String,
    #[serde(rename = "iconId")]
    icon_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties {
    downloads: Vec<Downloads>,
    #[serde(rename = "WheelchairAccess")]
    wheelchair_access: String,
    #[serde(rename = "AREA_NIVEAU_DIVA")]
    area_niveau_diva: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties1 {
    #[serde(rename = "isTTB")]
    is_ttb: bool,
    #[serde(rename = "tripCode")]
    trip_code: i64,
    #[serde(rename = "RealtimeTripId")]
    realtime_trip_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties2 {
//    #[serde(rename = "WheelchairAccess")]
//    wheelchair_access: String,
    #[serde(rename = "AREA_NIVEAU_DIVA")]
    area_niveau_diva: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties3 {
    #[serde(rename = "appliesTo")]
    applies_to: String,
    #[serde(rename = "stopIDglobalID")]
    stop_i_dglobal_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties4 {
    #[serde(rename = "PlanLowFloorVehicle")]
    plan_low_floor_vehicle: String,
    #[serde(rename = "PlanWheelChairAccess")]
    plan_wheel_chair_access: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties5 {
    #[serde(rename = "riderCategoryName")]
    rider_category_name: String,
    #[serde(rename = "priceTotalFare")]
    price_total_fare: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trips {
    version: String,
    #[serde(rename = "systemMessages")]
    system_messages: Vec<SystemMessages>,
    journeys: Vec<Journeys>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopSequence {
    #[serde(rename = "isGlobalId")]
    is_global_id: bool,
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
    coord: Vec<f64>,
    parent: Parent,
    properties: Properties2,
    #[serde(rename = "departureTimePlanned")]
    departure_time_planned: Option<String>,
    #[serde(rename = "arrivalTimePlanned")]
    arrival_time_planned: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemMessages {
    #[serde(rename = "type")]
    _type: String,
    module: String,
    code: i64,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tickets {
    id: String,
    name: String,
    comment: String,
    #[serde(rename = "URL")]
    _url: String,
    currency: String,
    #[serde(rename = "priceLevel")]
    price_level: String,
    #[serde(rename = "priceBrutto")]
    price_brutto: f64,
    #[serde(rename = "priceNetto")]
    price_netto: f64,
    #[serde(rename = "taxPercent")]
    tax_percent: f64,
    #[serde(rename = "fromLeg")]
    from_leg: i64,
    #[serde(rename = "toLeg")]
    to_leg: i64,
    net: String,
    person: String,
    #[serde(rename = "travellerClass")]
    traveller_class: String,
    #[serde(rename = "timeValidity")]
    time_validity: String,
    #[serde(rename = "validMinutes")]
    valid_minutes: i64,
    #[serde(rename = "isShortHaul")]
    is_short_haul: String,
    #[serde(rename = "returnsAllowed")]
    returns_allowed: String,
    #[serde(rename = "validForOneJourneyOnly")]
    valid_for_one_journey_only: String,
    #[serde(rename = "validForOneOperatorOnly")]
    valid_for_one_operator_only: String,
    #[serde(rename = "numberOfChanges")]
    number_of_changes: i64,
    #[serde(rename = "nameValidityArea")]
    name_validity_area: String,
    properties: Properties5,
}

#[derive(Serialize, Deserialize, Debug)]
struct Timestamps {
    creation: String,
    #[serde(rename = "lastModification")]
    last_modification: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transportation {
    id: String,
    name: String,
    #[serde(rename = "disassembledName")]
    disassembled_name: String,
    number: String,
    #[serde(rename = "iconId")]
    icon_id: i64,
    description: String,
    product: Product,
    operator: Operator,
    destination: Destination1,
    properties: Properties1,
}

#[derive(Serialize, Deserialize, Debug)]
struct Zones {
    net: String,
    #[serde(rename = "toLeg")]
    to_leg: i64,
    #[serde(rename = "fromLeg")]
    from_leg: i64,
    #[serde(rename = "neutralZone")]
    neutral_zone: String,
}

// Stops tsruct

#[derive(Serialize, Deserialize, Debug)]
struct Stops {
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
    parent: StopsParent1,
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
    parent: StopsParent,
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

fn get_stops_for_stop_key(stop_key: &str) -> Stops {
    let client = reqwest::Client::new();
    let api_key = "apikey 66oiEpcdH8zrdwW9YzJnTIlnTK7VKcmCHsdH";
    let request_url = format!(
        "https://api.transport.nsw.gov.au/v1/tp/stop_finder?outputFormat=rapidJSON\
         &type_sf=stop&name_sf={stop}&coordOutputFormat=EPSG%3A4326&TfNSWSF=true&version=10.2.1.42",
        stop = stop_key
    );
    println!("stops request url is {:?}", request_url);
    let mut response = client
        .get(&request_url)
        .header(AUTHORIZATION, api_key)
        .send()
        .unwrap();

    let stop: Stops = response.json().unwrap();
    stop
}

// TODO:add a type param that distinguishes between bus, rail, tram and ferries
fn get_trips_between_stops(start_stop_id: &str, end_stop_id: &str) -> Trips {
    let client = reqwest::Client::new();
    let api_key = "apikey 66oiEpcdH8zrdwW9YzJnTIlnTK7VKcmCHsdH";
    let request_url = format!("https://api.transport.nsw.gov.au/v1/tp/trip?outputFormat=rapidJSON&coordOutputFormat=EPSG%3A4326&depArrMacro=dep&itdTime=1200&type_origin=any&name_origin={orig}&type_destination=any&name_destination={dest}&calcNumberOfTrips=1&excludedMeans=checkbox&exclMOT_4=1&exclMOT_5=1&exclMOT_7=1&exclMOT_9=1&exclMOT_11=1&TfNSWTR=true&version=10.2.1.42", orig = start_stop_id, dest = end_stop_id);
    println!("trips request url is {:?}", request_url);
    let mut response = client
        .get(&request_url)
        .header(AUTHORIZATION, api_key)
        .send()
        .unwrap();
//    let text = response.text_with_charset("utf-8").unwrap();
//    println!("text is {:?}", text);
    let trips: Trips = serde_json::from_str(&text).unwrap();
    trips
}
fn main() {
    let stop1 = get_stops_for_stop_key("Wynyard");
    let stop2 = get_stops_for_stop_key("parramatta");
    let trip = get_trips_between_stops(&stop1.locations[0].id, &stop2.locations[0].id);
    println!("the trip is {:?}", trip);
}

//https://api.transport.nsw.gov.au/v1/tp/trip?outputFormat=rapidJSON&coordOutputFormat=EPSG%3A4326&depArrMacro=dep//&itdTime=1200&type_origin=any&name_origin=10101102&type_destination=any&name_destination=10101229//&calcNumberOfTrips=1&excludedMeans=checkbox&exclMOT_4=1&exclMOT_5=1&exclMOT_7=1&exclMOT_9=1&exclMOT_11=1&TfNSWTR//=true&version=10.2.1.42
