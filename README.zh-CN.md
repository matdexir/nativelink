[English](README.md) | **简体中文** | [繁體中文](README.zh-TW.md)

<div id="logo" align="center">
  <a href="https://www.nativelink.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg" />
      <source media="(prefers-color-scheme: light)" srcset="assets/logo-light.svg" />
      <img alt="NativeLink" src="assets/logo-light.svg" width="376" height="100" />
    </picture>
  </a>

  <br />
</div>

<div id="description" align="center">
  enter the shipstorm
</div>

<br />

<div id="badges" align="center">

  [![Homepage](https://img.shields.io/badge/Homepage-8A2BE2)](https://nativelink.com)
  [![GitHub stars](https://img.shields.io/github/stars/tracemachina/nativelink?style=social)](https://github.com/TraceMachina/nativelink)
  [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/TraceMachina/nativelink/badge)](https://securityscorecards.dev/viewer/?uri=github.com/TraceMachina/nativelink)
  [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/8050/badge)](https://www.bestpractices.dev/projects/8050)
  [![Slack](https://img.shields.io/badge/slack--channel-blue?logo=slack)](https://forms.gle/LtaWSixEC6bYi5xF7)
  [![License](https://img.shields.io/badge/License-FSL--1.1--Apache--2.0-blue.svg)](LICENSE)
</div>

<a href="https://trendshift.io/repositories/11202" target="_blank"><img src="https://trendshift.io/api/badge/repositories/11202" alt="TraceMachina%2Fnativelink | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/></a>

## NativeLink 是什么？

NativeLink 是一个高效、高性能的构建缓存与远程执行系统，能够在加速软件编译与测试的同时降低基础设施成本。它通过智能缓存构建产物并将任务分发到多台机器，为各种规模的项目优化构建流程。当机器人/自动驾驶、半导体、生物计算，或 Chromium 级别 / 移动端 monorepo 需要共享缓存与分布式执行时，NativeLink 能让未改动的 target 在笔记本电脑与 CI 之间永远不必重新编译。

NativeLink 在生产环境中备受信赖，能够降低成本并缩短开发者迭代时间——每月为客户处理超过**数十亿次请求**，客户包括 **Samsung** 等大型企业。

NativeLink 尤其擅长：

- Chromium 级别和移动端 monorepo（C++/ObjC/Swift/Kotlin/Java），拥有繁重的 CI 构建图
- 基于 Bazel/Buck 的机器人与自动驾驶 C++/CUDA 技术栈（感知、规划、仿真、硬件在环测试）
- 半导体流程（SystemVerilog/Verilog RTL、C/C++ 固件、与 EDA 相关的 Bazel/Buck 工具链）
- C++/CUDA 与 Python 原生管线的生物计算（生物信息学、影像重建、生物仿真）

<p align="center">
  <a href="https://www.youtube.com/watch?v=WLpqFuyLMUQ">
      <img src="https://trace-github-resources.s3.us-east-2.amazonaws.com/harper-90-thumbnail.webp" alt="NativeLink Explained in 90 seconds" loading="lazy" width="480" />
  </a>
</p>

## 🤖 面向 AI 智能体与智能协作者

NativeLink 的设计既能被人阅读，也能被智能体阅读：

- 仓库根目录下的 [`llms.txt`](llms.txt)、[`llms-small.txt`](llms-small.txt) 和
  [`llms-full.txt`](llms-full.txt) 分别以三种规模收录了网站、博客和文档；
  它们就是 [nativelink.com/llms.txt](https://nativelink.com/llms.txt) 上提供的文件。
  文档站点也有自己的三个文件：[docs.nativelink.com/llms.txt](https://docs.nativelink.com/llms.txt)，
  根目录的副本已将其合并进来。
- [`AGENTS.md`](AGENTS.md) 是供智能体修改代码时使用的代码地图。
- [`.claude/skills/`](.claude/skills) 提供可安装的技能，包括
  [`migrate-to-bazelmod`](.claude/skills/migrate-to-bazelmod)。
- [nativelink.com](https://nativelink.com) 和文档的每个页面都带有
  "Ask AI" 区块，可以在 ChatGPT、Claude、Perplexity、Google AI Mode 或 Copilot 中打开该站点。
- 欢迎借助智能体进行贡献，但需要有真人对改动负责；
  详见 [`CONTRIBUTING.md`](CONTRIBUTING.md#ai-assisted-contributions)。

完整列表见 [nativelink.com/agents](https://nativelink.com/agents)。

## 🔑 核心特性

1. **先进的构建缓存**：
   - 存储并复用未改动组件的历史构建结果
   - 显著缩短构建时间，尤其是增量变更时

2. **高效的远程执行**：
   - 将构建与测试任务分发到机器网络中
   - 并行化工作负载，加快完成速度
   - 利用远程资源，将计算负担从本地机器卸载
   - 通过统一、受控的构建环境确保一致性

NativeLink 可与使用 Remote Execution 协议的构建工具无缝集成，例如 [Bazel](https://bazel.build)、[Buck2](https://buck2.build)、[Goma](https://chromium.googlesource.com/infra/goma/client/) 和 [Siso](https://chromium.googlesource.com/build/+/refs/heads/main/siso/README.md)。CMake 项目也可以通过 [`recc`](https://buildgrid.gitlab.io/recc) 接入，参见[使用 NativeLink 构建 CMake 项目](https://docs.nativelink.com/getting-started/connect-your-build/cmake-recc)。它支持 Unix 系操作系统和 Windows，确保在不同开发环境中的广泛兼容性。

## 🚀 快速上手

你可以像下面这样以 Docker 镜像部署 NativeLink。以下配置均为**生产级**安装。如需从源码构建，请参考贡献文档中的 [Bazel](https://nativelink.com/docs/contribute/bazel/)、[Cargo](https://nativelink.com/docs/contribute/cargo/) 和 [Nix](https://nativelink.com/docs/contribute/nix/) 说明。

你可以在[文档](https://nativelink.com/docs/deployment-examples/kubernetes)中找到一些部署示例。

### 📦 预构建镜像

启动速度快，但目前仅支持 `x86_64` 系统。所有镜像 tag 请见[容器注册表](https://github.com/TraceMachina/nativelink/pkgs/container/nativelink)，自行构建镜像的方法请见[贡献文档](https://nativelink.com/docs/contribute/nix)。

**Linux x86_64**

```bash
curl -O \
    https://raw.githubusercontent.com/TraceMachina/nativelink/v1.4.0/nativelink-config/examples/basic_cas.json5

# See https://github.com/TraceMachina/nativelink/pkgs/container/nativelink
# to find the latest tag
docker run \
    -v $(pwd)/basic_cas.json5:/config \
    -p 50051:50051 \
    ghcr.io/tracemachina/nativelink:v1.4.0 \
    config
```

**Windows x86_64**

```powershell
# Download the configuration file
Invoke-WebRequest `
    -Uri "https://raw.githubusercontent.com/TraceMachina/nativelink/v1.4.0/nativelink-config/examples/basic_cas.json5" `
    -OutFile "basic_cas.json5"

# Run the Docker container
# Note: Adjust the path if the script is not run from the directory containing basic_cas.json
docker run `
    -v ${PWD}/basic_cas.json5:/config `
    -p 50051:50051 `
    ghcr.io/tracemachina/nativelink:v1.4.0 `
    config
```

### ❄️ 使用 Nix 运行原生可执行文件

速度较慢（因为从源码构建），但更灵活，且支持 MacOS。
不支持原生 Windows，但可在 WSL2 中运行。

请确保你的 Nix 版本较新并支持 flakes。例如，可以通过
[next-gen nix installer](https://github.com/NixOS/experimental-nix-installer) 安装。

> [!CAUTION]
> 为 MacOS 构建的可执行文件动态链接了来自 Nix 的库，
> 在没有这些库的系统上无法运行。

> [!TIP]
> **macOS / Linux 上常见的 Nix 安装问题：**
> * **Shell 环境未生效**：如果安装完成后 shell 无法识别 `nix` 命令，你需要重启终端会话，或手动 source daemon profile：
>   ```bash
>   . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
>   ```
> * **启用实验特性**：如果你使用标准 Nix 安装器并遇到 `experimental Nix feature 'nix-command' is disabled` 错误，请创建或编辑 `~/.config/nix/nix.conf`：
>   ```text
>   experimental-features = nix-command flakes
>   ```
> * **磁盘空间**：解压和构建编译器及其依赖需要大量存储空间。在运行安装命令之前，请确保系统卷至少有 **15–20 GB 可用空间**。

**Linux、MacOS、WSL2**

```bash
curl -O \
    https://raw.githubusercontent.com/TraceMachina/nativelink/main/nativelink-config/examples/basic_cas.json5

nix run github:TraceMachina/nativelink ./basic_cas.json5
```

更多信息请参见[贡献文档](https://nativelink.com/docs/contribute/nix)。

## 🤝 贡献

访问我们的[贡献指南](https://github.com/tracemachina/nativelink/blob/main/CONTRIBUTING.md)，了解如何为 NativeLink 做贡献。我们欢迎各种技能水平和背景的开发者参与贡献！

## 📊 统计

![Alt](https://repobeats.axiom.co/api/embed/d8bfc6d283632c060beaab1e69494c2f7774a548.svg "Repobeats analytics image")

## 📜 许可证

版权所有 2020–2026 Trace Machina, Inc.

基于 Functional Source License 1.1 版（未来转为 Apache 2.0 许可证）授权。
SPDX 标识符：`FSL-1.1-Apache-2.0`。

自本版本发布之日起满两周年后，你可以依据 Apache 许可证 2.0 版使用本软件。

---

> 本翻译由社区贡献。如发现翻译错误或与英文原文不一致之处，欢迎提交 PR 修正；如有歧义，以 [英文原文](README.md) 为准。
