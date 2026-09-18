import argparse
import json
import os
import re
import subprocess
import sys
from io import SEEK_SET
from typing import Any


def run_cmd_list(cmd: list[str]) -> None:
    print("Run:", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True)


def run_cmd(cmd: str) -> None:
    run_cmd_list(cmd.split())


def get_info() -> dict[str, Any]:
    with open("package.json") as f:
        info: dict[str, Any] = json.load(f)
        version = info.get("version", "")
        if version == "":
            sys.exit(1)

    return info


def bump_version(mode: str) -> str:
    mode_map = {
        "major": 0,
        "minor": 1,
        "patch": 2,
    }
    index = mode_map[mode]
    version: str = get_info()["version"]
    v_list = version.split(".")
    vi = int(v_list[index])
    v_list[index] = str(vi + 1)
    for i in range(index + 1, 3):
        v_list[i] = "0"
    return ".".join(v_list)


def replace_content(
    file_name: str, pattern: str, new_string: str, count: int = 1
) -> None:
    print("Update:", file_name, flush=True)
    with open(file_name, "r+") as f:
        rst = re.sub(pattern, new_string, f.read(), count=count)
        f.seek(0, SEEK_SET)
        f.truncate(0)
        f.write(rst)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["major", "minor", "patch"])
    parser.add_argument(
        "--commit",
        default=True,
        action=argparse.BooleanOptionalAction,
        help="make a git commit (default: true)",
    )
    parser.add_argument(
        "--tag",
        default=True,
        action=argparse.BooleanOptionalAction,
        help="create a git tag (default: true)",
    )
    parser.add_argument(
        "-p",
        "--push",
        action="store_true",
        help="push to remote",
    )
    opts = parser.parse_args()

    print(opts)

    version = bump_version(opts.mode)
    print("version:", version)

    replace_content("src-tauri/Cargo.toml", r'version = ".+"', f'version = "{version}"')
    replace_content("package.json", r'"version": ".+"', f'"version": "{version}"')
    replace_content(
        "src-tauri/tauri.conf.json", r'"version": ".+"', f'"version": "{version}"'
    )
    replace_content("release/PKGBUILD", r"pkgver=.+", f"pkgver={version}")

    os.chdir("src-tauri")
    run_cmd("cargo fetch")
    os.chdir("..")

    if opts.commit:
        run_cmd("git add .")
        run_cmd_list(["git", "commit", "-m", f"chore: bump version to v{version}"])
        if opts.tag:
            run_cmd_list(["git", "tag", f"v{version}"])

        if opts.push:
            run_cmd("git push --follow-tags")
