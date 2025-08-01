set -e

cd $(git rev-parse --show-toplevel)
cargo build --target=x86_64-pc-windows-gnu --release
trash-put -f output
mkdir output
mkdir output/boops-and-hoops
cp target/x86_64-pc-windows-gnu/release/hoops-boops-and-loops.exe ./output/boops-and-hoops
cp -r assets/ ./output/boops-and-hoops
cd output
zip -r boops-and-hoops.zip boops-and-hoops/
cd ../
echo DONE, see output/hoops-and-boops.zip
