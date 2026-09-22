import argparse
import os
import platform
import re
import shutil
from pathlib import Path

from bump import get_info, run_cmd


def copy_file(name: str, src_dir: str, dist_dir: str) -> None:
    src = Path(src_dir)
    dist = Path(dist_dir)

    for s in list(src.glob(name)):
        d = dist.joinpath(os.path.basename(s))
        if d.exists():
            os.remove(d)
        print(f"Copy: {s} -> {d}", flush=True)
        shutil.copy(s, d)


def make_changelog() -> None:
    if platform.system() != "Linux":
        return

    print("Make changelog", flush=True)
    with open("CHANGELOG.md", "r") as f:
        text = f.read()
        m_list = list(re.finditer(r"# v.+\n", text))
        with open("release/DIFF_CHANGELOG.md", "w") as f:
            f.write(text[m_list[1].end() : m_list[2].start()])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i",
        "--install",
        action="store_true",
        help="install PKGBUILD",
    )
    parser.add_argument(
        "--amd64",
        action="store_true",
    )
    opts = parser.parse_args()
    macos_amd64 = False
    if opts.amd64 and platform.system() == "Darwin":
        macos_amd64 = True
        run_cmd("rustup target add x86_64-apple-darwin")

    info = get_info()

    name = info["name"]
    version = info["version"]
    print(f"{name}: {version}", flush=True)

    additional_arg = ""
    if macos_amd64:
        additional_arg = " --target x86_64-apple-darwin"

    os.chdir("src-tauri")
    run_cmd("cargo fetch")
    run_cmd("cargo check --release" + additional_arg)
    os.chdir("..")
    run_cmd("pnpm tauri build" + additional_arg)

    if "cachyos" in platform.release():
        cmd = "makepkg --force --dir=release"
        if opts.install:
            cmd += " -i"
        run_cmd(cmd)

    copy_file(
        f"{name}*{version}*.deb",
        "src-tauri/target/release/bundle/deb/",
        "release/",
    )
    copy_file(
        f"{name}*{version}*.rpm",
        "src-tauri/target/release/bundle/rpm/",
        "release/",
    )
    copy_file(
        f"{name}*{version}*.msi",
        "src-tauri/target/release/bundle/msi/",
        "release/",
    )
    if macos_amd64:
        copy_file(
            f"{name}*{version}*.dmg",
            "src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/",
            "release/",
        )
    else:
        copy_file(
            f"{name}*{version}*.dmg",
            "src-tauri/target/release/bundle/dmg/",
            "release/",
        )

    make_changelog()
