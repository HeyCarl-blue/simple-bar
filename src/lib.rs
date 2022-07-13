//! `simple_bar` is an extremely simple terminal progress bar
//! 
//! # Example
//! 
//! ```
//! use std::{thread::sleep, time::Duration};
//! use simple_bar::ProgressBar;
//! 
//! let num_iterations = 500;
//! let mut bar = ProgressBar::default(num_iterations, 50);
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

use std::time;
use std::io::{stdout, Write};

pub struct ProgressBar {
    num_iterations: u32,
    length: u32,
    state: u32,
    progress_char: char,
    last_progress_char: char,
    empty_char: char,
    eta: bool,
    last_percs: Cyclic<u32>,
    last_times: Cyclic<time::SystemTime>,
}

const LAST_PERC: usize = 10;

impl ProgressBar {
    /// Creates a new ProgressBar given:
    /// 1. `num_iterations: u32` the number of iterations
    /// 2. `progress_char: char` the `char` to be printed on the completed spots of the progress bar
    /// 3. `last_progress_char: char` the `char` to be printed as the last of the progess ones (e.g '>' to make '==>')
    /// 4. `empty_char: char` the `char` to be printed on the empty spots of the progress bar
    pub fn new(num_iterations: u32, length: u32, progress_char: char, last_progress_char: char, empty_char: char, eta: bool) -> ProgressBar {
        let last_percs = Cyclic::new(LAST_PERC);
        let last_times = Cyclic::new(LAST_PERC);
        ProgressBar { num_iterations, length, state: 0, progress_char, last_progress_char, empty_char, eta, last_percs, last_times }
    }

    /// Creates a new ProgressBar with the default `char`s for the completed and empty spots of the
    /// progress bar, which are: `'█'` and `' '` respectively.
    pub fn default(num_iterations: u32, length: u32) -> ProgressBar {
        ProgressBar::new(num_iterations, length, '█', '█', ' ', false)
    }

    /// Creates a new ProgressBar with the default `char`s for the completed and empty spots of the
    /// progress bar, which are: `'█'` and `' '` respectively, with E.T.A.
    pub fn default_eta(num_iterations: u32, length: u32) -> ProgressBar {
        ProgressBar::new(num_iterations, length, '█', '█', ' ', true)
    }

    ///Creates a new ProgressBar using the cargo style (e.g. [===>  ])
    pub fn cargo_style(num_iterations: u32, length: u32) -> ProgressBar {
        ProgressBar::new(num_iterations, length, '=', '>', ' ', false)
    }

    ///Creates a new ProgressBar using the cargo style (e.g. [===>  ]) with E.T.A.
    pub fn cargo_style_eta(num_iterations: u32, length: u32) -> ProgressBar {
        ProgressBar::new(num_iterations, length, '=', '>', ' ', true)
    }

    /// Changes the `char`s for the completed, last progress, and empty spots of the progress bar.
    pub fn reformat(&mut self, progress_char: char, last_progress_char: char, empty_char: char) {
        self.progress_char = progress_char;
        self.last_progress_char = last_progress_char;
        self.empty_char = empty_char;
    }

    pub fn reset(&mut self) {
        self.state = 0;
    }

