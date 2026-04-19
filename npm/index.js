const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

function getBinaryPath() {
  const binaryName = process.platform === 'win32' ? 'owlguard-mcp.exe' : 'owlguard-mcp';
  const binaryPath = path.join(__dirname, 'bin', binaryName);

  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }

  return null;
}

function startServer() {
  const binaryPath = getBinaryPath();

  if (!binaryPath) {
    console.error('枭卫(OwlGuard) MCP 二进制文件未找到');
    console.error('请运行: npm run build 或 npm install');
    process.exit(1);
  }

  const child = spawn(binaryPath, [], {
    stdio: 'inherit',
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });

  child.on('error', (err) => {
    console.error(`启动失败: ${err.message}`);
    process.exit(1);
  });
}

module.exports = { startServer, getBinaryPath };

if (require.main === module) {
  startServer();
}
