#! /bin/bash

gcc -std=c11 -g -o caller main.c -lcalled_by_rust -L./
