#!/bin/bash
COMMAND="build"
if [ "$1" == "clean" ]; then
    COMMAND="clean"
fi
CWD=`pwd`
for dir in */; do 
    echo $dir;
    cd $CWD/$dir
    cargo $COMMAND
    cd ..
done
