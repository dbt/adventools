use std::time::Instant;

use anyhow::Result;

use crate::day::Day;

pub fn run(days: Vec<Box<dyn Day>>) -> Result<()> {
    for d in days {
        {
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
