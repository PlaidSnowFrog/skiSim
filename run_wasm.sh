#!/bin/sh
# Cargo runner for wasm32-unknown-unknown: bindgen the .wasm and serve it locally.
# Uses the installed wasm-bindgen CLI, which must match the wasm-bindgen version in Cargo.lock.
set -e

WASM="$1"
OUT="target/wasm-web"
PORT="${PORT:-1334}"

rm -rf "$OUT"
mkdir -p "$OUT"
wasm-bindgen --target web --no-typescript --out-dir "$OUT" --out-name skiSim "$WASM"
[ -d assets ] && cp -R assets "$OUT/assets"

cat > "$OUT/index.html" <<'EOF'
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>skiSim</title>
  <style>html, body { margin: 0; height: 100%; background: #000; } canvas { display: block; }</style>
</head>
<body>
  <script type="module">
    import init from "./skiSim.js";
    init();
  </script>
</body>
</html>
EOF

echo "Serving on http://127.0.0.1:$PORT"
exec python3 -m http.server "$PORT" --bind 127.0.0.1 -d "$OUT"
