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

use std::io::{stdout, Write};

pub struct ProgressBar {
    num_iterations: usize,
    state: usize,
    progress_char: char,
    last_progress_char: char,
    empty_char: char,
}

impl ProgressBar {
    /// Creates a new ProgressBar given:
    /// 1. `num_iterations: usize` the number of iterations
    /// 2. `progress_char: char` the `char` to be printed on the completed spots of the progress bar
    /// 3. `last_progress_char: char` the `char` to be printed as the last of the progess ones (e.g '>' to make '==>')
    /// 4. `empty_char: char` the `char` to be printed on the empty spots of the progress bar
    pub fn new(num_iterations: usize, progress_char: char, last_progress_char: char, empty_char: char) -> ProgressBar {
        ProgressBar { num_iterations, state: 0, progress_char, last_progress_char, empty_char }
    }

    /// Creates a new ProgressBar with the default `char`s for the completed and empty spots of the
    /// progress bar, which are: `'█'` and `' '` respectively.
    pub fn default(num_iterations: usize) -> ProgressBar {
        ProgressBar { num_iterations, state: 0, progress_char: '█', last_progress_char: '█',  empty_char: ' ' }
    }

    /// Changes the `char`s for the completed, last progress, and empty spots of the progress bar.
    pub fn reformat(&mut self, progress_char: char, last_progress_char: char, empty_char: char) {
        self.progress_char = progress_char;
        self.last_progress_char = last_progress_char;
        self.empty_char = empty_char;
    }

    ///Creates a new ProgressBar using the cargo style (e.g. [===>  ])
    pub fn cargo_format(num_iterations: usize) -> ProgressBar {
        ProgressBar { num_iterations, state: 0, progress_char: '=', last_progress_char: '>', empty_char: ' ' }
    }

    /// Updates the progress bar
    pub fn next(&mut self) {
        self.state += 1;

        let length_dig = (self.num_iterations as f64).log10() as i32;
        let state_dig = (self.state as f64).log10() as i32;

        let perc = ((self.state as f32 / self.num_iterations as f32) * 100.0) as i32;

        eprint!("\r");
        for _ in 0..(length_dig - state_dig) {
            eprint!("0");
        }
        eprint!("{} / {} [", self.state, self.num_iterations);
        for _ in 0..perc-1 {
            eprint!("{}", self.progress_char);
        }
        if perc == 100 {
            eprint!("{}", self.progress_char);
        } else {
            eprint!("{}", self.last_progress_char);
        }
        for _ in perc..100 {
            eprint!(" ");
        }
        eprint!("] ({}%)", perc);

        stdout().flush().unwrap();
    }
}