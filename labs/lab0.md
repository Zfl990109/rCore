# rCore 实验 - Lab0

### 简介

- 关于 rCore-Tutorial-Book 第零章的一些笔记
- 请搭配 rCore-Tutorial-Book 食用

### 实验环境配置(windows 10 操作系统 + WSL2)

- 按照指导书安装好 `WSL2`，并安装好 `ubuntu`

- 在 `Windows` 下安装好 `vscode`，在 `ubuntu` 的 `.bashrc` 文件中添加 `alias code="/mnt/d/Microsoft VS Code/Code.exe"` ，接下来就可以直接在命令行输入 `code` 来打开 `Windows` 下的 `vscode` 写代码

- 更换 ubuntu 源

- 按照指导书安装好 **C 开发环境** 、 **Rust 开发环境（nightly 1.63.0）** 以及 **Qemu**

- 安装好 **riscv64** 的相关工具链（riscv64-unknown-elf-toolchain-10.2.0-2020.12.8-x86_64-linux-ubuntu14），并且在 .bashrc 文件中添加

  ```
  export PATH=$PATH:$HOME/riscv64-unknown-elf-toolchain-10.2.0-2020.12.8-x86_64-linux-ubuntu14/bin
  ```

- `git clone https://github.com/rcore-os/rCore-Tutorial-v3.git` 将 `rCore` 项目 克隆到本地


### 关于 rCore 本地编译运行时出现的问题以及解决方案

- 试图从 github 上下载 `riscv `、`virtio-drivers`、`k210-pac`、`k210-hal`、`k210-soc`几个库的时候卡死

  解决措施：从 `2022spring` 分支中下载 `third-party` 第三方安装包，复制到 rCore 目录下，然后将 `os/Cargo.toml` 替换为：

  ```
  riscv = { path = "../third-party/riscv", features = ["inline-asm"] }
  virtio-drivers = { path = "../third-party/virtio-drivers" }
  k210-pac = { path = "../third-party/k210-pac" }
  k210-hal = { path = "../third-party/k210-hal" }
  k210-soc = { path = "../third-party/k210-soc" }
  ```

- rust 版本不一致

  解决措施：将 `rCore-Tutorial-V3/rust-toolchain` 中的版本更改为对应的版本，例如：

  ```
  nightly-2022-06-14
  ```






