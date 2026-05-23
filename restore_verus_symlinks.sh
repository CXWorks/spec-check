#!/bin/bash
# Run this after restarting Docker container to restore vstd runtime symlinks.
# Without these, Verus will fail with "can't find crate for vstd".
#
# Usage:
#   bash restore_verus_symlinks.sh [CONTAINER_ID]
# Default container: baf374735845 (hardcore_hawking)

CONTAINER="${1:-baf374735845}"
VERUS_RELEASE="/opt/verus/source/target-verus/release"
VERUS_ROOT="/opt/verus"

echo "Restoring Verus vstd symlinks in container: $CONTAINER"

docker exec "$CONTAINER" ln -sf "$VERUS_RELEASE/libbuiltin.rlib"              "$VERUS_ROOT/libbuiltin.rlib"
docker exec "$CONTAINER" ln -sf "$VERUS_RELEASE/libbuiltin_macros.so"         "$VERUS_ROOT/libbuiltin_macros.so"
docker exec "$CONTAINER" ln -sf "$VERUS_RELEASE/libstate_machines_macros.so"  "$VERUS_ROOT/libstate_machines_macros.so"
docker exec "$CONTAINER" ln -sf "$VERUS_RELEASE/libvstd.rlib"                 "$VERUS_ROOT/libvstd.rlib"
docker exec "$CONTAINER" ln -sf "$VERUS_RELEASE/vstd.vir"                     "$VERUS_ROOT/vstd.vir"

echo "Done. Verifying:"
docker exec "$CONTAINER" ls -la "$VERUS_ROOT" | grep "^l"
