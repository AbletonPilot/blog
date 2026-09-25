---
title: "TMUX 넌 누구냐"
date: 2025-08-08T22:54:04Z
tags: [linux, tmux, terminal, manager, kr]
description: "터미널 관리 패키지중 최고"
---

## TMUX 란?

![TMUX](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/tmux-always-feared-tmux-finally-managed-to-configure-it-to-v0-8fm7zwad6iac1.webp)
출처: [reddit](https://www.reddit.com/r/unixporn/comments/18yqmgk/tmux_always_feared_tmux_finally_managed_to/)

TMUX는 터미널 멀티플렉서(Terminal MUltipleXer)라는 어려운 말의 약자 이다.

그리고 쉽게 말하면 그냥 터미널 관리자 이다.

하는 일은 거창하지 않다. 주로 한개의 터미널을 분할해서 어려개의 터미널을 띄울 수 있다. 딱 요정도이다.

하지만 TMUX 로 인해 지저분한 화면 및 여러개의 터미널창을 띄우지 않게 해준다는 점 하나로 TMUX 는 개발자들 사이에서 찬양받아 마땅한 존재로 각인된다.

이제 실질적인 TMUX 에 대한 기능을 3개로 나누어 말해보자면

1. 세션 유지 및 복구
이게 tmux 에서 핵심이자 유명해지게 된 기능이다. 원격 서버. 즉, SSH 에 접속해서 작업하던 중 네트워크 연결이 끊겨도 **TMUX 세션 안에서 실행중이던 작업은 절대 멈추지 않는다.**(끄고 다시 키는 경우 제외) 그냥 나중에 다시 서버에 접속해서 tmux 세션에 다시 붙기만 하면 연결이 끊기기 직전의 화면과 상태 그대로 작업을 이어할 수 있다. 이 놀라운 기능으로 인해 오랜 시간동안 실행해야 하는 스크립트나 컴파일 작업을 안전하게 할 수 있다.

즉, ssh 에 접속해서 일하다가 노트북 덮어서 다시 집에와서 인터넷만 연결되도 그대로 진행이 된다는 것이다. (와우!)

2. 화면분할
딱히 설명할게 없이 그냥 하나의 터미널 안에서 분할해서 작업할 수 있는 기능이다. 이렇게 간단하고 직관적이며, 필수인 기능들만 갖추어져 있다는 점으로 TMUX 는 엄청 가벼운 프로그램중 하나이다. (약 1MB)

3. 여러 개의 작업 공간
이건 그냥 말 그대로 웹 브라우저처럼 여러개의 탭을 생성해줄 수 있다. 그로인해 탭 1번에 2개의 화면 분할, 탭 2번에 3개의 화면 분할 을 할 수 있어서 더욱 더 1개의 터미널로 작업 공간을 정리할 수 있다.

<br><br><br>

## 설치해보자

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/6601d126-7aab-41a7-b973-aafa79be72cf_text.gif" alt="let's get this started" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

내 리눅스 distro 는 Arch Linux 이기에 각각 다를수 있다는점 참고하자

Arch linux
```bash
sudo pacman -S tmux
```

MAC
```bash
brew install tmux
```

WINDOW(WSL)
```bash
sudo apt update
sudo apt install tmux
```

실행 명령어
```bash
tmux
```

<br><br><br>

## 유용한 단축키

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2397540.jpg" alt="where is the any key" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

기본 접두사(Prefix) 키 

- `Ctrl + b`: 가장 기본적인 명령 모드로 진입하는 키 이다. 이후 다른 단축키를 눌러 원하는 명령을 실행한다. 즉, 실질적인 단축키를 이용하기 위한 첫번째 단축키인 셈이다. `Ctrl + b` 를 누른 후에 키보드에서 손을 떼고 다음 명령어르 누르면 된다.

세션 및 창 관련 단축키

- `Ctrl + b` + `d`: 현재 세션을 분리한다 (Detach). 

- `Ctrl + b` + `c`: 새로운 창을 생성한다. 

- `Ctrl + b` + `w`: 창 목록을 보여준다. 

- `Ctrl + b` + `s`: 세션 목록을 보여준다. 

- `Ctrl + b` + `n` (next): 다음 창으로 이동한다. 

- `Ctrl + b` + `p` (previous): 이전 창으로 이동한다. 

- `Ctrl + b` + `[세션 이름]`: 특정 이름의 세션으로 이동한다. 

창(Panel) 분할 및 이동 단축키

- `Ctrl + b` + `%`: 세로로 새 창(패널)을 분할한다. (% 는 말 그대로 shift + 5)

- `Ctrl + b` + `"`: 가로로 새 창(패널)을 분할한다. 

- `Ctrl + b` + `방향키`: 현재 창(패널)에서 원하는 방향으로 이동한다. 

- `Ctrl + b` + `x`: 현재 창(패널)을 종료한다. 

- `exit`: 현재 창(패널)에서 `exit` 명령을 입력하여 종료할 수 있다. 

복사 및 붙여넣기 단축키 

- `Ctrl + b` + `[`: 스크롤 모드로 진입하여 복사할 범위를 선택한다.
- `Ctrl + b` + ` Spacebar ` (스페이스바): 스크롤 모드에서 복사를 시작하는 키 이다.
- `Ctrl + b` + `]` (닫는 대괄호): 복사한 내용을 붙여넣는다.

기타 유용한 단축키 

- `Ctrl + b` + `?`: Tmux의 모든 단축키를 보여주는 도움말 창을 연다.
- `Ctrl + b` + `k`: 현재 창을 닫고, 다른 창이 있으면 그 창으로 이동한다.

이 외에도 많은 단축키가 있지만 너무 많이 있어서 링크 남겨두도록 하겠다.

시작 가이드: [getting started guide](https://github.com/tmux/tmux/wiki/Getting-Started)

자세한 단축키 및 기능: [tmux(1) manual page](http://man.openbsd.org/OpenBSD-current/man1/tmux.1)

<br><br><br>

## 하지만...

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/jwdpkcecwr881.jpg" alt="stop scroll" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

만약 tmux 를 처음 사용하게 된다면 한가지 불편한 점을 확인할 수 있게 된다. 

바로 마우스 스크롤 기능이 없어서 위에 있는 복사 모드 단축키로 진입하여 화살표 키(↑, ↓) 나 Page Up/Down 키로 이동해야 한다는 점이다.

이는 참으로 불편하기 짝이 없는 기능이지만 물론 해결책은 있다. 바로 config 파일로 지정하게 해주는 것이다.

참고로 이 명령어는 linux 기준으로 말하는 것이며, 다른 운영체제는 이 파일을 어디에, 어떻게 만들어야되는지는 찾아봐야 할것이다. (MAC, WSL 대부분 비슷할 것이다.)

```bash
mkdir ~/.config/tmux
nvim ~/.config/tmux/tmux.conf

// 파일 안에 작성 후 저장 및 나가기
set -g mouse on

// 설정 적용시키기
tmux source-file ~/.config/tmux/tmux.conf 
```

이제 다시 tmux 에 들어가면 마우스 스크롤로 자유롭게 이동할 수 있게 된다.

고로 TMUX는 단축키 변경이나 theme등 자신에게 맞게 설정을 맞춰야만 빛을 내는 존재이기에 만약 사용하고자 한다면 설정을 변경하는 방법을 꼭 확인해보자.