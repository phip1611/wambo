# wambo - All-in-one binary that converts decimal/bin/hex + interprets input(bytes) as several (un)signed data types

**wambo** is a binary that can easily shows you a numeric value in all important
numeral systems (bin, hex, dec) + interprets the input as both signed 
and unsigned values (from i8 to i64, including f32 and f64). It also
easily calculates you mibibytes to bytes, kilobytes to gibibytes and so on.

### Usage
**Install:** `$ cargo install wambo`
##### Examples
- `$ wambo 1mb`
- `$ wambo 0xdeadbeef`
- `$ wambo 0b10001111_00000000`
- `$ wambo 0xf_gb` (15 gigabyte)
- `$ wambo -h` (for more help and examples)
Just input **an unsigned number** (maximum 64bit) and **wambo** calculates
all values that are interesting to developers. Example output: \
"-7" is represented by 0xc0e00000 in floating point standard (IEEE-754). \
So `$ wambo 0xc0e00000` results in *(you can find the -7 in the f32 line)*:

```
### interpreting as: different numeral systems (unsigned) ###
decimal: 3235905536
hex    : 0x00_00_00_00_c0_e0_00_00 (64bit)
bin    : 0b00000000_00000000_00000000_00000000_11000000_11100000_00000000_00000000 (64bit)

### interpreting as: several (un)signed data types (decimal) ###
 u8:          0
 i8:          0
u16:          0
i16:          0
i32: -1059061760
u64: 3235905536
i64: 3235905536
f32:         -7.0000000 (bits interpreted as IEEE-754)
f64:          0.0000000 (bits interpreted as IEEE-754)

### interpreting as: bytes/size ###
 B     :    3235905536
KB     :       3235905.5360000
MB     :          3235.9055360
GB     :             3.2359055

### interpreting as: *ibi-bytes (1024 (=multiple of 2) as base instead of 1000) ###
KiB    :       3160064.0000000
MiB    :          3086.0000000
GiB    :             3.0136719
```