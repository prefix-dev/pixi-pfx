#!/usr/bin/env python3
"""Verify package versions stay synchronized across project manifests."""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def read_version(path: pathlib.Path, table: str) -> str:
    with path.open("rb") as handle:
        data = tomllib.load(handle)
    value = data[table]["version"]
    if not isinstance(value, str):
        raise TypeError(f"{path}: {table}.version is not a string")
    return value


def version_at_ref(ref: str) -> str:
    manifest = subprocess.run(
        ["git", "show", f"{ref}:Cargo.toml"],
        cwd=ROOT,
        check=True,
        capture_output=True,
    ).stdout
    data = tomllib.loads(manifest.decode())
    value = data["package"]["version"]
    if not isinstance(value, str):
        raise TypeError(f"{ref}:Cargo.toml: package.version is not a string")
    return value


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tag", help="also require this git tag (vX.Y.Z) to match")
    parser.add_argument(
        "--changed-from",
        metavar="GIT_REF",
        help="return status 3 unless package.version changed from this ref",
    )
    args = parser.parse_args()

    versions = {
        "Cargo.toml package.version": read_version(ROOT / "Cargo.toml", "package"),
        "pixi.toml package.version": read_version(ROOT / "pixi.toml", "package"),
    }
    if len(set(versions.values())) != 1:
        for source, version in versions.items():
            print(f"{source}: {version}", file=sys.stderr)
        print("package versions are not synchronized", file=sys.stderr)
        return 1

    version = next(iter(versions.values()))
    if args.tag and args.tag != f"v{version}":
        print(f"tag {args.tag!r} does not match package version v{version}", file=sys.stderr)
        return 1

    if args.changed_from:
        previous = version_at_ref(args.changed_from)
        if previous == version:
            print(
                f"package version v{version} did not change from {args.changed_from}",
                file=sys.stderr,
            )
            return 3

    print(version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
