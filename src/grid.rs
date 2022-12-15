
use std::{ops::{Index,IndexMut}, panic};
use std::fmt;

pub struct Grid<T> where T: Clone {
    width: usize,
    height: usize,
    contents: Vec<T>,
}

pub trait CharSrc {
    fn char(&self) -> char;
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, zero: T) -> Self {
        let mut contents = vec![zero; width * height];
        contents.reserve(width * height);
        Grid { width, height, contents }
    }
    fn idx(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!("Index out of bounds");
        }
        self.width * y + x
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> fmt::Display for Grid<T>
where T: Clone + CharSrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chunks = self.contents.chunks(self.width);
        let vecs: Vec<String> = chunks.map(|ch| ch.iter().map(|t| t.char()).collect()).collect();
        f.write_str(&vecs.join("\n"))
    }
}

impl<T: Clone> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.contents[self.idx(x, y)]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = self.idx(x, y);
        &mut self.contents[idx]
    }
}
