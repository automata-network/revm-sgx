#!/bin/bash -e
make 
cd bin
./server --json ../t/x.json
