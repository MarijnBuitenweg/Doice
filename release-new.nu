# Build all
# cargo build -p doice_os --release # Deprecated since 0.2.6
cargo build -p doice_smol --release
# Get Doice version
let doice_ver = (open cargo.toml | get package.version)
# Move files to proper place, and rename them
cp target/release/doice_smol.exe ("pre-releases/Doice_Smol/Doice_Smol-" + $doice_ver + ".exe") -i
# cp target/release/doice_os.exe ("pre-releases/DoiceOS/DoiceOS-" + $doice_ver + ".exe") -i # Deprecated since 0.2.6