#! /bin/bash

gcc -std=c11 -g -o call_rust main.c -lfamily -L../../family/target/debug/ -I../../family/
