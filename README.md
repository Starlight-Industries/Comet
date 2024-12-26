<div align="center">

# Comet ☄️

### A Modern package manager for modern usecases

[Installation](#usage) • [Docs](#documentation) • [Roadmap](#roadmap) • [Community](#community)

[![Platform - Windows](https://img.shields.io/badge/platform-Windows-blue)](##)
[![Platform - Linux](https://img.shields.io/badge/platform-Linux-blue)](##)
[![Discord](https://img.shields.io/discord/1258146131372806217)](https://discord.gg/kv3jKuPW9F)
[![License](https://img.shields.io/badge/license-MIT-green)]( )

<p align="center">
  <sub>🚧 Early Alpha 🚧</sub>
</p>

</div>

---

## What is Comet?

Comet is a platform agnostic, automatically declarative package manager purpose built to simplify the needs of both developers and new users alike.
It tries to providing a versatile, fast, and capable alterntive to other monolthic

## Why?

Comet aims to provide:

* Automatic logging and managing of package configurations
* multi-source fetching (eg. git, self hosted repos, the comet Mono-repo)
* Easy integration for other managers, (see npm for example)
* Reduced reliance on complex package ecosystems (such as pip + conda)

## Quick Start

### Installation

> [!IMPORTANT]
> Comet is currently in early development. this means that the installation process is not yet stable. and must be done manually.
#### Method 1. Debug artifacts
0.

#### Method 2. Build from source


0. Install the rust toolchain 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

0. Clone the repository
```bash
git clone https://github.com/Starlight-Industries/Comet.git
```
1. Navigate to the project directory
```bash
cd Comet
```
2. Build & install the project
```bash
cargo install --path .
```

> [!NOTE]
> If you are on windows, you will need to add the cargo bin directory to your path.
> ```ps1
> setx PATH "%PATH%;%USERPROFILE%\.cargo\bin"
> ```

> [!TIP]
> if comet cannot be found make sure it is in your $PATH and restart your shell

#### ⚠️ ~~Method 3. Install script~~ (WIP)
> [!WARNING]
> This method is not yet stable, and may not work as expected.
> This method may or may not be available at the time of reading. (The script is a placeholder at the time of writing)

Simply run the following command to install the latest version of Comet.

```bash
curl -fsSL https://raw.githubusercontent.com/Starlight-Industries/Comet/main/install.sh | sh
```
### Usage

<div align="center">

**[WIP]** Check back later. Or you could [Contribute!]() 📚

</div>

## Roadmap

<div align="center">

**Current Status**: Project planing ❓

</div>

### 🎯 Basics

* [ ] Fully plan out package spec <- We are here :star:
* [ ] Basic package management system (install/remove).
* [ ] Core logging framework for package declarations.
* [ ] Dependency management with sandboxing.

### 🚀 Advanced

* [ ] Full-featured library backend.
* [ ] Cross-platform support Windows.
* [ ] Oficial GUI for easy package management.

### 🔮 Future Goals

* [ ] Environment variable configuration baked into the CLI.
* [ ] Cross-manager integration with npm, pip, and more.
* [ ] Community-driven plugin repository.
* [ ] MacOS Support.

### 🎨 Dream Features

* [ ] Web-based package browser (inspired by [Flathub](https://flathub.org/)).
* [ ] Intelligent version rollbacks.
* [ ] Bootstrap linux via Comet. ~~starlight-linux soon™~~

### 🌐 Platform Support
⭐ - The most support is avalible for this platform.

🔥 - This platform is currently supported with high priority.

⚠️ - The platform is planned to be supported in the future but is not currently supported for external reasons. (eg. MacOS requiring F)

| Platform | Status          | Priority      |
| :--------: | :-------------: |:----------: |
| Linux    | ✅ Supported   | ⭐ Main       |
| Windows  | 🔄 In progress | 🔥 High       |
| macOS    | 🔎 Planned     | ⚠️ High       |

> [!NOTE]
> We are always looking for new contributors to help us achieve our goals, so if you're interested and posess a targeted device, please reach out to us on [Discord](https://discord.com/invite/xJX4GXvbME) (Eg. MacOS 🍎).

## Community/Support
Here are the following links to all of our socials, if you discover a different account on any platform not listed here claiming to be affilated they are NOT affilated with the project, starlight-industries, or any of our related projects, products, or services.
<div align="center">

[![Discord Banner](https://img.shields.io/discord/1258146131372806217?style=for-the-badge&logo=discord)](https://discord.gg/xJX4GXvbME)

</div>

## FAQ


</details>

<details>
<summary><b>Can Comet replace system package managers?</b></summary>
No, not yet atleast. Comet is designed to complement system package managers by focusing on project-level and environment-specific package management, this is still subject to change though.
</details>

<details>
<summary><b>How can I contribute?</b></summary>
We welcome contributions! Check out the [Contributing Guide]() for details on how to get started. Whether it's fixing bugs, improving documentation, or building new features, we'd love your help!
</details>

<div align="center">

## Contributing

Thank you for considering contributing to Comet! We appreciate your interest in helping us improve the project. Comet is an open-source project and is made possible by the contributions of many individuals like You:

<a href="https://github.com/Starlight-Industries/Comet/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Starlight-Industries/Comet" />
</a>

For more information on how to contribute, please refer to the [Contributing Guide](CONTRIBUTING.md).

---

Made with ❤️ by Starlight-industries & the open source community
  <br>
  <sub>🌟 Star us on GitHub | 📢 Share with friends | 🤝 Join the community!</sub>
  </div>
</div>
