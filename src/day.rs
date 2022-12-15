use std::str::FromStr;

pub use anyhow::Result;

use crate::prelude::{read_lines, parse_lines};
pub trait Day {
    fn number(&self) -> u8;
    fn part01(&self) -> Result<()>;
    fn part02(&self) -> Result<()>;
    fn input(&self) -> Result<Vec<String>> {
        read_lines(&format!("input{:02}.txt", self.number()))
    }
}

pub trait DayParsed: Day {
    fn input_as<T: FromStr>(&self) -> Result<Vec<T>>
        where T: FromStr,
        T: FromStr<Err = anyhow::Error> {
        parse_lines::<T>(&self.input()?)
    }

}
