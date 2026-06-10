#!/usr/bin/env bash

set -e

# Run the design, switch designs manually
fud2 matmul_bad.futil --to dat --through icarus -s sim.data=data.json
fud2 matmul_good.futil --to dat --through icarus -s sim.data=data.json