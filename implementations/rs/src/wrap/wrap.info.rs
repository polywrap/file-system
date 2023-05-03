/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "Fs".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "importedEnumTypes": [
    {
      "constants": [
        "ASCII",
        "UTF8",
        "UTF16LE",
        "UCS2",
        "BASE64",
        "BASE64URL",
        "LATIN1",
        "BINARY",
        "HEX"
      ],
      "kind": 520,
      "namespace": "FileSystem",
      "nativeType": "FileSystem_Encoding",
      "type": "FileSystem_FileSystem_Encoding",
      "uri": "wrap://ens/wraps.eth:file-system@1.0.0"
    }
  ],
  "importedModuleTypes": [
    {
      "isInterface": false,
      "kind": 256,
      "methods": [
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            }
          ],
          "kind": 64,
          "name": "readFile",
          "required": true,
          "return": {
            "kind": 34,
            "name": "readFile",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "readFile",
              "required": true,
              "type": "Bytes"
            },
            "type": "Bytes"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "enum": {
                "kind": 16384,
                "name": "encoding",
                "type": "FileSystem_FileSystem_Encoding"
              },
              "kind": 34,
              "name": "encoding",
              "type": "FileSystem_FileSystem_Encoding"
            }
          ],
          "kind": 64,
          "name": "readFileAsString",
          "required": true,
          "return": {
            "kind": 34,
            "name": "readFileAsString",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "readFileAsString",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            }
          ],
          "kind": 64,
          "name": "exists",
          "required": true,
          "return": {
            "kind": 34,
            "name": "exists",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "exists",
              "required": true,
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "data",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "data",
                "required": true,
                "type": "Bytes"
              },
              "type": "Bytes"
            }
          ],
          "kind": 64,
          "name": "writeFile",
          "required": true,
          "return": {
            "kind": 34,
            "name": "writeFile",
            "scalar": {
              "kind": 4,
              "name": "writeFile",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "recursive",
              "scalar": {
                "kind": 4,
                "name": "recursive",
                "type": "Boolean"
              },
              "type": "Boolean"
            }
          ],
          "kind": 64,
          "name": "mkdir",
          "required": true,
          "return": {
            "kind": 34,
            "name": "mkdir",
            "scalar": {
              "kind": 4,
              "name": "mkdir",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "recursive",
              "scalar": {
                "kind": 4,
                "name": "recursive",
                "type": "Boolean"
              },
              "type": "Boolean"
            },
            {
              "kind": 34,
              "name": "force",
              "scalar": {
                "kind": 4,
                "name": "force",
                "type": "Boolean"
              },
              "type": "Boolean"
            }
          ],
          "kind": 64,
          "name": "rm",
          "required": true,
          "return": {
            "kind": 34,
            "name": "rm",
            "scalar": {
              "kind": 4,
              "name": "rm",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        },
        {
          "arguments": [
            {
              "kind": 34,
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            }
          ],
          "kind": 64,
          "name": "rmdir",
          "required": true,
          "return": {
            "kind": 34,
            "name": "rmdir",
            "scalar": {
              "kind": 4,
              "name": "rmdir",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        }
      ],
      "namespace": "FileSystem",
      "nativeType": "Module",
      "type": "FileSystem_Module",
      "uri": "wrap://ens/wraps.eth:file-system@1.0.0"
    }
  ],
  "moduleType": {
    "imports": [
      {
        "type": "FileSystem_Module"
      },
      {
        "type": "FileSystem_FileSystem_Encoding"
      }
    ],
    "interfaces": [
      {
        "kind": 2048,
        "type": "FileSystem_Module"
      }
    ],
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "readFile",
        "required": true,
        "return": {
          "kind": 34,
          "name": "readFile",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "readFile",
            "required": true,
            "type": "Bytes"
          },
          "type": "Bytes"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "enum": {
              "kind": 16384,
              "name": "encoding",
              "type": "FileSystem_FileSystem_Encoding"
            },
            "kind": 34,
            "name": "encoding",
            "type": "FileSystem_FileSystem_Encoding"
          }
        ],
        "kind": 64,
        "name": "readFileAsString",
        "required": true,
        "return": {
          "kind": 34,
          "name": "readFileAsString",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "readFileAsString",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "exists",
        "required": true,
        "return": {
          "kind": 34,
          "name": "exists",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "exists",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "data",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "data",
              "required": true,
              "type": "Bytes"
            },
            "type": "Bytes"
          }
        ],
        "kind": 64,
        "name": "writeFile",
        "required": true,
        "return": {
          "kind": 34,
          "name": "writeFile",
          "scalar": {
            "kind": 4,
            "name": "writeFile",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "recursive",
            "scalar": {
              "kind": 4,
              "name": "recursive",
              "type": "Boolean"
            },
            "type": "Boolean"
          }
        ],
        "kind": 64,
        "name": "mkdir",
        "required": true,
        "return": {
          "kind": 34,
          "name": "mkdir",
          "scalar": {
            "kind": 4,
            "name": "mkdir",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "recursive",
            "scalar": {
              "kind": 4,
              "name": "recursive",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          {
            "kind": 34,
            "name": "force",
            "scalar": {
              "kind": 4,
              "name": "force",
              "type": "Boolean"
            },
            "type": "Boolean"
          }
        ],
        "kind": 64,
        "name": "rm",
        "required": true,
        "return": {
          "kind": 34,
          "name": "rm",
          "scalar": {
            "kind": 4,
            "name": "rm",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "rmdir",
        "required": true,
        "return": {
          "kind": 34,
          "name": "rmdir",
          "scalar": {
            "kind": 4,
            "name": "rmdir",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      }
    ],
    "type": "Module"
  },
  "version": "0.1"
})).unwrap()
  }
}
