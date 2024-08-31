#!/bin/bash
APPIMAGE_PATH="src-tauri/target/release/bundle/appimage"
APPIMAGE_FILE="vocab-builder_0.1.0_amd64.AppImage"
APPIMAGE_TOOL="appimagetool-x86_64.AppImage"

#Expand appimage and copy internal files form pyinstaller one-dir build to expected location
pushd $APPIMAGE_PATH
./$APPIMAGE_FILE --appimage-extract
popd
cp -r src-tauri/binaries/server/_internal $APPIMAGE_PATH/squashfs-root/usr/bin/

#Re-create the appimage file from the modified tree
$APPIMAGE_TOOL $APPIMAGE_PATH/squashfs-root $APPIMAGE_PATH/$APPIMAGE_FILE

rm -rf $APPIMAGE_PATH/squashfs-root
