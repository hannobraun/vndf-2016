cd source\rust\ide
cargo build
echo "IDE built. Add it to your path by executing the following command:" &&
echo "set PATH=%PATH%:output\rust\debug"
