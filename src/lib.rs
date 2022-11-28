#[allow(unused_macros)]
macro_rules! example {
    ($($values:expr) +) => {
        &stringify!($($values)*).replace(" ", "\n")
    };
}

pub mod days;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2022 }
