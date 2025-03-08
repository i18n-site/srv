DIR_HPC=$(dirname $(dirname $DIR))/hpc/coffee/rust2proto

mkdir -p gen
cd gen
rm -rf pb
cargo new --lib pb
cd ..

cargo build -p mod
