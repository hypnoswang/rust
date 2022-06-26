#!/bin/bash

gcc -std=c11 -c -g *.c
ar rcs libcalled_by_rust.a *.o
rm -f *.o
