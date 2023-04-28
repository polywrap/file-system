# @polywrap/file-system-plugin-js

The Filesystem plugin enables wraps running within the Polywrap client to interact with the local filesystem.

## Interface

The FileSystem plugin implements an existing wrap interface at `wrap://ens/wraps.eth:file-system@1.0.0`.

## Usage

``` typescript
import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";
import { filesystemPlugin } from "@polywrap/file-system-plugin-js";

const config = new ClientConfigBuilder()
  .addPackage(
    "wrap://ens/wraps.eth:file-system@1.0.0",
    fileSystemPlugin({ })
  )
  .build();

client.invoke<Uint8Array>({
  uri: "wrap://ens/wraps.eth:file-system@1.0.0",
  method: "readFile",
  args: {
    path: "./file.txt"
  }
});
```

For more usage examples see `src/__tests__`.
