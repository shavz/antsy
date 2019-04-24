extern crate reqwest;
extern crate tempdir;
extern crate protobuf_codegen_pure;

use reqwest::header::AUTHORIZATION;
use std::fs::File;
use std::io::prelude::*;
use std::io::{copy, SeekFrom};
use std::path::PathBuf;
use tempdir::TempDir;

fn unpack_gtfs_zip_archive(file: &mut File) {
    let mut zip_reader = zip::ZipArchive::new(File::open("sydneytrains.zip").unwrap()).unwrap();

    for i in 0..zip_reader.len() {
        let mut temp_file = zip_reader.by_index(i).unwrap();
        println!("Filename: {}", temp_file.name());
        let first_byte = temp_file.bytes().next().unwrap().unwrap();
        println!("{}", first_byte);
    }
}

fn unpack_and_read_dataset() {
    let tmp_dir = TempDir::new_in("src", "trains").unwrap();
    let client = reqwest::Client::new();
    let api_key = "apikey 66oiEpcdH8zrdwW9YzJnTIlnTK7VKcmCHsdH";
    let target = "https://api.transport.nsw.gov.au/v1/gtfs/schedule/sydneytrains";

    let mut response = client
        .get(target)
        .header(AUTHORIZATION, api_key)
        .send()
        .unwrap();

    let mut dest = {
        let mut fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin")
            .to_owned();

        fname.push_str(".zip");

        println!("file to download: '{}'", fname);
        let fname = PathBuf::from(".").join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).unwrap()
    };
    println!("destination is {:?}", dest);
    let bytes_copied = copy(&mut response, &mut dest).unwrap();
    println!("total bytes received were {:?}", bytes_copied);
    unpack_gtfs_zip_archive(&mut dest);
}

fn main() {

//    unpack_and_read_dataset();

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["protos/gtfs_realtime.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
