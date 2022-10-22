# tool_rand

## build

```sh
cargo build
cargo build --release
```

## Usage

- Help
  ```text
  $ rand -h
  Random generator

  Usage: rand --rt <RT> --len <LEN>

  Options:
  -r, --rt <RT>    Random type, "num", "alpha", "alphanum", "lower", "upper"
  -l, --len <LEN>  Random length
  -h, --help       Print help information
  -V, --version    Print version information
  ```
- Generate a random string, length is 10, include number.
  ```sh
  $ rand --rt "num" --len 10
  6075446545
  ```
- Generate a random string, length is 12, include lower/upper alphabet.
  ```sh
  $ rand --rt "alpha" --len 12
  pKARDgIYSPRX
  ```
- Generate a random string, length is 14, include number, lower/upper alphabet.
  ```sh
  $ rand --rt "alphanum" --len 14
  piW2Wxjy5AD3f7
  ```
- Generate a random string, length is 16, include lower alphabet.
  ```sh
  $ rand --rt "lower" --len 16
  mnwoihnhgctpecwo
  ```
- Generate a random string, length is 18, include upper alphabet.
  ```sh
  $ rand --rt "upper" --len 18
  PQLDYNUWBRKPPOOJRD
  ```

