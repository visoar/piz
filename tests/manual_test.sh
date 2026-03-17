#!/bin/bash
# ============================================================
# piz v0.2.3 手动测试脚本 (bash/zsh)
# 使用方法: 逐段复制执行，观察结果
# ============================================================

echo ""
echo "========================================"
echo " piz v0.2.3 手动测试 (bash/zsh)"
echo "========================================"
echo ""

# ----------------------------------------------------------
# 0. 准备工作
# ----------------------------------------------------------
echo "=== 0. 准备工作 ==="

echo "[0.1] 版本确认:"
echo "  运行: piz --version"
echo "  预期: piz 0.2.3"
# piz --version

echo ""
echo "[0.2] 清除缓存:"
echo "  运行: piz clear-cache"
# piz clear-cache

# ----------------------------------------------------------
# 1. 基本命令生成
# ----------------------------------------------------------
echo ""
echo "=== 1. 基本命令生成 ==="

echo "[1.1] 运行: piz list all files"
echo "  预期: ls -la 或类似"
# piz list all files

echo "[1.2] 运行: piz show disk usage"
echo "  预期: df -h 或类似"
# piz show disk usage

# ----------------------------------------------------------
# 2. 缓存策略测试
# ----------------------------------------------------------
echo ""
echo "=== 2. 缓存策略测试 ==="

echo "[2.1] 运行一个会失败的命令:"
echo "  运行: piz run program xxxyyy"
echo "  预期: 失败，选择不修复"
# piz run program xxxyyy

echo "[2.2] 同一查询再次运行:"
echo "  运行: piz run program xxxyyy"
echo "  预期: 重新调用 LLM，不走缓存"
# piz run program xxxyyy

echo "[2.3] 运行一个会成功的命令:"
echo "  运行: piz show current date"
# piz show current date

echo "[2.4] 同一查询再次运行:"
echo "  运行: piz show current date"
echo "  预期: 显示 [cached]"
# piz show current date

# ----------------------------------------------------------
# 3. --eval 模式测试
# ----------------------------------------------------------
echo ""
echo "=== 3. --eval 模式测试 ==="

echo "[3.1] eval 模式确认后写入文件:"
echo "  运行: piz --eval show current directory"
echo "  然后: cat ~/.piz/eval_command"
echo "  预期: 文件内容是 pwd"
# piz --eval show current directory
# cat ~/.piz/eval_command

echo "[3.2] eval 模式取消不写入:"
echo "  运行: piz --eval show disk space (选择取消)"
echo "  然后: ls ~/.piz/eval_command"
echo "  预期: 文件不存在"
# piz --eval show disk space

# ----------------------------------------------------------
# 4. piz init 测试
# ----------------------------------------------------------
echo ""
echo "=== 4. piz init 测试 ==="

echo "[4.1] 运行: piz init bash"
echo "  预期: 输出 piz() shell 函数"
# piz init bash

echo "[4.2] 运行: piz init fish"
echo "  预期: 输出 function piz"
# piz init fish

echo "[4.3] 运行: piz init unknown"
echo "  预期: 报错 Unsupported shell"
# piz init unknown

# ----------------------------------------------------------
# 5. shell 集成 E2E 测试
# ----------------------------------------------------------
echo ""
echo "=== 5. Shell 集成 E2E (cd 真正生效) ==="

echo "[5.1] 加载集成:"
echo '  运行: eval "$(piz init bash)"'
# eval "$(piz init bash)"

echo "[5.2] cd 测试:"
echo "  运行:"
echo "    pwd"
echo "    piz go to /tmp"
echo "    pwd  # 应该变成 /tmp"
# pwd && piz go to /tmp && pwd

echo "[5.3] 切回来:"
echo "    piz go back to home directory"
echo "    pwd  # 应该变成 ~"
# piz go back to home directory && pwd

# ----------------------------------------------------------
# 6. 安全测试
# ----------------------------------------------------------
echo ""
echo "=== 6. 安全测试 ==="

echo "[6.1] 危险命令:"
echo "  运行: piz delete all files in root"
echo "  预期: dangerous 警告"
# piz delete all files in root

echo "[6.2] 非命令拒绝:"
echo "  运行: piz hello"
echo "  预期: 拒绝"
# piz hello

echo ""
echo "========================================"
echo " 测试项列表结束，请逐项手动验证"
echo "========================================"
