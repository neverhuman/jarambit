#!/usr/bin/env python3
"""Verify saved Phase-8 hashes and repeat synthetic native/Python GCI checks."""
import sys
from phase8_synthetic_gci_check import main

if __name__ == "__main__":
    sys.argv.append("--verify")
    main()
