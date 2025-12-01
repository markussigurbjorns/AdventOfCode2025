rustc --edition=2021 --emit=obj -C panic=abort main.rs
cc  main.o -o main
