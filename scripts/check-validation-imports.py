#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONSUMERS = ROOT / "validation-consumer"
files = [path for path in CONSUMERS.rglob("*") if path.is_file()]
text = "\n".join(path.read_text(errors="ignore") for path in files)

required = [
    "@evento-globolo/evgl-validation",
    "evgl-validation",
    "github.com/evento-globolo/evgl-lib-core/validation/golang",
    "evgl_validation",
]
for dependency in required:
    assert dependency in text, f"missing public lib-core import: {dependency}"

for forbidden in (
    "evgl-validation-server",
    "golang-server",
    "evgl_validation_server",
):
    assert forbidden not in text, f"client imported server-only validation package: {forbidden}"

print("all four client consumers import only public lib-core validation packages")
