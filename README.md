# TON LiteClient

An attempt to rewrite [lite-client](https://github.com/ton-blockchain/ton/tree/master/lite-client) for TON Blockchain in Rust using [ton-labs-adnl library](https://github.com/tonlabs/ton-labs-adnl).

## Installation
```bash
# workdir = cloned repo folder
cargo install --path .
```

## Usage
Create file `config.json` with liteserver ip, port and public key:
```json
{  
  "server_address": "65.21.74.140:46427",  
  "server_key": {  
    "type_id": 1209251014,  
    "pub_key": "JhXt7H1dZTgxQTIyGiYV4f9VUARuDxFl/1kVBjLSMB8="  
  }  
}
```
(that is a testnet server from https://newton-blockchain.github.io/testnet-global.config.json)

Run liteclient:
```bash
liteclient -c ./config.json
```
It prints:
```
result = LiteServer_CurrentTime(CurrentTime { now: 1647197653 })
```
Note that for now it simply executes `liteServer.getTime` command and prints raw result.
## Debug logging
```bash
RUST_LOG=debug liteclient
```
prints:
```
[2022-03-13T18:55:19Z DEBUG liteclient::private] Sending query 00000000 60 78 d0 e3 8d 55 00 00 e0 9b 06 e3 8d 55 00 00
[2022-03-13T18:55:19Z DEBUG liteclient::private] Received 00000000      a0 75 d0 e3 8d 55 00 00 80 72 08 e3 8d 55 00 00
result = LiteServer_CurrentTime(CurrentTime { now: 1647197720 })
```