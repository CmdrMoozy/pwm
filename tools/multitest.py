#!/usr/bin/env python3
#
# This script executes "cargo test" for each combination of features this crate
# supports. This is used to find bugs which only present for certain feature
# combinations.

import itertools
import os
import subprocess
import sys

_DIRECTORY = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))

# TODO: Read this from Cargol.toml instead of hard-coding it.
_FEATURES = ['clipboard', 'piv', 'wifiqr']

for r in range(1, len(_FEATURES) + 1):
    for combo in itertools.combinations(_FEATURES, r):
        print('Testing with features [{}]'.format(', '.join(combo)))
        result = subprocess.run(['cargo', 'test', '--no-default-features', '--features', ','.join(combo)], cwd=_DIRECTORY, stdout=subprocess.PIPE,  stderr=subprocess.STDOUT, text=True)
        if result.returncode != 0:
            print(result.stdout)
            sys.exit(result.returncode)

sys.exit(0)
