#!/bin/bash
# download.sh

set -e

BINUTILS_VERSION=binutils-2.25
GCC_VERSION=gcc-5.2.0
LINUX_KERNEL_VERSION=linux-3.16
GLIBC_VERSION=glibc-2.22

wget -nc http://ftpmirror.gnu.org/binutils/$BINUTILS_VERSION.tar.gz
wget -nc http://ftpmirror.gnu.org/gcc/gcc-5.2.0/$GCC_VERSION.tar.gz
wget -nc https://www.kernel.org/pub/linux/kernel/v3.x/$LINUX_KERNEL_VERSION.tar.xz
wget -nc http://ftpmirror.gnu.org/glibc/$GLIBC_VERSION.tar.xz

for f in *.tar*; do
    tar xfk $f
done

cd $GCC_VERSION
./contrib/download_prerequisites
