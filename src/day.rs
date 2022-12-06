pub use anyhow::Result;
pub trait Day {
    fn number(&self) -> u8;
    fn part01(&self) -> Result<()>;
    fn part02(&self) -> Result<()>;
}
