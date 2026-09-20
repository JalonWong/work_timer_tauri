import argparse
import os
import platform
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


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i",
        "--install",
        action="store_true",
        help="install PKGBUILD",
    )
    opts = parser.parse_args()

    info = get_info()

    name = info["name"]
    version = info["version"]
    print(f"{name}: {version}", flush=True)

    os.chdir("src-tauri")
    run_cmd("cargo build --release")
    os.chdir("..")
    run_cmd("pnpm tauri build")
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
        f"{name}*{version}*.msi",
        "src-tauri/target/release/bundle/msi/",
        "release/",
    )
    copy_file(
        f"{name}*{version}*.dmg",
        "src-tauri/target/release/bundle/dmg/",
        "release/",
    )
    copy_file(
        f"{name}*{version}*.rpm",
        "src-tauri/target/release/bundle/rpm/",
        "release/",
    )
