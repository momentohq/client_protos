#!/bin/bash
set -e
set -x

PATH=node_modules/protoc-gen-ts/bin/:$PATH protoc -I=../proto -I=/usr/local/include --ts_out=dist cacheclient.proto controlclient.proto
