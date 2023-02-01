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

/// Yields the [`f64::round`] of [`TypoScaleIterator`].
///
/// ```
/// use typoscale::iter::TypoScaleIntIterator;
/// let mut i = TypoScaleIntIterator::default();
/// assert_eq!(i.nth(5).unwrap(), 2);
/// ```
#[derive(Default, Copy, Clone)]
pub struct TypoScaleIntIterator {
    curr: usize,
}

impl Iterator for TypoScaleIntIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.curr.int_typoscale());
        self.curr += 1;
        ret
    }
}

/// Yields the fractional string representation of [`TypoScaleIterator`] (_id est_, [`TypoScale::fraction_str`]).
///
/// ```
/// use typoscale::iter::TypoScaleStringIterator;
/// let mut i = TypoScaleStringIterator::default();
/// assert_eq!(&i.nth(6).unwrap(), "2 ⅕");
/// ```
#[derive(Default, Copy, Clone)]
pub struct TypoScaleStringIterator {
    curr: usize,
}

impl Iterator for TypoScaleStringIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.curr.fraction_str());
        self.curr += 1;
        ret
    }
}

/// This is a special iterator which follows the _traditional European rules_ (the LCG rules:
/// Latin–Cyrillic–Greek).
///
/// See <https://spencermortensen.com/articles/typographic-scale/>
#[derive(Copy, Clone)]
pub struct TypoScaleLcgIterator {
    curr: usize,
    last_ret: usize,
}

impl Default for TypoScaleLcgIterator {
    fn default() -> Self {
        Self {
            curr: 1,
            last_ret: 1
        }
    }
}

impl Iterator for TypoScaleLcgIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 0 {
            return None
        }
        let mut last_ret = self.last_ret;
        if last_ret == 28 {
            self.curr += 2;
            self.last_ret = 36usize;
            return Some(30usize);
        } else if last_ret == 42 {
            self.curr += 2;
            self.last_ret = 49usize;
            return Some(48usize);
        } else if last_ret == 49 {
            self.curr += 2;
            self.last_ret = 72usize;
            return Some(60usize);
        } else if last_ret == 84 {
            self.curr += 2;
            self.last_ret = 96usize;
            return Some(84usize);
        }
        while last_ret == self.last_ret {
            self.curr += 1;
            last_ret = self.last_ret;
            self.last_ret = self.curr.int_typoscale();
        }
        Some(last_ret)
    }
}
