#!/usr/bin/env bash

###
### Example:
### $ quickcheck_loop.sh --test prop_tclock
###

while true
do
    cargo test $@
    if [[ x$? != x0 ]] ; then
        exit $?
    fi
done
