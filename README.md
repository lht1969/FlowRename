# FlowRename - 批量文件重命名工具

FlowRename 是一款功能强大的跨平台批量文件重命名工具，支持 Windows、Linux 和 macOS 三大操作系统。

## 目录

- [功能特性](#功能特性)
- [环境要求](#环境要求)
- [快速开始](#快速开始)
- [详细构建指南](#详细构建指南)
  - [Windows](#windows-系统)
  - [Linux](#linux-系统)
  - [macOS](#macos-系统)
- [执行方式](#执行方式)
- [项目结构](#项目结构)
- [常见问题](#常见问题)

---

## 功能特性

- **批量重命名**: 支持同时对多个文件进行重命名操作
- **多种重命名方式**:
  - 添加前缀/后缀
  - 替换文字
  - 编号递增
  - 修剪空白
  - 更改大小写
  - 时间戳处理
  - 正则表达式支持
- **文件预览**: 支持实时预览重命名结果，避免误操作
- **撤销/恢复**: 支持撤销最近的重命名操作
- **文件过滤**: 支持按标签、类型等条件过滤文件
- **中文排序**: 智能中文拼音排序
- **元数据读取**: 支持读取图片、音频、视频文件的元数据

---

## 环境要求

### 通用环境

| 组件    | 版本要求 | 说明             |
| ------- | -------- | ---------------- |
| Node.js | >= 18.0  | 前端构建工具     |
| npm     | >= 9.0   | Node.js 包管理器 |
| Rust    | >= 1.70  | 后端编译环境     |

### 各操作系统特定要求

#### Windows

- Windows 10/11 (64位)
- Visual Studio Build Tools 或 Microsoft Visual Studio
- WebView2 运行时 (通常已内置于 Windows 10/11)

#### Linux

- Ubuntu 20.04+ / Debian 11+ / Fedora 36+ / Arch Linux
- GTK3 库
- WebKit2GTK 4.1
- libappindicator (用于系统托盘)

#### macOS

- macOS 10.15 (Catalina) 或更高版本
- Xcode 命令行工具
- Apple Silicon 或 Intel 处理器

---

## 快速开始

### 1. 克隆项目

```bash
git clone <repository-url>
cd rename_master
```

### 2. 安装依赖

**Windows (PowerShell):**

```powershell
.\scripts\install-deps.ps1
```

**Linux/macOS (Bash):**

```bash
chmod +x ./scripts/install-deps.sh
./scripts/install-deps.sh
```

### 3. 开发模式运行

**Windows:**

```powershell
.\scripts\build.ps1 dev
```

**Linux/macOS:**

```bash
./scripts/build.sh dev
```

### 4. 构建发布版本

**Windows:**

```powershell
.\scripts\build.ps1 release
```

**Linux/macOS:**

```bash
./scripts/build.sh release
```

---

## 详细构建指南

### Windows 系统

#### 方式一：使用 PowerShell 脚本 (推荐)

1. **以管理员身份打开 PowerShell**

2. **安装 Rust (如果未安装)**

   ```powershell
   irm https://rustup.rs | iex
   ```

3. **安装 Node.js**
   从 https://nodejs.org 下载并安装 LTS 版本

4. **安装 Visual Studio Build Tools**
   - 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
   - 安装时选择 "C++ 生成工具" 工作负载

5. **运行安装脚本**

   ```powershell
   .\scripts\install-deps.ps1
   ```

6. **构建项目**

   ```powershell
   # 开发模式
   .\scripts\build.ps1 dev

   # 发布模式 (生成 MSI 和 NSIS 安装包)
   .\scripts\build.ps1 release
   ```

#### 方式二：手动构建

```powershell
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 发布构建
npm run tauri build -- --bundles msi,nsis
```

#### 构建产物位置

```
src-tauri/target/release/bundle/
├── msi/          # MSI 安装包
├── nsis/         # NSIS 安装包
└── exe/          # 可执行文件
```

---

### Linux 系统

#### 方式一：使用 Bash 脚本 (推荐)

1. **安装系统依赖**

   **Ubuntu/Debian:**

   ```bash
   sudo apt-get update
   sudo apt-get install -y \
       libwebkit2gtk-4.1-dev \
       libgtk-3-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev \
       patchelf \
       curl \
       wget \
       file
   ```

   **Fedora:**

   ```bash
   sudo dnf install -y \
       webkit2gtk4.1-devel \
       gtk3-devel \
       libappindicator-gtk3-devel \
       librsvg2-devel
   ```

   **Arch Linux:**

   ```bash
   sudo pacman -S --noconfirm \
       webkit2gtk-4.1 \
       gtk3 \
       libappindicator-gtk3 \
       librsvg
   ```

2. **安装 Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **安装 Node.js**

   ```bash
   # 使用 nvm (推荐)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install --lts
   nvm use --lts
   ```

4. **运行安装脚本**

   ```bash
   chmod +x ./scripts/install-deps.sh
   ./scripts/install-deps.sh
   ```

5. **构建项目**

   ```bash
   # 开发模式
   ./scripts/build.sh dev

   # 发布模式
   ./scripts/build.sh release
   ```

#### 方式二：手动构建

```bash
# 克隆并进入目录
git clone <repository-url>
cd rename_master

# 安装 Node 依赖
npm install

# 开发模式
npm run tauri dev

# 发布构建 (生成 AppImage 和 deb 包)
npm run tauri build
```

#### 构建产物位置

```
src-tauri/target/release/bundle/
├── appimage/      # AppImage 便携版
├── deb/          # Debian/Ubuntu 安装包
└── rpm/          # Fedora/RHEL 安装包
```

---

### macOS 系统

#### 方式一：使用 Bash 脚本 (推荐)

1. **安装 Xcode 命令行工具**

   ```bash
   xcode-select --install
   ```

   在弹出的对话框中点击 "安装"

2. **安装 Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **安装 Node.js (推荐使用 nvm)**

   ```bash
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   export NVM_DIR="$HOME/.nvm"
   [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
   nvm install --lts
   nvm use --lts
   ```

4. **运行安装脚本**

   ```bash
   chmod +x ./scripts/install-deps.sh
   ./scripts/install-deps.sh
   ```

5. **构建项目**

   ```bash
   # 开发模式
   ./scripts/build.sh dev

   # 发布模式
   ./scripts/build.sh release
   ```

#### 方式二：手动构建

```bash
# 克隆并进入目录
git clone <repository-url>
cd rename_master

# 安装 Node 依赖
npm install

# 开发模式
npm run tauri dev

# 发布构建 (生成 .app 和 .dmg)
npm run tauri build
```

#### 注意事项

- 如果使用 Apple Silicon (M1/M2/M3)，确保 Rust 已配置为支持 aarch64-apple-darwin
- 如果遇到权限问题，可能需要给脚本添加执行权限: `chmod +x scripts/*.sh`

#### 构建产物位置

```
src-tauri/target/release/bundle/
├── app/          # macOS 应用包
├── dmg/          # DMG 安装镜像
└── app.tar.gz    # 应用存档
```

---

## 执行方式

### 开发模式

开发模式会启动热重载的前端开发服务器和 Tauri 应用:

**Windows:**

```powershell
.\scripts\build.ps1 dev
```

**Linux/macOS:**

```bash
./scripts/build.sh dev
```

### 调试模式

启动带有调试工具的开发版本:

**Windows:**

```powershell
.\scripts\run.ps1 -Debug
```

**Linux/macOS:**

```bash
./scripts/run.sh --debug
```

### 运行已构建的应用

构建完成后，可以直接运行生成的可执行文件:

| 操作系统 | 可执行文件位置                                                                 |
| -------- | ------------------------------------------------------------------------------ |
| Windows  | `src-tauri/target/release/bundle/exe/FlowRename.exe`                           |
| Linux    | `src-tauri/target/release/bundle/appimage/FlowRename`                          |
| macOS    | `src-tauri/target/release/bundle/app/FlowRename.app/Contents/MacOS/FlowRename` |

### NPM 脚本命令

除脚本外，也可以直接使用 npm 命令:

```bash
# 开发模式
npm run tauri dev

# 构建 (指定bundles)
npm run tauri build -- --bundles msi          # Windows MSI
npm run tauri build -- --bundles nsis         # Windows NSIS
npm run tauri build -- --bundles appimage     # Linux AppImage
npm run tauri build -- --bundles deb          # Linux DEB
npm run tauri build -- --bundles dmg          # macOS DMG
npm run tauri build -- --bundles app          # macOS App

# 构建所有平台的安装包
npm run tauri build
```

---

## 项目结构

```
rename_master/
├── src/                          # SvelteKit 前端源代码
│   ├── lib/                      # 组件和工具库
│   │   ├── components/           # UI 组件
│   │   │   ├── ControlPanel.svelte
│   │   │   ├── FileList.svelte
│   │   │   ├── MethodPanel.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   ├── TagPanel.svelte
│   │   │   └── TitleBar.svelte
│   │   │   └── method-editors/   # 各重命名方式的编辑器组件
│   │   ├── stores/               # Svelte 状态管理
│   │   └── utils/                # 工具函数
│   └── routes/                   # 页面路由
├── src-tauri/                    # Rust 后端源代码
│   ├── src/
│   │   ├── commands/             # Tauri 命令 (与前端交互)
│   │   ├── file_manager/        # 文件管理模块
│   │   ├── metadata_reader/     # 元数据读取 (EXIF/ID3/视频)
│   │   ├── method_engine/       # 重命名方法引擎
│   │   ├── methods/             # 具体重命名实现
│   │   ├── models/              # 数据模型
│   │   ├── tag_system/          # 标签系统
│   │   ├── lib.rs               # 库入口
│   │   └── main.rs              # 程序入口
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置文件
├── scripts/                     # 构建和执行脚本
│   ├── build.ps1                 # Windows 构建脚本
│   ├── build.sh                 # Linux/macOS 构建脚本
│   ├── run.ps1                  # Windows 执行脚本
│   ├── run.sh                   # Linux/macOS 执行脚本
│   ├── install-deps.ps1         # Windows 依赖安装脚本
│   ├── install-deps.sh          # Linux/macOS 依赖安装脚本
│   └── create-portable-zip.ps1  # 便携版打包脚本
├── package.json                 # Node.js 项目配置
├── Cargo.toml                   # Rust workspace 配置
└── README.md                    # 项目文档
```

---

## 常见问题

### 1. 编译时出现 Rust 错误

**问题**: `error: failed to run custom build command for tauri-build`

**解决方案**:

```bash
# 更新 Rust 到最新版本
rustup update

# 清除缓存并重新构建
cargo clean
npm run tauri build
```

### 2. WebView2 错误 (Windows)

**问题**: `WebView2 runtime not found`

**解决方案**:

- 下载并安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### 3. Linux 下缺少 WebKit 库

**问题**: `error: Failed to find webkit2gtk-4.1`

**解决方案**:

```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel
```

### 4. macOS 上签名错误

**问题**: `error: cannot find code object on disk`

**解决方案**:
在 `src-tauri/tauri.conf.json` 中暂时禁用签名:

```json
{
  "bundle": {
    "macOS": {
      "signingIdentity": null
    }
  }
}
```

### 5. Windows Defender 阻止运行

**问题**: 生成的可执行文件被 Windows Defender 标记

**解决方案**:

- 这是因为应用未签名，属于正常现象
- 可以临时关闭 Defender 进行测试
- 或将 .exe 所在目录添加到排除项

### 6. 内存不足错误

**问题**: `error: could not compile .../out of memory`

**解决方案**:

```bash
# 减少并行编译数量
export CARGO_BUILD_JOBS=2

# 或者在 package.json 中设置
npm run tauri build -- --jobs 2
```

### 7. npm install 失败

**问题**: `npm ERR! network timeout`

**解决方案**:

```bash
# 使用国内镜像
npm config set registry https://registry.npmmirror.com
npm install
```

### 8. 脚本执行策略错误 (Windows)

**问题**: `cannot be loaded because running scripts is disabled`

**解决方案**:

```powershell
# 修改执行策略
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# 然后重新运行脚本
.\scripts\install-deps.ps1
```

---

## 技术栈

- **前端框架**: SvelteKit 2.x
- **UI 组件**: Skeleton UI 2.x
- **样式**: TailwindCSS 3.x
- **后端框架**: Tauri 2.x
- **后端语言**: Rust
- **构建工具**: Vite 5.x

---

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。

---

## 获取帮助

- 提交 Issue: https://github.com/lht1969/FlowRename/issues
- 文档更新: 欢迎提交 PR 来完善本 README
