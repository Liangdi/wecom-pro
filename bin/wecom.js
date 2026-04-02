#!/usr/bin/env node

import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import os from 'node:os';
import { join } from 'node:path';

const require = createRequire(import.meta.url);

/**
 * Returns the platform-specific package name based on current OS and CPU architecture.
 */
function getPlatformPackage() {
  const platform = os.platform();
  const arch = os.arch();

  const platformMap = {
    'darwin-arm64': '@liangdi/wecom-pro-darwin-arm64',
    'darwin-x64': '@liangdi/wecom-pro-darwin-x64',
    'linux-x64': '@liangdi/wecom-pro-linux-x64',
    'win32-x64': '@liangdi/wecom-pro-win32-x64',
  };

  const key = `${platform}-${arch}`;
  const pkg = platformMap[key];

  if (!pkg) {
    console.error(
      `Error: unsupported platform ${platform}-${arch}.\n` +
        `Supported platforms: ${Object.keys(platformMap).join(', ')}`,
    );
    process.exit(1);
  }

  return pkg;
}

/**
 * Resolves the path to the platform-specific binary.
 */
function getBinaryPath() {
  const pkg = getPlatformPackage();
  const binaryName = os.platform() === 'win32' ? 'wecom-pro.exe' : 'wecom-pro';

  try {
    const pkgDir = require.resolve(`${pkg}/package.json`);
    return join(pkgDir, '..', 'bin', binaryName);
  } catch {
    console.error(
      `Error: cannot find @liangdi/wecom-pro binary.\n` +
        `Please try reinstalling: npm install @liangdi/wecom-pro\n\n` +
        `If the problem persists, check:\n` +
        `  1. Your npm config does not disable optional dependencies (--no-optional)\n` +
        `  2. Your platform (${os.platform()}-${os.arch()}) is supported`,
    );
    process.exit(1);
  }
}

// Execute the binary, passing through all arguments
const binaryPath = getBinaryPath();

try {
  execFileSync(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    env: process.env,
  });
} catch (error) {
  if (error.status != null) {
    process.exit(error.status);
  }
}
