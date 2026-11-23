[English](README.md) | [简体中文](README.zh-CN.md) | **繁體中文**

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

## NativeLink 是什麼？

NativeLink 是一套高效、高效能的建置快取（cache）與遠端執行系統，能在加速軟體編譯與測試的同時降低基礎設施成本。它透過智慧化快取建置產物並將任務分派到多台機器，為各種規模的專案最佳化建置流程。當機器人／自駕車、半導體、生物運算，或 Chromium 等級／行動端 monorepo 需要共享快取與分散式執行時，NativeLink 能讓未變更的 target 在筆記型電腦與 CI 之間永遠不必重新編譯。

NativeLink 在生產環境中備受信賴，能降低成本並縮短開發者迭代時間——每月為客戶處理超過**數十億次請求**，客戶包括 **Samsung** 等大型企業。

NativeLink 特別擅長：

- Chromium 等級與行動端 monorepo（C++/ObjC/Swift/Kotlin/Java），擁有繁重的 CI 建置圖
- 基於 Bazel/Buck 的機器人與自駕車 C++/CUDA 技術堆疊（感知、規劃、模擬、硬體在環測試）
- 半導體流程（SystemVerilog/Verilog RTL、C/C++ 韌體、與 EDA 相關的 Bazel/Buck 工具鏈）
- C++/CUDA 與 Python 原生管線的生物運算（生物資訊學、影像重建、生物模擬）

<p align="center">
  <a href="https://www.youtube.com/watch?v=WLpqFuyLMUQ">
      <img src="https://trace-github-resources.s3.us-east-2.amazonaws.com/harper-90-thumbnail.webp" alt="NativeLink Explained in 90 seconds" loading="lazy" width="480" />
  </a>
</p>

## 🤖 給 AI 代理與智慧協作者

NativeLink 的設計既能被人閱讀，也能被代理閱讀：

