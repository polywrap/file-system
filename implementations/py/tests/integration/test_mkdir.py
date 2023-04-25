from pathlib import Path
from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, Uri
import pytest

async def test_mkdir_new_dir_integration(client: PolywrapClient, temp_dir: Path):
    new_dir_path = temp_dir / "new_dir"
    await client.invoke(
        InvokerOptions(
            uri=Uri.from_str("plugin/fs"),
            method="mkdir",
            args={"path": str(new_dir_path)},
        )
    )

    assert new_dir_path.exists() and new_dir_path.is_dir()


async def test_mkdir_nested_dir_integration(client: PolywrapClient, temp_dir: Path):
    nested_dir_path = temp_dir / "dir1" / "dir2" / "dir3"
    await client.invoke(
        InvokerOptions(
            uri=Uri.from_str("plugin/fs"),
            method="mkdir",
            args={"path": str(nested_dir_path), "recursive": True},
        )
    )

    assert nested_dir_path.exists() and nested_dir_path.is_dir()


async def test_mkdir_existing_dir_integration(client: PolywrapClient, temp_dir: Path):
    existing_dir_path = temp_dir / "existing_dir"
    existing_dir_path.mkdir()

    with pytest.raises(Exception) as e:
        await client.invoke(
            InvokerOptions(
                uri=Uri.from_str("plugin/fs"),
                method="mkdir",
                args={"path": str(existing_dir_path)},
            )
        )

    assert isinstance(e.value.__cause__, FileExistsError)


async def test_mkdir_non_recursive_integration(client: PolywrapClient, temp_dir: Path):
    non_recursive_nested_dir_path = temp_dir / "dir1" / "dir2"

    with pytest.raises(Exception) as e:
        await client.invoke(
            InvokerOptions(
                uri=Uri.from_str("plugin/fs"),
                method="mkdir",
                args={"path": str(non_recursive_nested_dir_path), "recursive": False},
            )
        )

    assert isinstance(e.value.__cause__, FileNotFoundError)
    assert not non_recursive_nested_dir_path.exists()
