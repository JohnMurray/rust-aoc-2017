#!/bin/bash

rand() {
    FLOOR=$1
    CEIL=$2
    number=0
    while [ "$number" -le $FLOOR ] ; do
        number=$RANDOM
        let "number %= $CEIL"
    done
    echo -n $number;
}

rows=$(rand 500 1000)
echo "let input_data = vec!("
for x in $(seq 1 $rows); do
    echo -n "    vec!("
    columns=$(rand 20 500)
    for c in $(seq 1 $columns); do
        echo -n "$(rand 1 10000), "
    done
    echo "$(rand 1 10000)),"
done
echo ");"