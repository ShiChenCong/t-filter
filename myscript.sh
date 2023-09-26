#!/bin/bash

# 执行一个命令
ls

# 获取上一个命令的返回值
return_code=$?

# 打印返回值
echo "命令的返回值是: $return_code"

# 根据返回值执行其他逻辑
if [ $return_code -eq 0 ]; then
    echo "命令执行成功"
else
    echo "命令执行失败"
fi

