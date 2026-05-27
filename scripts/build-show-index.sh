#!/usr/bin/env bash
set -euo pipefail
BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SHOW_DIR="$BASE_DIR/show"
PARTIALS_DIR="$SHOW_DIR/partials"
OUTPUT_FILE="$SHOW_DIR/index.html"

if [ ! -d "$PARTIALS_DIR" ]; then
  echo "ERROR: partials directory not found: $PARTIALS_DIR" >&2
  exit 1
fi

if [ ! -d "$SHOW_DIR/pages" ]; then
  echo "ERROR: pages directory not found: $SHOW_DIR/pages" >&2
  exit 1
fi

printf "Building %s from page fragments...\n" "$OUTPUT_FILE"
{
  cat "$PARTIALS_DIR/head.html"
  for page in "$SHOW_DIR"/pages/*.html; do
    [ -e "$page" ] || continue
    cat "$page"
  done
  cat "$PARTIALS_DIR/footer.html"
} > "$OUTPUT_FILE"

printf "Built %s\n" "$OUTPUT_FILE"
