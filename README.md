# repair

`repair` is a data structure encodes text with grammer compression.

## usage

Install Rust according to [this](https://doc.rust-lang.org/book/2018-edition/ch01-01-installation.html).

```
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/repair`
input text to indicate string attractors:
abracadabra
text with string attractors:
[a][b][r][a][c][a][d]abra

input text to indicate string attractors:
みるみるミルキィ
text with string attractors:
[み][る]みる[ミ][ル][キ][ィ]

input text to indicate string attractors:
...
```
