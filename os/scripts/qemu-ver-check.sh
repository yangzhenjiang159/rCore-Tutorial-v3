#!/bin/sh

# Argument1: The filename of qemu executable, e.g. qemu-system-riscv64
# 查找 qemu 可执行文件的路径
QEMU_PATH=$(which $1)
# 获取查找命令的返回值
RET=$?
# 定义 qemu 最低主版本号
MINIMUM_MAJOR_VERSION=7
# 定义颜色常量
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# 如果查找失败
if [ $RET!= 0 ]
then
    # 打印错误信息
    echo "$1 not found"
    # 以非零状态码退出，表示有错误发生
    exit 1
else
    # 获取 qemu 版本信息
    QEMU_VERSION=$($1 --version|head -n 1|awk '{print $4}')
    # 提取 qemu 主版本号
    MAJOR_VERSION=$(echo $QEMU_VERSION|cut -c1-1)
    # 如果 qemu 主版本号低于最低要求
    if [ $MAJOR_VERSION -lt $MINIMUM_MAJOR_VERSION ]
    then
        # 打印错误信息
        echo "${RED}Error: Required major version of QEMU is ${MINIMUM_MAJOR_VERSION}, " \
             "but current is ${QEMU_VERSION}.${NC}"
        # 以非零状态码退出，表示有错误发生
        exit 1
    else
        # 打印成功信息
        echo "${GREEN}QEMU version is ${QEMU_VERSION}(>=${MINIMUM_MAJOR_VERSION}), OK!${NC}"
        # 以零状态码退出，表示成功
        exit 0
    fi
fi
