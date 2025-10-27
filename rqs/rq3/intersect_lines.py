#!/usr/bin/env python3
import sys
from pathlib import Path

def load_keys(path: Path) -> set[str]:
    """Read a file and return a set of stripped lines."""
    with path.open("r", encoding="utf-8") as f:
        return {line.strip() for line in f if line.strip()}

def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} file1 file2", file=sys.stderr)
        sys.exit(1)

    f1, f2 = map(Path, sys.argv[1:3])
    if not f1.exists() or not f2.exists():
        print("Error: one of the files does not exist", file=sys.stderr)
        sys.exit(1)

    set1 = load_keys(f1)
    set2 = load_keys(f2)

    intersect = sorted(set1 & set2)

    print(f"CoveredUnsafeLines: {len(intersect)}")
    print("Lines:")
    for key in intersect:
        print(key)

if __name__ == "__main__":
    main()
