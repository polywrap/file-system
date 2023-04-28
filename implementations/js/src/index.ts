import {
  Module,
  manifest,
  Args_readFile,
  Args_readFileAsString,
  Args_exists,
  Args_writeFile,
  Args_mkdir,
  Args_rm,
  Args_rmdir,
} from "./wrap";
import fileSystemEncodingToBufferEncoding from "./utils/fileSystemEncodingToBufferEncoding";

import fs from "fs";
import { PluginFactory, PluginPackage } from "@polywrap/plugin-js";

type NoConfig = Record<string, never>;

export class FileSystemPlugin extends Module<NoConfig> {
  async readFile(args: Args_readFile): Promise<Uint8Array> {
    return fs.promises
      .readFile(args.path)
      .then((buffer) => new Uint8Array(buffer));
  }

  async readFileAsString(
    args: Args_readFileAsString
  ): Promise<string> {
    return fs.promises.readFile(args.path, {
      encoding: fileSystemEncodingToBufferEncoding(args.encoding),
    });
  }

  async exists(args: Args_exists): Promise<boolean> {
    return fs.existsSync(args.path);
  }

  async writeFile(
    args: Args_writeFile
  ): Promise<boolean | null> {
    await fs.promises.writeFile(args.path, Buffer.from(args.data));

    return true;
  }

  async mkdir(args: Args_mkdir): Promise<boolean | null> {
    await fs.promises.mkdir(args.path, {
      recursive: args.recursive ?? false,
    });

    return true;
  }

  async rm(args: Args_rm): Promise<boolean | null> {
    await fs.promises.rm(args.path, {
      recursive: args.recursive ?? false,
      force: args.force ?? false,
    });

    return true;
  }

  async rmdir(args: Args_rmdir): Promise<boolean | null> {
    await fs.promises.rmdir(args.path);

    return true;
  }
}

export const fileSystemPlugin: PluginFactory<NoConfig> = () =>
  new PluginPackage(new FileSystemPlugin({}), manifest);

export const plugin = fileSystemPlugin;