    fn print_bar(&mut self, perc: u32, eta: f64){
        let length_dig = (self.num_iterations as f64).log10() as i32;
        let state_dig = (self.state as f64).log10() as i32;
        let ratio = remap(self.state as f32, 0., self.num_iterations as f32, 1., self.length as f32);

        let mut terminal_string = String::from("");

        print!("\r");

        for _ in 0..(length_dig - state_dig) {
            terminal_string = format!("{}{}", terminal_string, 0);
        }
        terminal_string = format!("{}{} / {} [", terminal_string, self.state, self.num_iterations);
        for _ in 0..ratio-1 {
            terminal_string = format!("{}{}", terminal_string, self.progress_char);
        }
        if self.is_last() {
            terminal_string = format!("{}{}", terminal_string, self.progress_char);
        } else {
            terminal_string = format!("{}{}", terminal_string, self.last_progress_char);
        }
        for _ in ratio..self.length {
            terminal_string = format!("{}{}", terminal_string, self.empty_char);
        }
        terminal_string = format!("{}] ({}%)", terminal_string, perc);

        if self.eta {
            let eta_hours = eta as i32 / 60 / 60;
            let eta_minutes = ( eta as i32 / 60 ) % 60 ;
            let eta_secs = eta as i32 % 60;

            let eta_h_dig = i32::max(0, f64::log10(eta_hours as f64) as i32);
            let eta_m_dig = i32::max(0, f64::log10(eta_minutes as f64) as i32);
            let eta_s_dig = i32::max(0, f64::log10(eta_secs as f64) as i32);

            if eta_h_dig > 2 {
                terminal_string = format!("{} ETA: ∞∞:∞∞:∞∞", terminal_string);
            } else {

                terminal_string = format!("{} ETA: ", terminal_string,);

                for _ in 0..1-eta_h_dig {
                    terminal_string = format!("{}0", terminal_string);
                }
                terminal_string = format!("{}{}:", terminal_string, eta_hours);
                for _ in 0..1-eta_m_dig {
                    terminal_string = format!("{}0", terminal_string);
                }
                terminal_string = format!("{}{}:", terminal_string, eta_minutes);
                for _ in 0..1-eta_s_dig {
                    terminal_string = format!("{}0", terminal_string);
                }
                terminal_string = format!("{}{}", terminal_string, eta_secs);
            }
        }
        
        print!("{}", terminal_string);

        if self.is_last() {
            println!("");
        }

        stdout().flush().unwrap();
    }

    /// Updates the progress bar
    pub fn next(&mut self) {
        assert!(self.state < self.num_iterations);
        self.state += 1;
        let perc = ((self.state as f32 / self.num_iterations as f32) * 100.) as u32;

        if self.eta {
            self.last_percs.add(perc);
            self.last_times.add(time::SystemTime::now());
            let last_length = self.last_percs.length();
            let y = if last_length >= 2 {
                let mut vm = 0.;
                for i in 0..last_length-1 {
                    let mut it_progress = ( self.last_percs.get(i+1) - self.last_percs.get(i) ) as f64;
                    let time_progress = self.last_times.get(i+1).duration_since(*self.last_times.get(i)).unwrap();
                    
                    let mut secs_progress = time_progress.as_secs_f64();

                    if secs_progress == 0. {
                        secs_progress = 0.01;
                    }

                    if it_progress == 0. {
                        it_progress = 0.01;
                    }

                    vm += it_progress / secs_progress;
                }
                vm *= 1. / last_length as f64;

                if vm == 0. {
                    vm = 0.1;
                } 
                
                ( 100. - *self.last_percs.last() as f64 ) / vm

            } else {
                f64::INFINITY
            };

            self.print_bar(perc, y);

        } else {
            self.print_bar(perc, 0.);
        }        
    }

    fn is_last(&self) -> bool {
        self.state == self.num_iterations
    }
}

struct Cyclic<T: std::clone::Clone> {
    items: Vec<T>,
    size: usize,
}

impl <T: std::clone::Clone> Cyclic<T> {
    pub fn new(size: usize) -> Cyclic<T> {
        let items = Vec::<T>::with_capacity(size);
        Cyclic { items, size }
    }

    pub fn add(&mut self, item: T) {
        let items_length = self.length();
        if items_length >= self.size {
            self.items.remove(0);
            self.items.push(item);
        } else {
            self.items.push(item);
        }
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, i: usize) -> &T {
        &self.items[i]
    }

    pub fn last(&mut self) -> &T {
        &self.items[self.length()-1]
    }

}

fn remap(val: f32, min: f32, max: f32, new_min: f32, new_max: f32) -> u32 {
    (new_min + (val - min) * (new_max - new_min) / (max - min)) as u32
}