#!/usr/bin/env bash

rm -rf src/*.proto

cd POGOProtos

rm -rf tmp

python2 compile_single.py -l rust -o ../src

cd ..

# for f in src/POGOProtos_*.rs; do
#     nf=$(echo $f | sed "s/POGOProtos_//g")
#     mv $f $nf
# done

rm -rf src/lib.rs

echo "extern crate protobuf;" > src/lib.rs
echo "" >> src/lib.rs

find src  -name '*.rs' | grep POGOProtos_ | sed 's_src/__g' | sed 's/.rs//g' | awk '{print "pub mod "$1";"}' | sort >> src/lib.rs

# find src  -name '*.rs' | sed 's_src/__g' | sed 's/.rs//g' | awk '{print "pub use "$1";"}' | sort >> src/lib.rs
