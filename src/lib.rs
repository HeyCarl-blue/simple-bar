//! `simple_bar` is an extremely simple terminal progress bar
//! 
//! # Example
//! 
//! ```
//! use std::{thread::sleep, time::Duration};
//! use simple_bar::ProgressBar;
//! 
//! let num_iterations = 500;
//! let mut bar = ProgressBar::default(num_iterations);
//! 
//! for _ in 0..num_iterations {
//!     bar.next();
//!     sleep(Duration::from_millis(200));
//! }
//! ```
//! 
//! This example generates the following output:
//! 
//! ![above code generates](https://mie-res.netlify.app/simple_bar_example.png)

pub struct ProgressBar {
    length: usize,
    state: usize,
    progress_char: char,
    empty_char: char,
}

impl ProgressBar {
    ///Creates a new ProgressBar given:
    /// 1. `num_iterations: usize` the number of iterations
    /// 2. `progress_char: char` the `char` to be printed on the completed spots of the progress bar
    /// 3. `empty_char: char` the `char` to be printed on the empty spots of the progress bar
    pub fn new(length: usize, progress_char: char, empty_char: char) -> ProgressBar {
        ProgressBar { length, state: 0, progress_char, empty_char }
    }

    /// Creates a new ProgressBar with the default `char`s for the completed and empty spots of the
    /// progress bar, which are: `'▅'` and `' '` respectively.
    pub fn default(length: usize) -> ProgressBar {
        ProgressBar { length, state: 0, progress_char: '█', empty_char: ' ' }
    }

    /// Changes the `char`s for the completed and empty spots of the progress bar.
    pub fn reformat(&mut self, progress_char: char, empty_char: char) {
        self.progress_char = progress_char;
        self.empty_char = empty_char;
    }

    /// Updates the progress bar
    pub fn next(&mut self) {
        self.state += 1;

        let length_dig = (self.length as f64).log10() as i32;
        let state_dig = (self.state as f64).log10() as i32;

        let perc = ((self.state as f32 / self.length as f32) * 100.0) as i32;

        eprint!("\r");
        for _ in 0..(length_dig - state_dig) {
            eprint!("0");
        }
        eprint!("{} / {} [", self.state, self.length);
        for _ in 0..perc {
            eprint!("{}", self.progress_char);
        }
        for _ in perc..100 {
            eprint!(" ");
        }
        eprint!("] ({}%)", perc);
    }
}