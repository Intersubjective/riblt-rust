cargo test
if cmp "mapping.txt" "mapping_ref.txt"; then
  echo "OK"
else
  echo "FAIL: indices don't match the reference"
  exit 1
fi
cargo test --release
if cmp "mapping.txt" "mapping_ref.txt"; then
  echo "OK"
else
  echo "FAIL: indices don't match the reference"
  exit 1
fi
exit 0
