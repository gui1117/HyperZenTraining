APPVEYOR_WIN64_ARTEFACT=$1
APPVEYOR_WIN32_ARTEFACT=$2
STEAM_CMD=/home/thiolliere/steam-sdk/tools/ContentBuilder/builder_linux/steamcmd.sh

rm -rf target/artefact/
mkdir -p target/artefact/
cd target/artefact/
wget $1
wget $2
mv hyperzen-training-x86_64-pc-windows-msvc.exe hyperzen-training-win64.exe
mv hyperzen-training-i686-pc-windows-msvc.exe hyperzen-training-win32.exe
cd ../..

cargo build --release
cargo build --release --target i686-unknown-linux-gnu

cp target/release/hyperzen-training target/artefact/hyperzen-training-linux64
cp target/i686-unknown-linux-gnu/release/hyperzen-training target/artefact/hyperzen-training-linux32

rm -rf target/steam/
mkdir -p target/steam/output/
mkdir -p target/steam/content/

$STEAM_CMD +login "thiolliere" "$(pass steam)" +run_app_build /home/thiolliere/Developpement/pepe/steam_scripts/app_build_884160.vdf +quit
