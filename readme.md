# Command line calculator
A wrapper application of https://github.com/ISibboI/evalexpr with some minor tweaks.

# Execution mode:
1. Parameter: 
```
> cal 1 + 2
3
```
2. Pipeline
```
> echo '1 + 2' | cal
3
```
3. Interactive
```
> cal
base: 10
input> 1 + 2
3
```

# Flags
```
  -B, --base <output_base>  Change outputs radix. Only affects integer results [default: 10]
  -x, --hex                 Short hand of --base 16
  -b, --bin                 Short hand of --base 2
  -o, --oct                 Short hand of --base 8
  -h, --help                Print help information
  -V, --version             Print version information
```

# Additional features
Application behaviors that are not provided by `evalexpr` crate.

1. Change output base with command line flags, or by `base = #` or `base(#)` command in the interactive mode.
2. Pre-processed alias: (case insensitive)
    1. `BIT#` -> `shl(1, #)`
    2. `#KB` -> `(# * 1024)`
    3. `#MB` -> `(# * 1024 * 1024)`
    4. `#GB` -> `(# * 1024 * 1024 * 1024)`
    5. `#TB` -> `(# * 1024 * 1024 * 1024 * 1024)`
    6. `#PB` -> `(# * 1024 * 1024 * 1024 * 1024 * 1024)`
3. Binary, octal, hexadecimal literal
    1. `0b###` or `###b` for binary
    2. `0###` for octal
    3. `0x###` or `###h` for hexadecimal
4. Fixed width bit operation functions
  Includes `not#(a)`, `or#(a, b)`, `and#(a, b)`, `xor(a, b)`. `#` can be 8, 16, 32, 64
5. `float` function to force floating number type.
6. `bits` and `bits_t` function to count set bits in the input. `bits` prints the result as string while `bits_t` will output `evalexpr` crates `Tuple` type.
7. Make `or`, `and`, `xor`, `not` aliases for `bitor`, `bitand`, `bitxor`, `bitnot`
8. A memory storage in the interactive mode that can retrieve the previous result by `$#` where `$1` is the most recent result, and `$2` is the second one... etc.
