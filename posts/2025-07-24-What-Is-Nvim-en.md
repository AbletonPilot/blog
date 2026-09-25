---
title: "Why On Earth Do You Use NVIM?"
date: 2025-07-24T00:12:21Z
tags: [linux, nvim, neovim, lazyvim, vscode, ide, my-opinion, en]
description: "Once you get used to it, it's definitely the best! Let's learn about nvim"
---

## NVIM?

![nvim](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/neovim-screenshot-with-logo_hu_7e67f3a00595ec6.png)

It may be unfamiliar. The official name NeoVim can be seen as an upgraded version of Vim. If you're a developer, you might be familiar with the command vim along with nano and cat, which are commands used to open text files in the terminal.

##### Vim VS NeoVim(NVIM)

As mentioned above, I said it's an upgraded version, but it's not an official upgrade. (It would be more accurate to call it a fork relationship.) So what's been upgraded?

1. Asynchronous processing support: Unlike Vim, which stops until external commands or plugins finish executing, NeoVim supports asynchronous processing, allowing you to continue editing without lag or freezing by processing heavy tasks in the background.

2. Extensibility and scripting: Unlike Vim, which has low extensibility because plugins are made with VimScript, an unfamiliar and difficult-to-learn proprietary language, NeoVim introduced the Lua language, resulting in many plugins expanding in the community and a vibrant ecosystem.

3. Built-in LSP support: LSP is simply a feature that provides code auto-completion and error checking. This is a technology provided by modern IDEs. Vim doesn't have it by default and requires plugins, but NeoVim has an LSP client built in from the start, allowing it to become a modern IDE.

In table form:

| Category | Vim | NeoVim |
| ------ | -------------- | -------------- |
| Goal | Stability, Tradition, Portability | Modernity, Extensibility, Speed |
| Main Language | Vimscript | Lua, VimScript |
| Async Support | Not supported (editor freezes) | Fully supported (no freezing) |
| LSP Support | Possible with plugins | Built-in support |
| Development | Founder-centered | Community-centered (open source) |

<br><br><br>

## We Have VS Code, Isn't It Better to Just Use That?

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/hq720.jpg" alt="vim vs vscode" style="width: 100%; display: block; margin: 0 auto;">

<br>

That's a valid point. I also use VS Code. In fact, VS Code is even better for some features.

"But who are you to explain nvim?"

You might say that. Currently, I use VS Code for work-related tasks and NeoVim for personal projects, and I'm slowly transitioning to nvim.

So, what advantages does NeoVim have over VS Code:

- Overwhelming speed and lightness: It launches almost instantly and uses less memory, making it very snappy.
- Terminal-centric program: You just use `cd` to navigate around, and from the desired path, it opens in the terminal with just one nvim command. You can use the `code .` command, but there's no hassle of opening the program and closing the terminal. You can also return to the terminal immediately with `:q`.
- Customization: VS Code is a "product" provided by a company, so you use it within its framework. Of course, you can customize it to your liking through many extensions, but you move within a fundamental framework. However, NeoVim can be made into a one-of-a-kind tool in the world, customized 100% to my work style as I please.
- Text editing efficiency: Since it's a fork of Vim, no mouse is needed. If you know the shortcuts, you can do all tasks from the keyboard, minimizing hand movement to use the mouse. However, this is when you've mastered it to some extent.

To summarize, **VS Code is 'immediate convenience', NeoVim is 'overwhelming efficiency and complete control'**. But why do most developers use VS Code?

As explained above, nvim has many good points like performance, features, extensibility, customization, etc., but the biggest disadvantage among the reasons it's not widely known compared to VS Code is that **it's really ~~fucking~~ difficult**.

For settings or environment in Linux, MacBook, or Windows, if you don't know something, you can just look it up, but since NeoVim is an IDE to begin with, the process of searching for a feature during work and coming back to work is very inefficient, and you lose the reason for using NeoVim. And the fact that using settings or plugins in NeoVim requires writing all the code yourself and configuring shortcuts without conflicts to control NeoVim takes a lot of time.

The conclusion is **investing high initial learning costs (time) to obtain a powerful 'productivity asset' that can be used for a lifetime**. In other words, if you endure the initial phase, you get advantages over using VS Code, but the entry barrier is very high.

So, before directly using the editor, do you just have to bash your head, add, and create?

<br><br><br>

## LazyVim

![LazyVim](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/213447056-92290767-ea16-430c-8727-ce994c93e9cc.png)

Since this difficulty is something all developers around the world feel, there are starter packs with NeoVim equipped with only the necessary features from the start. Representative ones include AstroNvim, Kickstart nvim, LunarVim, LazyVim, etc., and I'm using LazyVim. Each has its own concept, but LazyVim is recommended for beginners, and since most neovim users actually use lazyvim, it's easy to find help if you have difficulties.

[LazyVim Start](https://www.lazyvim.org/)

When you install and run it, LazyVim automatically installs all dependency packages and you can use it right away. However, since LazyVim is ultimately NeoVim, you need to know many commands. 

And you can check downloads, updates, etc. in LazyUI with `:Lazy` or `<leader>l`, and you can add additional features with the `x` key on the first screen. (Additionally, it recommends additional features depending on the currently open file.)

Also, shortcuts for many plugins that serve as an IDE can be found at the link below, and if used well, it can serve as an IDE quite effectively.

[LazyVim KeyMaps](https://www.lazyvim.org/keymaps)

By the way, the leader key is the key to enter command mode, and the initially set key is the `space bar`. Of course, you can't know everything, but here's a brief overview of the most commonly used keys:

- `<Space>e`: Open/close file explorer (check hidden files: H)
- `<Space>bd`: Close current buffer
- `<Space>wd`: Close current window
- `<Space>-`: Split new window below
- `<Space>|`: Split new window to the right
- `[b`: Move to previous buffer
- `]b`: Move to next buffer

- `<Space>ff`: Search by file name (from root directory)
- `<Space>fF`: Search by file name (from current directory)
- `/`: Search by name in current file content (then next: n, prev: N)

- `<Space>fT`: Open terminal (from current directory)
- `<Space>ft`: Open terminal (from root directory)

If you don't know too well, just click the space bar and KeyMaps will appear, so you can check step by step and get used to it while executing.

By the way, the shortcuts above are separate from the existing NeoVim shortcuts, so you need to know NeoVim shortcuts as well.

<br><br><br>

## I'll Just Use VS Code...

![vi](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/visual.jpeg)

In a way, this could be a wise decision. Even I would recommend VS Code rather than vim if I had to recommend something.

The reason I also didn't immediately delete VS Code and switch to NeoVim is because of the very steep learning curve and numerous shortcuts. There are also some posts based on this.([here](https://officialrajdeepsingh.dev/posts/why-i-switched-to-neovim-from-vscode-and-how-did-you-configure-neovim)) But should you only use VS Code just because nvim is difficult?

NeoVim has clear advantages and disadvantages, and I think it's worth trying rather than cutting off an IDE that has only advantages once the disadvantages are secured just because it's 'difficult'. It's free, lightweight, and has the advantage of being usable anywhere as long as there's a terminal, so I'm trying to get used to it little by little.