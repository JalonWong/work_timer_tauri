import argparse
import platform
import shutil

from bump import get_info, run_cmd

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

    run_cmd("pnpm install")
    run_cmd("pnpm tauri build")
    if platform.system() == "Linux":
        cmd = "makepkg --force --dir=release"
        if opts.install:
            cmd += " -i"
        run_cmd(cmd)
        shutil.copy(
            f"src-tauri/target/release/bundle/deb/{name}_{version}_amd64.deb",
            "release/",
        )
