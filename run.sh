#!/usr/bin/env bash

set -e

# Run the design, switch designs manually
fud2 matmul_bad.futil --to dat --through icarus -s sim.data=data.json
fud2 matmul_good.futil --to dat --through icarus -s sim.data=data.json

# Added in pass to have the bad design switch to repeat
fud2 matmul_bad.futil --to dat --through icarus -s sim.data=data.json -s "calyx.args=-p validate -p while-to-repeat -p pre-opt -p compile -p lower"