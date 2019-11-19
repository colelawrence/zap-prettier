#!/bin/bash
cargo build
for FILE in $(ls ./tests)
do
    echo "Testing $FILE"
    bash ./tests/$FILE | ./target/debug/zap-prettier
done
