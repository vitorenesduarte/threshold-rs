#!/usr/bin/env bash

###
### Example:
### $ quickcheck_loop.sh --test prop_tclock
###
### Another way is to use the QUICKCHECK_TESTS env variable:
### $ QUICKCHECK_TESTS=1000 cargo t --test prop_tclock

while true
do
    cargo test $@
    if [[ x$? != x0 ]] ; then
        exit $?
    fi
done
