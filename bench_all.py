#!/bin/env python

import os
import subprocess

def main():
    os.environ['RUSTFLAGS'] = '-C target-cpu=native'
    subprocess.run(['cargo', 'build', '--release'])

    for day in range(1, 26):
        path = f'./target/release/day{day}'
        if os.path.isfile(path):
            print(f'Running {path}')
            subprocess.run([path, '--bench'])
            print()

if __name__ == '__main__':
    main()
