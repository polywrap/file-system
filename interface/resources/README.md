# File-system Wrapper Interface

| Version | URI | WRAP Version |
|-|-|-|
| 1.0.0 | [`wrap://ens/wrappers.polywrap.eth:file-system@1.0.0`](https://wrappers.io/v/ens/wrappers.polywrap.eth:file-system@1.0.0) | 0.1 |

## Interface
```graphql
type Module {
    readFile(path: String!): Bytes!
    readFileAsString(path: String!, encoding: Encoding): String!

    exists(path: String!): Boolean!

    writeFile(path: String!, data: Bytes!): Boolean

    mkdir(path: String!, recursive: Boolean): Boolean

    rm(path: String!, recursive: Boolean, force: Boolean): Boolean
    rmdir(path: String!): Boolean
}

enum Encoding {
    ASCII
    UTF8
    UTF16LE
    UCS2
    BASE64
    BASE64URL
    LATIN1
    BINARY
    HEX
}
```

## Usage
```graphql
#import { Module } into FileSystem from "ens/wrappers.polywrap.eth:file-system@1.0.0"

type Module implements FileSystem_Module {}
```

And implement the interface methods within your programming language of choice.

## Source Code
[Link](https://github.com/polywrap/file-system)

## Known Implementations
[Link](https://github.com/polywrap/file-system/tree/master/implementations)