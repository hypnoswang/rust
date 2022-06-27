#!/bin/bash

gcc -std=c11 -c -g *.c
ar rcs libhome.a *.o
rm -f *.o
