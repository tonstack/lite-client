# TON lite_api

Implementation of [lite_api](https://github.com/ton-blockchain/ton/blob/master/tl/generate/scheme/lite_api.tl) and [lite-client](https://github.com/ton-blockchain/ton/tree/master/lite-client) in Rust using [adnl-rs](https://github.com/tonstack/adnl-rs).

| Feature           | Status                           |
| ----------------- | -------------------------------- |
| lite_api client   | ✅ Implemented                  |
| lite_api server   | ❌ Not implemented              |
| lite-client cli   | ✅ Implemented                  |
| async             | ❌ Not implemented              |

## Installation
```bash
cargo install --git https://github.com/tonstack/lite-client
```

## Usage
Without any options, [mainnet config](https://ton.org/global.config.json) will be used.
For testnet, use `-t / --testnet` flag.
To use your own config, pass `-c / --config <FILE>` option. 

Send an external message to TON:
```bash
echo 1234 | liteclient send -  # accept message bytes from stdin
liteclient send ./query.boc  # read from file
```
It prints:
```
[ERROR] Server error [code=0]: cannot apply external message to current state : failed to parse external message cannot deserialize bag-of-cells: invalid header, error 0
```

## Debug logging
```bash
echo 1234 | RUST_LOG=debug liteclient send -
```
prints:
```
[2022-03-15T10:43:55Z DEBUG liteclient::private] Sending query:
    Length: 20 (0x14) bytes
    0000:   df 06 8c 79  0c 82 d4 0a  69 05 31 32  33 34 0a 00   ...y....i.1234..
    0010:   00 00 00 00                                          ....
[2022-03-15T10:43:55Z DEBUG liteclient::private] Received:
    Length: 148 (0x94) bytes
    0000:   48 e1 a9 bb  00 00 00 00  8a 63 61 6e  6e 6f 74 20   H........cannot
    0010:   61 70 70 6c  79 20 65 78  74 65 72 6e  61 6c 20 6d   apply external m
    0020:   65 73 73 61  67 65 20 74  6f 20 63 75  72 72 65 6e   essage to curren
    0030:   74 20 73 74  61 74 65 20  3a 20 66 61  69 6c 65 64   t state : failed
    0040:   20 74 6f 20  70 61 72 73  65 20 65 78  74 65 72 6e    to parse extern
    0050:   61 6c 20 6d  65 73 73 61  67 65 20 63  61 6e 6e 6f   al message canno
    0060:   74 20 64 65  73 65 72 69  61 6c 69 7a  65 20 62 61   t deserialize ba
    0070:   67 2d 6f 66  2d 63 65 6c  6c 73 3a 20  69 6e 76 61   g-of-cells: inva
    0080:   6c 69 64 20  68 65 61 64  65 72 2c 20  65 72 72 6f   lid header, erro
    0090:   72 20 30 00                                          r 0.
[ERROR] Server error [code=0]: cannot apply external message to current state : failed to parse external message cannot deserialize bag-of-cells: invalid header, error 0
```
