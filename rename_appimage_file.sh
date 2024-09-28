#!/bin/bash

script_dir=$(readlink -f $(dirname "$0"))

VERSION=1.0.1
APPIMAGE_PATH="$script_dir/src-tauri/target/release/bundle/appimage"
CENTOS_APPIMAGE_FILE="vocab-builder_${VERSION}_amd64.AppImage"
UBUNTU_APPIMAGE_FILE="vocab-builder_${VERSION}_amd64.AppImage"


echo $script_dir
function get_distro() {
  if [ -f /etc/lsb-release ]; then
    DISTRO="ubuntu"
  elif [ -f /etc/redhat-release ]; then
    DISTRO="centOS"
  else
    echo Unknown distro. Exiting...
    exit
  fi
  echo $DISTRO
}

distro=$(get_distro)

if [[ "${distro}" -eq "ubuntu" ]];then
  echo The release is Ubuntu
  old_file_name=${UBUNTU_APPIMAGE_FILE}
elif [[ "${distro}" -eq "centos" ]];then
  echo The release is CentOS
  old_file_name=${CENTOS_APPIMAGE_FILE}
else
  echo Unknown release
fi

new_file_name="vocab-builder_${VERSION}_${distro}_amd64.AppImage"

if [[ ! -f $APPIMAGE_PATH/$old_file_name ]];then
  echo File $APPIMAGE_PATH/"$old_file_name" does not exist
else
  echo renaming ${APPIMAGE_PATH}/"$old_file_name" to ${APPIMAGE_PATH}/"$new_file_name"
pwd
  mv ${APPIMAGE_PATH}/${old_file_name} ${APPIMAGE_PATH}/${new_file_name}
fi