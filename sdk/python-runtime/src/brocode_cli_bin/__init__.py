from __future__ import annotations

import os
from pathlib import Path

PACKAGE_NAME = "brocode-cli-bin"


def bundled_brocode_path() -> Path:
    exe = "brocode.exe" if os.name == "nt" else "brocode"
    path = Path(__file__).resolve().parent / "bin" / exe
    if not path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its packaged brocode binary at {path}"
        )
    return path


__all__ = ["PACKAGE_NAME", "bundled_brocode_path"]
