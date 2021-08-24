#!/bin/bash

#copy so
if [ -d "target/debug" ]; then
    cp dh/x64/* target/debug
fi
if [ -d "target/release" ]; then
    cp dh/x64/* target/release
fi
