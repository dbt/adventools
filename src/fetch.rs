use anyhow::Result;
use dirs::home_dir;
use lazy_static::lazy_static;

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use reqwest::blocking::Client;

fn get_token() -> Result<String> {
    let mut cfg = home_dir().unwrap();
    cfg.push(".advent_token");
    let mut token = String::new();
    File::open(cfg)?.read_to_string(&mut token)?;
    let len = token.trim_end().len();
    token.truncate(len);
    Ok(token)
}

pub struct Fetcher {
    year: u32,
    client: Client,
}

impl Fetcher {
    pub fn new(year: u32) -> Fetcher {
        Fetcher {
            year,
            client: Client::new(),
        }
    }

    pub fn check(&self, day: u8) -> Result<()> {
        let file = format!("input{:02}.txt", day);
        let p = Path::new(&file);
        if p.exists() {
            Ok(())
        } else {
            self.fetch(day, &p)
        }
    }
    fn fetch(&self, day: u8, dest: &Path) -> Result<()> {
        lazy_static! {
            static ref TOKEN: String = get_token().unwrap();
        }
        println!("Fetching data for day {}...", day);
        let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, day);
        let res = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", TOKEN.to_string()))
            .send()?;
        let b = res.bytes()?;
        let mut f = File::create(dest)?;
        f.write_all(&b)?;
        Ok(())
    }
}
