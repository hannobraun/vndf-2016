#!/bin/bash
if test "x$BINDGEN" == "x"; then
    BINDGEN=bindgen
fi
$BINDGEN -DSTBI_HEADER_FILE_ONLY -match stb_image -o stb_image.rs stb_image.c

