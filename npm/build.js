const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function build() {
  const binDir = path.join(__dirname, 'bin');

  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  console.log('正在编译枭卫(OwlGuard) MCP...');

  try {
    const projectRoot = path.resolve(__dirname, '..');
    execSync('cargo build --release', {
      cwd: projectRoot,
      stdio: 'inherit',
    });

    const binaryName = process.platform === 'win32' ? 'owlguard-mcp.exe' : 'owlguard-mcp';
    const sourcePath = path.join(projectRoot, 'target', 'release', binaryName);
    const destPath = path.join(binDir, binaryName);

    if (fs.existsSync(sourcePath)) {
      fs.copyFileSync(sourcePath, destPath);
      if (process.platform !== 'win32') {
        fs.chmodSync(destPath, 0o755);
      }
      console.log(`二进制文件已复制到: ${destPath}`);
    } else {
      console.error(`编译产物未找到: ${sourcePath}`);
      process.exit(1);
    }
  } catch (err) {
    console.error(`编译失败: ${err.message}`);
    process.exit(1);
  }
}

build();
