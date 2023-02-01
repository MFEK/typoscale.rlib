//! Iterator implementations and structs for different use cases

use super::*;

/// The `IndexTyposcaleIterator` struct implements the `Iterator` trait and provides a convenient way to iterate through a range of `typoscale` values, along with their indices and fraction string representations.
///
/// # Example
///
/// ```
/// use typoscale::iter::*;
/// let iterator = IndexTyposcaleIterator::new(0, 100, 10);
/// for (index, typoscale, fraction_str) in iterator {
///     println!("index: {}, typoscale: {:.2}, fraction_str: {}", index, typoscale, fraction_str);
/// }
/// ```
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

/// The `TypoScaleIterator` struct implements the `Iterator` trait and provides a convenient way to iterate through a range of `typoscale` values.
///
/// # Example
///
/// ```
/// use typoscale::iter::*;
/// let iterator = TypoScaleIterator::default();
/// for typoscale in iterator.take(100) {
///     println!("typoscale: {:.2}", typoscale);
/// }
/// ```
#[derive(Default, Copy, Clone)]
pub struct TypoScaleIterator {
    curr: usize,
}

impl Iterator for TypoScaleIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.curr.typoscale());
        self.curr += 1;
        ret
    }
}
