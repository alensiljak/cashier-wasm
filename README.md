# cashier-wasm
Cashier functionality in Wasm

# Introduction

This library contains functionality which is provided as a Wasm component and used in [Cashier](https://cashier.alensiljak.eu.org) PWA.

The purpose is code reusability and learning more about the practical side of the technology.

The functions are written in Rust.

# Deploy

Build the npm package with

```shell
wasm-pack build --target web
```

and include in Cashier by adding the line

```json
    "ledger-rs-lib": "file:D:\\src\\ledger-rs-lib\\pkg",
```
to package.json.
