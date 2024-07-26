[![Rust](https://github.com/PaddyThePaddy/cal/actions/workflows/rust.yml/badge.svg)](https://github.com/PaddyThePaddy/cal/actions/workflows/rust.yml)  
A cli calculator for myself (mostly firmware programming scenario)

# Example
`cal "2+(1-5)*3/4"`  
Outputs -1  

`cal "~(0xF<<4)" -x`  
Outputs FFFFFF0F  

`cal "0b101 b^ b!0" -b`  
Outputs 11111010  

`cal "ascii(0x5f465648)"`  
Outputs _FVH  

# Input format
Number literals in the expression accepts following format:
- Decimal numbers
- Hexadecimal numbers with leading `0x`
- Octal numbers with leading `0o`
- Binary number with leading `0b`
- Floating point numbers in scientific notation

# Output format
- `--hex` / `-x`: Hexadecimal
- `--oct` / `-o`: Octal
- `--bin` / `-b`: Binary
- `--bits`: List of set bits

# Operators
| Operator                   | Meaning          |
| -------------------------- | ---------------- |
| +, -, *, /                 | As normal        |
| ^^                         | Exponent / power |
| %                          | Mod              |
| &                          | Bit-wise AND     |
| \|                         | Bit-wise OR      |
| [bit width]^               | Bit-wise XOR     |
| [bit width]!, [bit width]~ | Bit-wise NOT     |
| <<, >>                     | Bit-wise shift   |
| FunctionName()             | Function call    |

## Bit width indicator
| Bit width indicator | bits count |
| ------------------- | ---------- |
| b                   | 8          |
| w                   | 16         |
| dw (default)        | 32         |
| l                   | 64         |
| ll                  | 128        |

## Default Functions
| Name  | Usage                                                                                       |
| ----- | ------------------------------------------------------------------------------------------- |
| ascii | Convert string to an integer consist of ascii code of the string characters. Or vise versa. |
| rev   | Reverse bytes in an integer or characters in an string                                      |

# Multiple shortcuts
Adding character k, m, g, t, p after a decimal integer multiply its value.

| Shortcut | Multiple                |
| -------- | ----------------------- |
| k        | 1024                    |
| m        | 1024k, 1048576          |
| g        | 1024m, 1073741824       |
| t        | 1024g, 1099511627776    |
| p        | 1024t, 1125899906842624 |

# Bit shortcuts
`bit#` converts into `1 << #`, like `bit3` equals `8`
