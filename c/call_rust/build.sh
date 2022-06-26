#! /bin/bash

gcc -std=c11 -g -o call_rust main.c -l:libfamily.a -L../../family-ffi/target/debug/ -I../../family-ffi/
