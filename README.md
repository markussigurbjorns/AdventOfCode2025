rustc --edition=2021 --emit=obj -C panic=abort -C opt-level=z -C strip="debuginfo" main.rs && cc  main.o -o main

Advent of code in Crust - ref: https://github.com/tsoding/Crust
