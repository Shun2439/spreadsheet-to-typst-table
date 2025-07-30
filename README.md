# spreadsheet-to-typst-table

## Usage

### Run program

``` shell
cargo run
```

### Inputs

1. Number of row
2. Table from spreadsheet

## feature

Just convert

``` txt
1	84.9
2	78.31
3	75.4
4	68.71
5	60.5
6	52.8
7	45.2
8	37.79
9	33.35
10	29.1
11	24.3
12	16.21
13	10.7
14	6.2
15	0.79
```

this one to

``` txt
[1], [84.9],
[2], [78.31],
[3], [75.4], 
[4], [68.71],
[5], [60.5],
[6], [52.8],
[7], [45.2],
[8], [37.79],
[9], [33.35],
[10], [29.1],
[11], [24.3],
[12], [16.21]
[13], [10.7],
[14], [6.2],
[15], [0.79],
```

on terminal.

## Requirements

- cargo(Rust Environment)

