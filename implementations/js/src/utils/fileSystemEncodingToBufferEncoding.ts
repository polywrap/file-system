import { Encoding, EncodingEnum } from "../wrap";

const fileSystemEncodingToBufferEncoding = (
  encoding: Encoding | null | undefined
): BufferEncoding => {
  switch (encoding) {
    case EncodingEnum.ASCII:
    case "ASCII":
      return "ascii";

    case EncodingEnum.BASE64:
    case "BASE64":
      return "base64";

    case EncodingEnum.BASE64URL:
    case "BASE64URL":
      return "base64url";

    case EncodingEnum.BINARY:
    case "BINARY":
      return "binary";

    case EncodingEnum.HEX:
    case "HEX":
      return "hex";

    case EncodingEnum.LATIN1:
    case "LATIN1":
      return "latin1";

    case EncodingEnum.UCS2:
    case "UCS2":
      return "ucs2";

    case EncodingEnum.UTF16LE:
    case "UTF16LE":
      return "utf16le";

    case EncodingEnum.UTF8:
    case "UTF8":
      return "utf8";

    default:
      return "utf8";
  }
};

export default fileSystemEncodingToBufferEncoding;