- 儲存庫根目錄下的 [`llms.txt`](llms.txt)、[`llms-small.txt`](llms-small.txt) 和
  [`llms-full.txt`](llms-full.txt) 分別以三種規模收錄了網站、部落格和文件；
  它們就是 [nativelink.com/llms.txt](https://nativelink.com/llms.txt) 上提供的檔案。
  文件站點也有自己的三個檔案：[docs.nativelink.com/llms.txt](https://docs.nativelink.com/llms.txt)，
  根目錄的副本已將其合併進來。
- [`AGENTS.md`](AGENTS.md) 是供代理修改程式碼時使用的程式碼地圖。
- [`.claude/skills/`](.claude/skills) 提供可安裝的技能，包括
  [`migrate-to-bazelmod`](.claude/skills/migrate-to-bazelmod)。
- [nativelink.com](https://nativelink.com) 和文件的每個頁面都帶有
  「Ask AI」區塊，可以在 ChatGPT、Claude、Perplexity、Google AI Mode 或 Copilot 中開啟該站點。
- 歡迎藉助代理進行貢獻，但需要由真人為變更負責；
  詳見 [`CONTRIBUTING.md`](CONTRIBUTING.md#ai-assisted-contributions)。

完整列表見 [nativelink.com/agents](https://nativelink.com/agents)。

## 🔑 核心功能

1. **先進的建置快取**：
   - 儲存並重用未變更元件的歷史建置結果
   - 顯著縮短建置時間，尤其是增量變更時

2. **高效的遠端執行**：
   - 將建置與測試任務分派到機器網路中
   - 平行化工作負載，加快完成速度
   - 利用遠端資源，將運算負擔從本機卸載
   - 透過統一、受控的建置環境確保一致性

NativeLink 可與使用 Remote Execution 協定的建置工具無縫整合，例如 [Bazel](https://bazel.build)、[Buck2](https://buck2.build)、[Goma](https://chromium.googlesource.com/infra/goma/client/) 和 [Siso](https://chromium.googlesource.com/build/+/refs/heads/main/siso/README.md)。CMake 專案也可以透過 [`recc`](https://buildgrid.gitlab.io/recc) 接入，參見[使用 NativeLink 建置 CMake 專案](https://docs.nativelink.com/getting-started/connect-your-build/cmake-recc)。它支援 Unix 系作業系統和 Windows，確保在不同開發環境中的廣泛相容性。

## 🚀 快速上手

你可以像下面這樣以 Docker 映像部署 NativeLink。以下設定均為**生產級**安裝。如需從原始碼建置，請參考貢獻文件中的 [Bazel](https://nativelink.com/docs/contribute/bazel/)、[Cargo](https://nativelink.com/docs/contribute/cargo/) 和 [Nix](https://nativelink.com/docs/contribute/nix/) 說明。

你可以在[文件](https://nativelink.com/docs/deployment-examples/kubernetes)中找到一些部署範例。

### 📦 預建映像

啟動速度快，但目前僅支援 `x86_64` 系統。所有映像 tag 請見[容器註冊表](https://github.com/TraceMachina/nativelink/pkgs/container/nativelink)，自行建置映像的方法請見[貢獻文件](https://nativelink.com/docs/contribute/nix)。

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

### ❄️ 使用 Nix 執行原生可執行檔

速度較慢（因為從原始碼建置），但更有彈性，且支援 MacOS。
不支援原生 Windows，但可在 WSL2 中執行。

請確保你的 Nix 版本夠新並支援 flakes。例如，可以透過
[next-gen nix installer](https://github.com/NixOS/experimental-nix-installer) 安裝。

> [!CAUTION]
> 為 MacOS 建置的可執行檔動態連結了來自 Nix 的函式庫，
> 在沒有這些函式庫的系統上無法執行。

> [!TIP]
> **macOS / Linux 上常見的 Nix 安裝問題：**
> * **Shell 環境未生效**：如果安裝完成後 shell 無法識別 `nix` 指令，你需要重新啟動終端機工作階段，或手動 source daemon profile：
>   ```bash
>   . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
>   ```
> * **啟用實驗功能**：如果你使用標準 Nix 安裝器並遇到 `experimental Nix feature 'nix-command' is disabled` 錯誤，請建立或編輯 `~/.config/nix/nix.conf`：
>   ```text
>   experimental-features = nix-command flakes
>   ```
> * **磁碟空間**：解壓縮和建置編譯器及其相依套件需要大量儲存空間。在執行安裝指令之前，請確保系統磁區至少有 **15–20 GB 可用空間**。

**Linux、MacOS、WSL2**

```bash
curl -O \
    https://raw.githubusercontent.com/TraceMachina/nativelink/main/nativelink-config/examples/basic_cas.json5

nix run github:TraceMachina/nativelink ./basic_cas.json5
```

更多資訊請參見[貢獻文件](https://nativelink.com/docs/contribute/nix)。

## 🤝 貢獻

造訪我們的[貢獻指南](https://github.com/tracemachina/nativelink/blob/main/CONTRIBUTING.md)，了解如何為 NativeLink 做出貢獻。我們歡迎各種技能水準和背景的開發者參與貢獻！

## 📊 統計

![Alt](https://repobeats.axiom.co/api/embed/d8bfc6d283632c060beaab1e69494c2f7774a548.svg "Repobeats analytics image")

## 📜 授權條款

版權所有 2020–2026 Trace Machina, Inc.

基於 Functional Source License 1.1 版（未來轉為 Apache 2.0 授權條款）授權。
SPDX 識別碼：`FSL-1.1-Apache-2.0`。

自本版本發布之日起滿兩週年後，你可以依據 Apache 授權條款 2.0 版使用本軟體。

---

> 本翻譯由社群貢獻。如發現翻譯錯誤或與英文原文不一致之處，歡迎提交 PR 修正；如有歧義，以 [英文原文](README.md) 為準。
