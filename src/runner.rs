use std::time::Instant;
use std::env;

use anyhow::Result;

use crate::{day::Day, fetch::Fetcher};

fn day_matcher() -> Box<dyn Fn(u8) -> bool> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        return Box::new(|_| true);
    }
    let num: Result<u8, _> = args[1].parse();
    if num.is_err() {
        eprint!("Invalid argument: {}", args[1]);
        return Box::new(|_| false);
    }
    let val = num.unwrap();
    return Box::new(move |u| u == val);
}

pub fn run(year: u32, days: Vec<Box<dyn Day>>) -> Result<()> {
    let fetcher = Fetcher::new(year);
    let matcher = day_matcher();
    for d in days {
        {
            if !matcher.as_ref()(d.number()) {
                continue;
            }
            fetcher.check(d.number())?;
            print!("Day {} part 1: ", d.number());
            let i1 = Instant::now();
            let res1 = d.part01();
            let dur1 = i1.elapsed();
            println!("Elapsed time: {:?}", dur1);
            res1?;
            println!()
        }

        print!("Day {} part 2: ", d.number());
        let i2 = Instant::now();
        let res2 = d.part02();
        let dur2 = i2.elapsed();
        println!("Elapsed time: {:?}", dur2);
        res2?;
        println!()
    }
    Ok(())
}
