# Ktnack
Stack-based interpreted language made in Rust. This project is made for me to learn Rust coming from C++.

# References
Inspired by [Porth](https://gitlab.com/tsoding/porth) by [Tsoding](https://www.youtube.com/@TsodingDaily).

# Build & Run
Uses only core cargo crates to my knowledge.<br>
Build using this command
```
cargo build
```
Then run the demo program using
```sh
# Windows
target\debug\ktanck.exe code.ktnck
# Linux/Posix
./target/debug/ktnack code.ktnck
```

You can also run it directly with the build like this
```
cargo run -- code.ktnck
```