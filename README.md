# jif

This crate aims to provide an interface that parses and decompresses GIF files. It exposes a simple API with 2
functions: one for parsing bytes into a compressed GIF data stream, and another for decompressing the stream into a list
of image frames.

This repository also contains a crate to render GIFs. It provides a command-line executable that
uses [`minifb`](https://github.com/emoon/rust_minifb) to render pixels.

`jif` aims to decode any GIF from [GIPHY](https://giphy.com). Any deviation from Giphy's rendering should be
considered a bug.

## Grammar

### Legend

The following legend defines the symbols used in this grammar:

| Symbol | Definition               |
|--------|--------------------------|
| `<>`   | grammar word             |
| `::=`  | defines symbol           |
| `*`    | zero or more occurrences |
| `+`    | one or more occurrences  |
| `\|`   | alternate element        |
| `[]`   | optional element         |

### The Grammar

`<GIF Data Stream> ::= Header <Logical Screen> <Data>* Trailer`

- `<Logical Screen> ::= Logical Screen Descriptor [Global Color Table]`
- `<Data> ::= <Graphic Block> | <Special-Purpose Block>`
    - `<Graphic Block> ::= [Graphic Control Extension] <Graphic-Rendering Block>`
        - `<Graphic-Rendering Block> ::= <Table-Based Image> | Plain Text Extension`
            - `<Table-Based Image> ::= Image Descriptor [Local Color Table] Image Data`
    - `<Special-Purpose Block> ::= Application Extension | Comment Extension`

## Reading

GIF - https://web.archive.org/web/20050216194905/http://www.danbbs.dk/~dino/whirlgif/gif89.html

LZW and GIF explained - https://web.archive.org/web/20050217131148/http://www.danbbs.dk/~dino/whirlgif/lzw.html

What's in a GIF - https://www.matthewflickinger.com/lab/whatsinagif/bits_and_bytes.asp

A breakdown of a GIF decoder - https://commandlinefanatic.com/cgi-bin/showarticle.cgi?article=art011

LZW Compression Wikipedia - https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch  