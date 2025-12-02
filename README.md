rustc --edition=2021 --emit=obj -C panic=abort -C opt-level=z -C strip="debuginfo" main.rs && cc  main.o -o main
