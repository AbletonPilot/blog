---
title: "What Is NVM?"
date: 2025-08-19T23:39:55Z
tags: [nvm, nodejs, package-manger, en]
description: "Node.js management program, nvm!"
---

## What Is NVM?

![nvm](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/1_Ft8vscKkg4KZkbMqP9Uy0w.jpg)

NVM (Node Version Manager), in other words, is a tool for managing multiple versions of Node.js. If your Node.js is different from the version required by the project, problems such as dependency package installation failures, compatibility issues with third-party libraries, and errors during code execution can occur.

However, if you install multiple versions of Node.js itself, global package conflicts can occur, so through nvm's isolated management, you can install multiple versions of Node.js without conflicts. Therefore, even if you want to install only one Node.js, installing through nvm works advantageously in the future.

<br><br><br>

## nvm Installation

![nvm install](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/install-nvm.png)

[nvm github](https://github.com/nvm-sh/nvm)

The link above is the official nvm github site. Always check with the link because the installation version may differ.

```bash
// curl nvm installation command (Linux, MacOS, Windows)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

// wget (Linux)
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
```

Of course, in the case of MacBook, you can create it through homebrew, but I recommend installing with curl.

Then write it at the end of the configuration file according to your shell (bash, zsh). (~/.bashrc, ~/.bash_profile, ~/.zshrc, ~/.profile)

```bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
```

Or if you just want to add it in one command line, you can add it with the `echo` command.

```bash
// At the end, just put it according to your shell.
echo 'export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.zshrc

// Then refresh terminal (according to your shell)
source ~/.bashrc

// Check
nvm -v
```

If your shell is fish, the installation method may be different, so you need to check separately.

<br><br><br>

## Node.js Installation

![nodejs](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2400%D1%851260-rw-blog-node-js.png)

Then install node.js through nvm. npm is automatically installed.

```
// Check installable node.js versions
nvm ls-remote

// Check specific installable node.js versions
nvm ls-remote 23.*

// Just install the latest version
nvm install node

// Install latest release
nvm install --lts

// Install desired version
nvm install 23.8.0

// Install the latest version of the desired version
nvm install 23
```

If you installed versions 18 and 23 with nvm, it's good to know the following commands as well.

```
// Check installed Node.js
nvm ls

// Use version 18 from current 23
nvm use 18

// Set default value
nvm alias default 18
```

```
// Final check
npm -v
10.9.2

node -v
v23.8.0

nvm -v
0.39.7
```

<br><br><br>

## Conclusion

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/IMG_2090.GIF" alt="end" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

When I first used Node.js, I had an experience of struggling for a long time because the version of the project used at the company was different. I made this post reminiscing about that time.