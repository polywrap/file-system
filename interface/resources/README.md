# File-system Wrapper Interface

| Version | URI                                                                                               | WRAP Version |
|---------|---------------------------------------------------------------------------------------------------|-|
| 1.0.0   | [`wrap://ens/wraps.eth:file-system@1.0.0`](https://wrappers.io/v/ens/wraps.eth:file-system@1.0.0) | 0.1 |
| 2.0.0   | [`wrap://ens/wraps.eth:file-system@2.0.0`](https://wrappers.io/v/ens/wraps.eth:file-system@2.0.0) | 0.1 |


## Interface
```graphql
type Module {
    readFile(path: String!): Bytes!
    readFileAsString(path: String!, encoding: Encoding): String!

    exists(path: String!): Boolean!

    writeFile(path: String!, data: Bytes!): Boolean

    mkdir(path: String!, recursive: Boolean, mode: Int, mustCreate:  Boolean): Boolean

    rm(path: String!, recursive: Boolean, force: Boolean): Boolean
    rmdir(path: String!, recursive: Boolean): Boolean

    copyFile(source: String!, destination: String!): Boolean
    copyDir(source: String!, destination: String!): Boolean

    move(source: String!, destination: String!): Boolean

    realPath(path: String!): String!

    getMetadata(path: String!): Metadata!

    renameFile(oldPath: String!, newPath: String!): Boolean

    listDir(path: String!): [String!]!
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

type Metadata {
    name: String!
    creationTime: String!
    lastModifiedTime: String!
    permissions: Int!
    size: Int!
    isDirectory: Boolean!
    isFile: Boolean!
}
```

## Usage
```graphql
#import * from "ens/wraps.eth:file-system@2.0.0"
```

And implement the interface methods within your programming language of choice.

## Source Code
[Link](https://github.com/polywrap/std/file-system)

## Known Implementations
[Link](https://github.com/polywrap/file-system/tree/master/implementations)