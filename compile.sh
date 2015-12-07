#!/bin/bash
CWD=`pwd`
for dir in */; do 
    echo $dir;
    cd $CWD/$dir
    cargo build 
    cd ..
done
