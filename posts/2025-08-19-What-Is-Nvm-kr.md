---
title: "NVM 은 뭐지?"
date: 2025-08-19T23:22:42Z
tags: [nvm, nodejs, package-manger, kr]
description: "Node.js의 관리 프로그램, nvm!"
---

## NVM 이란?

![nvm](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/1_Ft8vscKkg4KZkbMqP9Uy0w.jpg)

NVM(Node Version Manager) 즉 Node.js 의 여러 버전을 관리하기 위한 도구이다. 자신의 Node.js 가 프로젝트에서 요구하는 버전이랑 다른 경우 의존성 패키지 설치가 안되거나, 서드파티 라이브러리와의 호환성 문제, 코드 실행 중 오류 등의 문제들이 발생할 수있다.

그렇다고 Node.js 자체를 여러버전 설치하게 된다면 전역 패키지 충돌이 발생할 수 있기 때문에 nvm을 통해서 서로 격리하는 관리를 통해 충돌없이 여러 버전의 Node.js를 설치할 수 있다. 고로 자신은 Node.js 하나만 설치하고 싶다 하더라도 nvm을 통해서 설치하는게 추후에 유리하게 작용된다.

<br><br><br>

## nvm 설치

![nvm install](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/install-nvm.png)

[nvm github](https://github.com/nvm-sh/nvm)

위의 링크는 nvm 공식 github 사이트이다. 설치 버전이 다를 수 있기 때문에 항상 링크로 확인하도록하자.

```bash
// curl nvm 설치 명령어 (Linux, MacOS, Windows)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

// wget (Linux)
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
```

물론 맥북 같은 경우에는 homebrew 를 통해서 만들 수 있지만 curl 설치하는걸 추천한다.

이후 자신의 쉘(bash, zsh)에 맞춰서 설정 파일 끝에 작성하면 된다. (~/.bashrc, ~/.bash_profile, ~/.zshrc, ~/.profile)

```bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
```

혹은 그냥 명령어 한줄로 추가하고 싶으면 `echo` 명령어로 추가하면 된다.

```bash
// 마지막에 자신의 쉘에 맞춰서 넣으면 된다.
echo 'export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.zshrc

// 이후 터미널 새로고침 (자신의 쉘에 맞게)
source ~/.bashrc

// 확인하기
nvm -v
```

만약 자신의 쉘이 fish 일 경우 설치 방법이 다를 수 있기 때문에 따로 확인해야된다.

<br><br><br>

## Node.js 설치

![nodejs](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2400%D1%851260-rw-blog-node-js.png)

이후 nvm 을 통해서 node.js 를 설치하면 된다. npm 은 자동으로 설치된다.

```
// 설치 가능한 node.js 버전들 확인
nvm ls-remote

// 설치 가능한 특정 node.js 버전들 확인
nvm ls-remote 23.*

// 그냥 제일 최신 버전 설치
nvm install node

// 최신 릴리즈 설치
nvm install --lts

// 원하는 버전 설치
nvm install 23.8.0

// 원하는 버전에서 가장 최신 버전 설치
nvm install 23
```

만약 자신이 nvm 으로 18, 23 버전을 설치하였으면 다음의 명령어도 숙지해놓으면 좋다.

```
// 설치된 Node.js 확인하기
nvm ls

// 현재 23 에서 18버전 사용하기
nvm use 18

// 기본값 설정하기
nvm alias default 18
```

```
// 마지막 확인하기
npm -v
10.9.2

node -v
v23.8.0

nvm -v
0.39.7
```

<br><br><br>

## 마치며

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/IMG_2090.GIF" alt="end" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

처음 Node.js 를 사용할 때 회사에서 사용하는 프로젝트의 버전이 달라서 한참 찾아 해멨던 경험이 있었다. 그 시절을 회고하며 이번 포스트를 만들었다.