extern crate reqwest;
extern crate tempdir;

use reqwest::header::AUTHORIZATION;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempdir::TempDir;

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
}

fn main() {
    unpack_and_read_dataset();
}
