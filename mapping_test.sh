cargo test
if cmp -s "mapping.txt" "mapping_ref.txt"; then
  echo "OK"
else
  echo "FAIL: indices don't match the reference"
  exit 1
fi
cargo test --release
if cmp -s "mapping.txt" "mapping_ref.txt"; then
  echo "OK"
else
  echo "FAIL: indices don't match the reference"
  exit 1
fi
exit 0
