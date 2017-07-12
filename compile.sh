#!/bin/bash
COMMAND="build"
if [ "$1" == "clean" ]; then
    COMMAND="clean"
fi
if [ "$1" == "test" ]; then
    COMMAND="test"
fi
if [ "$1" == "update" ]; then
    COMMAND="update"
fi
if [ "$1" == "check" ]; then
    COMMAND="check"
fi
CWD=`pwd`
RESULT=""
for dir in */; do
    echo $dir;
    cd $CWD/$dir
    cargo $COMMAND
    rc=$?
    if [[ $rc != 0 ]]; then RESULT="$RESULT $dir"; fi
    cd ..
done
if [ "$RESULT" != "" ]; then
    echo "Failed projects are"
    echo $RESULT
fi
