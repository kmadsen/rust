# https://makefiletutorial.com/
#   For reference, can always improve these things.

# Adds support to call this make file from external directory
#   $ make -C markdown_parser/ wasm32-wasi.run 
project_path := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
makefile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
project_name := $(notdir $(patsubst %/,%,$(dir $(makefile_path))))

wasm_target_file := target/wasm32-wasi/debug/${project_name}.wasm

# Example set of commands to run a wasm file with wasmtime

wasm32-wasi.run: wasm32-wasi.build
	wasmtime --dir ${project_path} ${wasm_target_file} -- ${project_path}/README.md

wasm32-wasi.build: wasm32-wasi.target
	cargo build --target wasm32-wasi

wasm32-wasi.target:
	rustup target add wasm32-wasi

# Example perf report on linux.
# WARNING, when running wasmtime --jitdump and then injecting the jit files. It
#  creates a bunch of jit files in the project directory. I did not find a way
#  to specify these file destinations which is annoying. Also, I would have filed
#  a ticket on wasmtime, but would like to get further in my introduction first.

wasm32-wasi-perf.report: wasm32-wasi-perf.inject
	perf report --input ${project_path}/perf.jit.data

wasm32-wasi-perf.inject: wasm32-wasi-perf.record
	perf inject --jit --input ${project_path}/perf.data --output ${project_path}/perf.jit.data

wasm32-wasi-perf.record:
	perf record -k mono \
		wasmtime --dir ${project_path} --jitdump ${wasm_target_file} ${project_path}/README.md \
