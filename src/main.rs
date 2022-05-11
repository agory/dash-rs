use quick_xml::{de::from_str, se::to_string};

use std::fs::File;
use std::io::prelude::*;

use crate::models::mpd::MPD;
mod models;
mod serde_custom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("resources\\hdeindex-1.mpd")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(&['\n'][..], "");

    let mpd: MPD = from_str(&contents).unwrap();
    println!("{}", &mpd);
    let _contents_serializer = to_string(&mpd)?;
    // assert_eq!(contents, contents_serializer);
    Ok(())
}
