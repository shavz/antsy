extern crate quick_protobuf;
extern crate reqwest;
extern crate tempdir;

use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Response;
use std::fs::File;
use std::io::prelude::*;
use std::io::{copy, SeekFrom};
use std::path::PathBuf;
use tempdir::TempDir;
use serde::{Serialize, Deserialize};

// Stops json

#[derive(Serialize, Deserialize)]
struct StopsRoot {
    version: String,
    locations: Vec<Locations>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct Parent {
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize)]
struct Parent1 {
    name: String,
    #[serde(rename = "type")]
    _type: String,
}
