import platform
import shutil

from bump import get_info, run_cmd

info = get_info()
name = info["name"]
version = info["version"]
print(f"{name}: {version}", flush=True)

run_cmd("pnpm tauri build")
if platform.system() == "Linux":
    run_cmd("makepkg --force --dir=release")
    shutil.copy(
        f"src-tauri/target/release/bundle/deb/{name}_{version}_amd64.deb", "release/"
    )
