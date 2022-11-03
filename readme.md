# Command line calculator
A wrapper application of https://github.com/ISibboI/evalexpr with some minor tweaks.  
The evalexpr used in this repository is [forked](https://github.com/PaddyThePaddy/evalexpr) to support bit operators and maximum data type(i128)

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
  cal> 1 + 2
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
Application behaviors that are not provided by the original `evalexpr` crate.

1. Change output radix base with command line flags, or by `_base  #` or `_b #` command in the interactive mode.
2. Pre-processed alias: (case insensitive)
    1. `BIT#` -> `shl(1, #)`
    2. `#KB` -> `(# * 1024)`
    3. `#MB` -> `(# * 1024 * 1024)`
    4. `#GB` -> `(# * 1024 * 1024 * 1024)`
    5. `#TB` -> `(# * 1024 * 1024 * 1024 * 1024)`
    6. `#PB` -> `(# * 1024 * 1024 * 1024 * 1024 * 1024)`
3. Binary, octal, hexadecimal literal
    1. `0b###` or `###b` for binary number
    2. `0o###` or `###o` for octal number
    3. `0x###` or `###h` for hexadecimal number
4. Fixed width bit operation functions
  Includes `not#(a)`, `or#(a, b)`, `and#(a, b)`, `xor(a, b)`. `#` can be 8, 16, 32, 64
5. `float` function to force floating number type.
6. `bits` and `bits_t` function to count set bits in the input. `bits` prints the result as a string while `bits_t` will output `evalexpr` crates `Tuple` type.
7. Make `or`, `and`, `xor`, `not` aliases for `bitor`, `bitand`, `bitxor`, `bitnot`.
8. A memory storage in the interactive mode that can retrieve the previous result by `$#` where `$#` is the #th result, `$-#` is the #th least result.
    Added pre-processed commands `_memlen` and `_memval` to inspect memory storage.
9. BitAnd ("&"), BitOr ("|"), BitXor ("^^") operator.
10. ASCII signature processing functions:
    1.  `ascii` to convert:
        1.  A single uint signature into string
        2.  Uint array into string
        3.  String into uint array
    2.  `com` or `mer` to merge uint array into a single uint by big endian
    3.  `rev` to reverse string, array or bytes in a single uint
    4.  `bytes` to split bytes in a uint

    Examples:
    1. `ascii 0x41424344` -> `"ABCD"`
    2. `acii 'ABCD'` -> `[Int(65), Int(66), Int(67), Int(68)]`
    3. `com ascii 'ABCD'` -> `41424344` in base 16 mode
    4. `ascii rev 0x41424344` -> `"DCBA"`
    5. `bytes 0x41424344` -> `[Int(65), Int(66), Int(67), Int(68)]`