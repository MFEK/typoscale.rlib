#![doc = include_str!("../README.md")]
/*
Python prototype:
import math
typoscale = lambda i: 1 * 2**(i/5)
fifths=list("⅕⅖⅗⅘")
fifths=dict({i+1: fifths[i] for i in range(4)})
fifths[0]=''
tenths=["½" if i%5==0 else str(i)+"⁄10" if not i%2==0 else fifths[(i/2)] for i in range(1,10)]
tenths=dict({i+1: tenths[i] for i in range(9)})
tenths[0]=''
print(", ".join([f"{i} {tenths[int(j*10)]}" for i, j in [(math.floor(typoscale(i)), math.fmod(typoscale(i),1.0)) for i in range(0,64)]]))
*/
/*
Yields:
1 , 1 1⁄10, 1 3⁄10, 1 ½, 1 7⁄10, 2 , 2 ⅕, 2 ⅗, 3 , 3 ⅖, 4 , 4 ½, 5 ⅕, 6 , 6 9⁄10, 8 , 9 1⁄10, 10 ½, 12 1⁄10, 13 9⁄10, 16 , 18 3⁄10, 21 1⁄10, 24 ⅕, 27 ⅘, 32 , 36 7⁄10, 42 ⅕, 48 ½, 55 7⁄10, 64 , 73 ½, 84 ⅖, 97 , 111 ⅖, 128 , 147 , 168 ⅘, 194 , 222 ⅘, 256 , 294 , 337 7⁄10, 388 , 445 7⁄10, 512 , 588 1⁄10, 675 ½, 776 , 891 ⅖, 1024 , 1176 ⅕, 1351 1⁄10, 1552 , 1782 ⅘, 2048 , 2352 ½, 2702 3⁄10, 3104 1⁄10, 3565 7⁄10, 4096 , 4705 , 5404 7⁄10, 6208 3⁄10
*/

pub mod iter;

use num_traits::{ToPrimitive, int::PrimInt};

const FIFTHS: [&str; 5] = ["", "⅕", "⅖", "⅗", "⅘"];
const NARROW: &str = " ";

/// By default, on the standard types like `usize`, this yields 1, 1 1⁄10, 1 3⁄10, 1 ½, 1 7⁄10, 2, 2 ⅕, 2 ⅗, 3, 3 ⅖, 4, 4 ½, 5 ⅕, 6, 6 9⁄10, 8, 9 1⁄10, 10 ½, 12 1⁄10, 13 9⁄10, 16, 18 3⁄10, 21 1⁄10, 24 ⅕, 27 ⅘, 32, 36 7⁄10, 42 ⅕, 48 ½, 55 7⁄10, 64, 73 ½, 84 ⅖, 97, 111 ⅖, 128, 147, 168 ⅘, 194, 222 ⅘, 256, 294, 337 7⁄10, 388, 445 7⁄10, 512, 588 1⁄10, 675 ½, 776, 891 ⅖, 1024, 1176 ⅕, 1351 1⁄10, 1552, 1782 ⅘, 2048, 2352 ½, 2702 3⁄10, 3104 1⁄10, 3565 7⁄10, 4096, 4705, 5404 7⁄10, 6208 3⁄10…
///
/// If you want the traditional European typographic scale, you want [`iter::TypoScaleLcgIterator`].
pub trait TypoScale<T> where T: Default {
    /// converts a positive integer index to its equivalent value on the typographic scale.
    fn typoscale(&self) -> f64;
    /// converts a positive integer index to its equivalent value on the typographic scale.
    /// (rounded to nearest)
    fn int_typoscale(&self) -> usize {
        self.typoscale().round() as usize
    }
    /// converts a positive integer index to its equivalent value on the typographic scale.
    /// (floored)
    fn int_typoscale_floor(&self) -> usize {
        self.typoscale().floor() as usize
    }
    /// returns a string representation of the whole and fractional components of the typographic scale value for a positive integer
    // for 2 yields 1 3⁄10
    fn fraction_str(&self) -> String {
        let typoscale = self.typoscale();
        let int_typoscale = self.int_typoscale_floor() as usize;
        if typoscale.fract() < std::f64::EPSILON {
            return int_typoscale.to_string();
        }
        let fract = (typoscale.fract() * 10.0).floor() as usize;
        let mut s = String::new();
        s.push_str(&format!("{}", int_typoscale));
        if fract == 5 {
            s.push_str(NARROW);
            s.push_str(&format!("½"));
        } else {
            if fract % 2 == 0 {
                if fract != 0 {
                    s.push_str(NARROW);
                }
                s.push_str(FIFTHS[fract/2]);
            } else {
                s.push_str(NARROW);
                s.push_str(&format!("{}⁄10", fract));
            }
        }

        s
    }
}

impl<T> TypoScale<T> for T
where
    T: ToPrimitive + PrimInt + std::fmt::Display + Default,
{
    fn typoscale(&self) -> f64 {
        let i = self.to_f64().unwrap();
        
        1.0 * (2.0f64.powf(i / 5.0))
    }
}

