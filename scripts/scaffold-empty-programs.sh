#!/usr/bin/env bash
# Scaffold completely empty programs/ subfolders. Does not modify non-empty folders.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROGRAMS="$ROOT/programs"

cargo_name_from_dir() {
  local base
  base="$(basename "$1")"
  printf '%s' "${base# }"
}

display_name_from_cargo() {
  local name="$1"
  echo "$name" | sed 's/-/ /g' | sed 's/\b\(.\)/\u\1/g'
}

write_cargo_toml() {
  local dir="$1" name="$2"
  cat >"$dir/Cargo.toml" <<EOF
[package]
name = "$name"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = { version = "=1.5.0", features = ["renderer-femtovg"] }

[build-dependencies]
slint-build = "=1.5.0"
EOF
}

write_build_rs() {
  local dir="$1"
  cat >"$dir/build.rs" <<'EOF'
fn main() {
    slint_build::compile("ui/app.slint").unwrap();
}
EOF
}

write_main_rs() {
  local dir="$1"
  cat >"$dir/src/main.rs" <<'EOF'
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    app.run()
}
EOF
}

write_app_slint() {
  local dir="$1" title="$2"
  cat >"$dir/ui/app.slint" <<EOF
export component AppWindow inherits Window {
    title: "$title";
    width: 640px;
    height: 480px;
    background: #0b0f19;

    VerticalLayout {
        alignment: center;
        spacing: 12px;
        padding: 24px;

        Text {
            text: "$title";
            font-size: 24px;
            font-weight: 700;
            color: #e2e8f0;
            horizontal-alignment: center;
        }

        Text {
            text: "Sumer OS — ready for development";
            font-size: 14px;
            color: #94a3b8;
            horizontal-alignment: center;
        }
    }
}
EOF
}

write_build_sh() {
  local dir="$1" name="$2" label="$3"
  cat >"$dir/build.sh" <<EOF
#!/bin/sh
set -e

cd "\$(dirname "\$0")"

echo "==========================================="
echo "  Compiling $label for Alpine...           "
echo "==========================================="

docker run --rm \\
  -v "\$PWD:/volume" \\
  -v "\$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \\
  -v "\$PWD/../../.cache/cargo_cache:/root/.cargo/git" \\
  -w /volume \\
  rust:alpine \\
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

rm -f $name
cp target-alpine/release/$name .

echo "[+] $label compiled successfully!"
EOF
  chmod +x "$dir/build.sh"
}

write_preview_sh() {
  local dir="$1" label="$2"
  cat >"$dir/preview.sh" <<EOF
#!/bin/bash
cd "\$(dirname "\$0")"

echo "==========================================="
echo "   $label Live UI Previewer"
echo "==========================================="
echo "[+] Starting live UI preview..."
echo "[*] Edits to ui/app.slint reload automatically."

../../programs/slint-viewer/slint-viewer --auto-reload ui/app.slint
EOF
  chmod +x "$dir/preview.sh"
}

write_run_sh() {
  local dir="$1"
  cat >"$dir/run.sh" <<'EOF'
#!/bin/bash
cd "$(dirname "$0")"
cargo run
EOF
  chmod +x "$dir/run.sh"
}

scaffold_one() {
  local dir="$1"
  local name
  name="$(cargo_name_from_dir "$dir")"
  local label
  label="$(display_name_from_cargo "$name")"

  mkdir -p "$dir/src" "$dir/ui"
  write_cargo_toml "$dir" "$name"
  write_build_rs "$dir"
  write_main_rs "$dir"
  write_app_slint "$dir" "$label"
  write_build_sh "$dir" "$name" "$label"
  write_preview_sh "$dir" "$label"
  write_run_sh "$dir"

  template_lock="$PROGRAMS/zaqura-store/Cargo.lock"
  if [ -f "$template_lock" ]; then
    sed "s/zaqura-store/$name/g" "$template_lock" >"$dir/Cargo.lock"
  else
    (cd "$dir" && cargo generate-lockfile) || true
  fi

  echo "  scaffolded: $name"
}

count=0
for dir in "$PROGRAMS"/*/; do
  [ -d "$dir" ] || continue
  if [ "$(find "$dir" -mindepth 1 -print -quit 2>/dev/null | wc -l)" -eq 0 ]; then
    scaffold_one "$dir"
    count=$((count + 1))
  fi
done

echo "[+] Scaffolded $count empty program folder(s)."
