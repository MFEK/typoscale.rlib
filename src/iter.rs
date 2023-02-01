//! Iterator implementations and structs for different use cases

use super::*;

#[derive(Copy, Clone, Debug)]
pub struct IndexTyposcaleIterator
{
    start: usize,
    end: usize,
    step: usize,
}

impl IndexTyposcaleIterator
{
    pub fn new(start: usize, end: usize, step: usize) -> IndexTyposcaleIterator {
        IndexTyposcaleIterator {
            start,
            end,
            step,
        }
    }
}

impl Iterator for IndexTyposcaleIterator
{
    type Item = (usize, f64, String);
    fn next(&mut self) -> Option<(usize, f64, String)> {
        if self.start.to_f64().unwrap() > self.end.to_f64().unwrap() {
            return None;
        }

        let typoscale = self.start.typoscale();
        let index = self.start.to_usize().unwrap();
        let fraction_str = self.start.fraction_str();

        self.start = (self.start.to_f64().unwrap() + self.step.to_f64().unwrap()) as usize;

        Some((index, typoscale, fraction_str))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TyposcaleIterator
{
    start: usize,
    end: usize,
    step: usize,
}

impl TyposcaleIterator
{
    pub fn new(start: usize, end: usize, step: usize) -> TyposcaleIterator {
        TyposcaleIterator {
            start,
            end,
            step,
        }
    }
}

impl Iterator for TyposcaleIterator
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.start.to_f64().unwrap() > self.end.to_f64().unwrap() {
            return None;
        }

        let typoscale = self.start.typoscale().to_usize().unwrap();

        self.start = (self.start.to_f64().unwrap() + self.step.to_f64().unwrap()) as usize;

        Some(typoscale)
    }
}
