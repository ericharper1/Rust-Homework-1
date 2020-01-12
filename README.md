# Modexp

This is a command-line calculator that calculates an exponential modulo some number.

## Usage

```bash
cargo run 2 20 17
```

## What I did

I began by writing a hard-coded unit test, to ensure my test behaved properly.
I then wrote ```modexp()``` to perform the desired calculation. I used to pseudocode
provided to structure my algorithm for the function. I added error handling
for invalid input values. The main function was the last to be accounted for.
There, I added support to parse the command line arguments into an array
and used the values as arguments to ```modexp()```.

## How it went

Everything came together well. I made sure to code iteratively, by planning,
implementing, testing, and evaluating. I have to thank Rust's clear error
messages for helping me resolve my syntax issues.

## How I tested

I created and automated test to test a variety of values using ```assert_eq!()```,
hoping to cover any edge cases. I also manually tested against certain cases
from the command-line to ensure my program guards against invalid values, and 
values out-of-range.
