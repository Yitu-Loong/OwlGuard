const https = require('https');
const http = require('http');
const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

const VERSION = '0.1.0';
const GITHUB_REPO = 'Yitu-Loong/OwlGuard';

function getPlatformTarget() {
  const platform = process.platform;
  const arch = process.arch;

  const platformMap = {
    'win32-x64': 'windows-amd64',
    'darwin-x64': 'darwin-amd64',
    'darwin-arm64': 'darwin-arm64',
    'linux-x64': 'linux-amd64',
    'linux-arm64': 'linux-arm64',
  };

  const key = `${platform}-${arch}`;
  const target = platformMap[key];

  if (!target) {
    console.error(`不支持的平台: ${key}`);
    console.error('支持的平台: windows-amd64, darwin-amd64, darwin-arm64, linux-amd64, linux-arm64');
    console.error('请从源码编译: https://github.com/Yitu-Loong/OwlGuard');
    process.exit(1);
  }

  return target;
}

function getBinaryName() {
  return process.platform === 'win32' ? 'owlguard-mcp.exe' : 'owlguard-mcp';
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const protocol = url.startsWith('https') ? https : http;

    const request = (currentUrl) => {
      protocol.get(currentUrl, { timeout: 30000 }, (response) => {
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          request(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(`下载失败，HTTP状态码: ${response.statusCode}`));
          return;
        }

        const file = fs.createWriteStream(dest);
        response.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      }).on('error', (err) => {
        reject(err);
      });
    };

    request(url);
  });
}

async function install() {
  const target = getPlatformTarget();
  const binaryName = getBinaryName();
  const binDir = path.join(__dirname, 'bin');

  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const binaryPath = path.join(binDir, binaryName);

  if (fs.existsSync(binaryPath)) {
    console.log('枭卫(OwlGuard) MCP 二进制文件已存在，跳过下载');
    return;
  }

  const ext = process.platform === 'win32' ? 'zip' : 'tar.gz';
  const archiveName = `owlguard-mcp-${VERSION}-${target}.${ext}`;
  const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${archiveName}`;

  console.log(`正在下载枭卫(OwlGuard) MCP v${VERSION} (${target})...`);
  console.log(`下载地址: ${downloadUrl}`);

  try {
    const archivePath = path.join(binDir, archiveName);
    await download(downloadUrl, archivePath);

    console.log('正在解压...');

    if (process.platform === 'win32') {
      const { execSync } = require('child_process');
      execSync(`powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${binDir}' -Force"`, { stdio: 'inherit' });
    } else {
      const { execSync } = require('child_process');
      execSync(`tar -xzf "${archivePath}" -C "${binDir}"`, { stdio: 'inherit' });
    }

    fs.unlinkSync(archivePath);

    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }

    console.log('枭卫(OwlGuard) MCP 安装完成！');
  } catch (err) {
    console.error(`下载失败: ${err.message}`);
    console.error('');
    console.error('你可以从源码编译安装:');
    console.error('  git clone https://github.com/Yitu-Loong/OwlGuard.git');
    console.error('  cd OwlGuard && cargo build --release');
    console.error('');
    console.error('然后将编译好的二进制文件放到: ' + binDir);

    if (fs.existsSync(binaryPath)) {
      fs.unlinkSync(binaryPath);
    }
  }
}

install();
