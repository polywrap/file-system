from typing import Any, cast
from polywrap_core import InvokerClient
import pytest
from pathlib import Path
from polywrap_fs_plugin import FileSystemPlugin


async def test_valid_read_file(fs_plugin: FileSystemPlugin, ascii_file_path: Path):
    result = await fs_plugin.read_file(
        {"path": str(ascii_file_path)}, cast(InvokerClient[Any], None), None
    )
    assert result == b"Hello, world!"


async def test_invalid_read_file(fs_plugin: FileSystemPlugin, temp_dir: Path):
    # Test with an invalid or non-existent file path
    with pytest.raises(FileNotFoundError):
        await fs_plugin.read_file(
            {"path": str(temp_dir / "non_existent.txt")},
            cast(InvokerClient[Any], None),
            None,
        )
