SET vndf_path=%CD%
cd source/rust/pan
cargo build
@echo off
(echo IDE built. Add it to your path by executing the following command:
 echo setx PATH "%%PATH%%;%vndf_path%\output\cargo\debug")
cd ..
cd ..
cd ..
