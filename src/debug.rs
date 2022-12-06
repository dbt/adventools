pub trait DebugExt: Iterator {
    fn debug(self, msg: &str) -> Self
    where
        Self: Clone + Sized,
        Self::Item: std::fmt::Display + std::fmt::Debug,
    {
        let copy = self.clone();
        let v: Vec<Self::Item> = copy.collect();
        println!("{}: {:?}", msg, v);
        return self;
    }
}

impl<I: Iterator> DebugExt for I {}
