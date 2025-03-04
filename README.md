# Owen's solutions for Advent of Code 2024

These are [Owen's](https://fosstodon.org/@omalley) Rust solutions for
[Advent of Code 2024](https://adventofcode.com/2024/). I use a
framework that I forked from
[here](https://gitlab.com/mbryant/aoc-2021/), but now is noticeably
different. In 2021, I started using Advent of Code to learn Rust and I
highly recommend that strategy. Learning new programming languages is
hard without using them to solve problems and Advent of Code provides
nice challenges where you won't have to support your code years from
now.

I prefer writing clean solutions that perform well. I'm far less concerned
with minimizing the time to an answer.

This year's current times on my laptop are:

| Time (secs) | Day |
| ----------- | --- |
|   0.047930 |    6 |
|   0.003660 |   11 |
|   0.000756 |   12 |
|   0.000746 |   14 |
|   0.000654 |   10 |
|   0.000470 |    5 |
|   0.000440 |    7 |
|   0.000363 |    4 |
|   0.000305 |    9 |
|   0.000228 |   13 |
|   0.000136 |    2 |
|   0.000120 |    1 |
|   0.000098 |    8 |
|   0.000065 |    3 |
  
The three targets that I use are:
* cargo run --release
* cargo test
* cargo bench

The run target will run all the defined days by default. If you
only want to run one day, give the day number as a cli parameter. By
default, the input comes from input/dayX.txt, unless you pass the -i
parameter with a directory to use instead.

Each day is put into a file src/dayX.rs and input/dayX.txt. You need
to update src/lib.rs to include it. Each day consists of three functions:

* generate(input: &str) -> ParsedType
* part1(input: &ParsedType) -> Display
* part2(input: &ParsedType) -> Display

The ParsedType may be different for each day. The output types must
implement Display so that it can be converted to a string, but they do
not need to be the same.

The framework will store the previous answer for each day's part 1 and
2 and will warn you if they change. That is really helpful when you
are optimizing after getting the right answer.

To run the benchmark, you need to set the day you want to benchmark in
benches/bench.rs.
