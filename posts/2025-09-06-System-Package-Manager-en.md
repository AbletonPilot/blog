---
title: "System Package Manager Compilation"
date: 2025-09-06T21:26:56Z
tags: [linux, window, mac, system-package-manger, en]
description: "MacOS, Windows, Linux Package Manager Compilation"
---

## I'm MacBook, I'm Linux, I'm Windows

![linux window mac](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/linux-vs-mac-vs-windows1.jpg)

All OSes have supported system package managers. Today, I'll talk about the most famous managers among the many system package managers.

<br><br><br>

## HomeBrew

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/homebrew-the-app-store-but-much-better-v0-16yc16ggu63f1.webp" alt="homebrew image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

This is a command that people using MacOS will have heard of familiarly. It uses a method that minimizes conflicts with the system by installing packages in independent paths like `/opt/homebrew` (Apple Silicon) or `/usr/local` (Intel Mac) without touching the system directories, and then connecting only the necessary files with symbolic links.

Also, HomeBrew makes it easy to install many development tools and libraries that Apple doesn't provide by default.

> In other words, **it means creating a community to use things that Apple doesn't originally support**.

Therefore, unlike other package managers, saying "Mac official support" just because it's in homebrew is wrong, so be careful when using it. (It's just Mac support)

```
// Update package list
brew update

// Search package
brew search <formula-name>

// Install package
brew install <formula-name>

// Install GUI app
brew install --cask <cask-name>

// Remove package
brew uninstall <formula-name>

// Upgrade all packages
brew upgrade
```

<br><br><br>

## Chocolatey

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/8dvdv.jpg" alt="chocolatey image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

It's no exaggeration to say it's just "a package manager for Windows". This package manager, which aims to bring the convenient package management experience of Linux environments to Windows, is based on PowerShell scripts and can install, remove, and update Windows installation files like `.exe` or `.msi`.

```
// Search package
choco search <package-name>

// Install package
choco install <package-name>

// Remove package
choco uninstall <package-name>

// List installed packages
choco list --local-only

// List upgradable packages
choco outdated

// Upgrade all packages
choco upgrade all -y
```

<br><br><br>

## Scoop

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/a9cb3da7cf205140f987b04a3c5f7c9a.jpg" alt="scoop image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

It's one of the Windows system package managers along with Chocolatey. While Chocolatey typically requires administrator privileges and is installed system-wide, Scoop emphasizes user-level installation and typically doesn't require administrator privileges. In particular, it uses ZIP files for package contents and focuses on standalone applications. However, since it has a smaller repository than Chocolatey, most people use Chocolatey.

```
// Search package
scoop search <package-name>

// Install package
scoop install <package-name>

// Remove package
scoop uninstall <package-name>

// List installed packages
scoop list

// List upgradable packages
scoop status

// Update (upgrade) all packages
scoop update *
```

<br><br><br>

## apt

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/sudo-apt-install-meme-v0-47vy0btxbayd1.webp" alt="apt image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

If you've used Ubuntu as a server administrator or used WSL, you've probably heard of this to some extent. apt is a package command for Debian-based distributions such as Ubuntu and Linux Mint. If you're a developer, it's good to know up to this point.

```
// Update package list
sudo apt update

// Search package
apt search <package-name>

// Install package
sudo apt install <package-name>

// Remove package (or including config files)
sudo apt remove <package-name>
sudo apt purge <package-name>

// Upgrade all installed packages
sudo apt upgrade
```

<br><br><br>

## dnf

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/lsar3pxgfif81.jpg" alt="dnf image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

This might be an unfamiliar system package manager. This is a package command used only by users using Fedora or Red Hat-based Distros among Linux distributions.

Previously, they used something called `yum`, but it's an upgraded package manager with improved dependency resolution capabilities and faster performance. While yum can still be used, dnf has become the standard.

```
// Search package
dnf search <package-name>

// Install package
sudo dnf install <package-name>

// Remove package
sudo dnf remove <package-name>

// Check updatable packages
sudo dnf check-update

// Update (upgrade) all packages
sudo dnf update
```

<br><br><br>

## pacman

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/p0lt49pm.jpg.webp" alt="pacman image" style="width: 100%; display: block; margin: 0 auto;">

<br>

The system package manager used in the operating system I use. It's a package command used in Arch-based systems like Arch Linux and Manjaro. Although the name is pacman, it's actually an abbreviation for package manager.

Unlike other operating systems, Arch Linux doesn't release frequently or update in large chunks; instead, it adopts a 'rolling release' method that keeps all packages at the latest version, allowing you to update the entire system to the latest state with the `pacman -Syu` command.

```
// Search package
pacman -Ss <package-name>

// Install package
sudo pacman -S <package-name>

// Remove package (or remove including dependencies)
sudo pacman -R <package-name>
sudo pacman -Rns <package-name>

// Check installed package list
pacman -Q <package-name>

// Upgrade packages
sudo pacman -Syu

// Force upgrade packages
sudo pacman -Syyu
```