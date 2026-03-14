# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
################################################################################
#!/bin/bash
set -euxo pipefail

echo "SRC=$SRC"
pwd
ls -la
ls -la "$SRC"
find "$SRC" -maxdepth 3 -type f | sort | head -200

cd "$SRC"

test -f Cargo.toml
test -d fuzz

cargo install cargo-fuzz --locked || true
cargo fuzz build -O

find fuzz/target -maxdepth 4 -type f | sort

cp fuzz/target/x86_64-unknown-linux-gnu/release/compat "$OUT/" || true
cp fuzz/target/x86_64-unknown-linux-gnu/release/roundtrip "$OUT/" || true
ls -la "$OUT"