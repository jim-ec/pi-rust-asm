#!/bin/sh

mkdir -p build

clang -o pi.o pi.S -c
ar rcs libpi.a pi.o
rustc -o main main.rs -l pi -L .
