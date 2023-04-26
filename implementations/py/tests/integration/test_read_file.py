from pathlib import Path
from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, Uri
import pytest


async def test_valid_read_file(client: PolywrapClient, ascii_file_path: Path):
    result = await client.invoke(
        InvokerOptions(
            uri=Uri.from_str("plugin/fs"),
            method="readFile",
            args={"path": str(ascii_file_path)},
        )
    )
    assert result == b"Hello, world!"

async def test_invalid_read_file(client: PolywrapClient, temp_dir: Path):
    # Test with an invalid or non-existent file path
    with pytest.raises(Exception) as e:
        await client.invoke(
            InvokerOptions(
                uri=Uri.from_str("plugin/fs"),
                method="readFile",
                args={"path": str(temp_dir / "non_existent.txt")},
            )
        )

    assert isinstance(e.value.__cause__, FileNotFoundError)

