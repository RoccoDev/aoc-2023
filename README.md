# Advent of Code 2023
These are my solutions for the 2023 [Advent of Code](https://adventofode.com/2023).  
Don't expect this to be maintained, I might miss some days or even stop prematurely.

## Challenge
The challenge is to run all days **on real inputs** in less than 1 second.  
I don't have as much time compared to the previous years, so I'm relaxing the rules a bit.

It is worth nothing that I upgraded my CPU (R5 3600 -> R7 5700X) this year, if the total 
time is too low I can try without `-C target-cpu=native`, for instance.

### Rules
* Only **stable** Rust
* No references to the input file, i.e. input is always loaded by copying it
* External libraries are limited to:
  + The `cargo-aoc` test and benchmarking harness
  + Faster hashers for integers and other common types (e.g. `fxhash`)
  + Enum sets and enum maps
  + `rayon` for parallel iterators (as somewhat of a last resort)
  + `itertools`
  + NO algorithm libraries, but own implementations from previous years are allowed
* Unsafe code as a last resort, with appropriate safety remarks
* Note: `-C target-cpu=native` and other **stable** compiler flags are allowed

## Running
Install `cargo-aoc`.  
```
cargo install cargo-aoc
```
Then, to run the solutions:  
```
cargo aoc 
# or cargo aoc -d <day number>
```

The utility also provides benchmarking, with:
```
cargo aoc bench
```
