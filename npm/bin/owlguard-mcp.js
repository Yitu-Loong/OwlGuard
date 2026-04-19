#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const platform = process.platform;
const arch = process.arch;

function getBinaryName() {
  if (platform === 'win32') return 'owlguard-mcp.exe';
  return 'owlguard-mcp';
}

function getBinaryPath() {
  const binaryDir = path.join(__dirname, '..', 'bin');
  const binaryName = getBinaryName();
  const binaryPath = path.join(binaryDir, binaryName);

  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }

  const platformArch = `${platform}-${arch}`;
  const platformBinaryPath = path.join(binaryDir, platformArch, binaryName);

  if (fs.existsSync(platformBinaryPath)) {
    return platformBinaryPath;
  }

  console.error(`枭卫(OwlGuard) MCP 二进制文件未找到: ${binaryPath}`);
  console.error(`请尝试重新安装: npm install owlguard-mcp`);
  console.error(`或从源码编译: https://github.com/Yitu-Loong/OwlGuard`);
  process.exit(1);
}

const binaryPath = getBinaryPath();

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: { ...process.env }
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

child.on('error', (err) => {
  console.error(`启动枭卫MCP失败: ${err.message}`);
  process.exit(1);
});
