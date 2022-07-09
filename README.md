# Simple Bar

An extremely minimal terminal progress bar for Rust.

## Example

```Rust
use std::{thread::sleep, time::Duration};
use simple_bar::ProgressBar;

let num_iterations = 500;
let mut bar = ProgressBar::default(num_iterations);

for _ in 0..num_iterations {
    bar.next();
    sleep(Duration::from_millis(200));
}
```

This example generates the following output:
![above code generates](https://mie-res.netlify.app/simple_bar_example.png)