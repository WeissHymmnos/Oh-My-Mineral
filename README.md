# Mineral GUI

Mineral 的桌面客户端，Qt 6 与 GTK 4 各有一个壳，共用同一套 Rust 会话核心（`mineral-gui-core`）与桥接进程（`mineral-gui-bridge`）。播放仍由 Mineral daemon 负责，界面只是它的 client。交互完整遵循 `docs/gui-interaction-design.md`：十种模式可以随时切换和叠加，搜索、队列、歌词和播放状态不因切换而复制或清空。

## 构建与测试

开发头文件与 `.pc` 配置放在本机 `.sysroot`（不需要 root），`.cargo/config.toml` 已自动指向它。

```bash
cargo test --workspace
cargo build --workspace
cargo build -p mineral-gui-bridge --release
```

桥接程序把 GUI 命令译成 Mineral 0.5 的 client 协议。没有 `mineral serve` 时，两个界面都会显示 Daemon 断连，并可在菜单/工具栏中一键启动本机的 `mineral serve` 或重连。

## 运行

```bash
./target/debug/mineral-gui-qt
./target/debug/mineral-gui-gtk
```

## 常用快捷键

- `Ctrl+Shift+M`：打开模式中心（切换 10 种界面方案或保存自定义组合）
- `Ctrl+K`：打开命令与全局跳转面板（输入 `>` 筛选命令，或直接输入关键词搜索/播放）
- `Space`：播放 / 暂停
- `Enter` / 双击：播放歌曲或打开歌单/专辑/艺人详情
- `Shift+Enter`：将选中歌曲加入播放队列
- `j` / `k`：上下移动列表选中项
- `N` / `P`：下一首 / 上一首
- `/`：聚焦当前列表过滤器
- `Tab`：打开 / 关闭播放队列抽屉
- `T` / `M`：切换歌词轨道（原文/翻译/罗马音）/ 切换循环模式
- `Z` / `Esc`：进入沉浸模式 / 返回上一层或退出全屏与迷你窗
- `?`：查看快捷键说明

