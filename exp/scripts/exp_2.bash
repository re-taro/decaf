TSC_PATH="./node_modules/.bin/tsc"
DECAF_PATH="../target/release/decaf"
TMP_DIR="./tmp"

mkdir -p $TMP_DIR

cargo build --release
tsc_version=$($TSC_PATH --version)
decaf_version=$($DECAF_PATH info | head -n 1)

echo ""
echo "tsc version: $tsc_version"
echo "decaf version: $decaf_version"

cat ../checker/specification/specification.md ../checker/specification/staging.md > $TMP_DIR/simple.md
cargo run -p decaf-parser --example code_blocks_to_script $TMP_DIR/simple.md --comment-headers --out $TMP_DIR/simple.tsx

for level in middle complex very_complex; do
  case $level in
    middle)
      repeat=10 ;;
    complex)
      repeat=20 ;;
    very_complex)
      repeat=40 ;;
  esac

  yes $TMP_DIR/simple.md | head -n $repeat | tr '\n' ' ' | xargs cat > $TMP_DIR/$level.md
  cargo run -p decaf-parser --example code_blocks_to_script $TMP_DIR/$level.md --comment-headers --out $TMP_DIR/$level.tsx
done

for file in simple middle complex very_complex; do
  hyperfine --warmup 3 -i "$TSC_PATH --noEmit $TMP_DIR/$file.tsx" "$DECAF_PATH check $TMP_DIR/$file.tsx" || true
done
