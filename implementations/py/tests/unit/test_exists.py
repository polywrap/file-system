from typing import Any
from pathlib import Path


async def test_exists_valid_file(fs_plugin: Any, ascii_file_path: Path):
    result = await fs_plugin.exists({"path": str(ascii_file_path)}, None, None)
    assert result == True


async def test_exists_valid_dir(fs_plugin: Any, temp_dir: Path):
    result = await fs_plugin.exists({"path": str(temp_dir)}, None, None)
    assert result == True


async def test_exists_nonexistent_path(fs_plugin: Any):
    non_existent_path = Path("nonexistent_path")
    result = await fs_plugin.exists({"path": str(non_existent_path)}, None, None)
    assert result == False
