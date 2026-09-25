---
title: "시스템 패키지 매니저 통합본"
date: 2025-09-06T21:26:56Z
tags: [linux, window, mac, system-package-manger, kr]
description: "MacOS, Window, Linux 패키지 매니저 통합본"
---

## 난 맥북, 난 리눅스, 난 윈도우

![linux window mac](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/linux-vs-mac-vs-windows1.jpg)

모든 OS들은 지원하는 시스템 패키지 매니저가 존재한다. 오늘은 수많은 시스템 패키지 매니저들 중 가장 유명한 매니저들을 말해보겠다.

<br><br><br>

## HomeBrew

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/homebrew-the-app-store-but-much-better-v0-16yc16ggu63f1.webp" alt="homebrew image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

MacOS를 사용하는 사람들은 친숙히 들어봤을 명령어이다. 이는 시스템 디렉토리를 건드리지 않는 선에서 `/opt/homebrew` (Apple Silicon) 혹은 `/usr/local` (Intel Mac) 같은 독립된 경로에 패키지를 설치한 후 필요한 파일만 심볼릭 링크로 연결하여 시스템과의 충돌을 최소화하는 방식을 사용한다.

또한 HomeBrew는 Apple이 기본적으로 제공하지 않는 수많은 개발 도구와 라이브러리를 쉽게 설치할 수 있게 해준다. 

> 즉, **원래는 애플에서 지원하지 않는 걸  커뮤니티를 만들어서 사용할 수 있게 끔 한다는 것**이다.

고로 다른 패키지 매니저들과는 다르게 homebrew 에 있다고 해서 "Mac 공식 지원" 이라는 말은 틀린 말이니까 주의해서 사용하자. (그저 Mac 지원)

```
// 패키지 목록 업데이트
brew update

// 패키지 검색
brew search <formula-name>

// 패키지 설치
brew install <formula-name>

// GUI 앱 설치
brew install --cask <cask-name>

// 패키지 제거
brew uninstall <formula-name>

// 모든 패키지 업그레이드
brew upgrade
```

<br><br><br>

## Chocolatey

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/8dvdv.jpg" alt="chocolatey image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

그저 "윈도우를 위한 패키지 매니저" 라고 해도 과언이 아니다. 리눅스 환경의 편리한 패키지 관리 경험을 윈도우로 가져오는 것을 목표로 하는 이 패키지 매니저는 PowerShell 스크립트를 기반으로 `.exe` 나 `.msi` 같은 윈도우 설치 파일들을 설치, 제거, 업데이트 등 할 수 있는 패키지 매니저이다.

```
// 패키지 검색
choco search <package-name>

// 패키지 설치
choco install <package-name>

// 패키지 제거
choco uninstall <package-name>

// 설치된 패키지 목록
choco list --local-only

// 업그레이드 가능한 패키지 목록
choco outdated

// 모든 패키지 업그레이드
choco upgrade all -y
```

<br><br><br>

## Scoop

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/a9cb3da7cf205140f987b04a3c5f7c9a.jpg" alt="scoop image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

Chocolatey랑 더불어 윈도우 시스템 패키지 매니저 중 하나이다. Chocolatey는 일반적으로 관리자 권한이 필요하며 시스템 전체에 설치되지만, Scoop은 사용자 수준 설치를 강조하며 일반적으로 관리자 권한이 필요하지 않다. 특히 패키지 내용물은 ZIP 파일을 사용하며, 독립 실행하는 애플리케이션에 중점을 둔다. 허나 Chocolatey보다 저장소가 작기 때문에 대부분 Chocolatey를 사용한다.

```
// 패키지 검색
scoop search <package-name>

// 패키지 설치
scoop install <package-name>

// 패키지 제거
scoop uninstall <package-name>

// 설치된 패키지 목록
scoop list

// 업그레이드 가능한 패키지 목록
scoop status

// 모든 패키지 업데이트 (업그레이드)
scoop update *
```

<br><br><br>

## apt

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/sudo-apt-install-meme-v0-47vy0btxbayd1.webp" alt="apt image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

만약 자신이 서버 관리자로서 ubuntu 를 사용거나 WSL을 사용해봤다면, 이것까지는 어느정도 들어봤을 것이다. apt 는 우분투, 리눅스 민트등 데비안 계열의 패키지 명령어이다. 자신이 개발자라면 이것까지는 알아놓으면 좋다.

```
// 패키지 목록 업데이트
sudo apt update

// 패키지 검색
apt search <package-name>

// 패키지 설치
sudo apt install <package-name>

// 패키지 제거 (혹은 설정 파일까지)
sudo apt remove <package-name>
sudo apt purge <package-name>

// 설치된 모든 패키지 업그레이드
sudo apt upgrade
```

<br><br><br>

## dnf

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/lsar3pxgfif81.jpg" alt="dnf image" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

생소한 시스템 패키지 매니저 일것이다. 이건 Linux 배포판 중 fedora 혹은 Red Hat 계열의 Distro를 사용하는 유저에게만 사용되는 패키지 명령어이다.

기존에 `yum` 이라는 걸 사용하였지만 개선된 의존성 해결 능력과 빠른 성능으로 업그레이드 된 패키지 매니저이다. 여전히 yum 을 사용할 수 있기는 하지만 dnf 가 표준으로 자리잡았다.

```
// 패키지 검색
dnf search <package-name>

// 패키지 설치
sudo dnf install <package-name>

// 패키지 제거
sudo dnf remove <package-name>

// 업데이드 가능한 패키지 확인
sudo dnf check-update

// 모든 패키지 업데이트 (업그레이드)
sudo dnf update
```

<br><br><br>

## pacman

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/p0lt49pm.jpg.webp" alt="pacman image" style="width: 100%; display: block; margin: 0 auto;">

<br>

내가 사용하는 운영체제에서 사용하는 시스템 패키지 매니저. 아치 리눅스, 만자로 등 Arch 계열에서 사용하는 패키지 명령어이다. 이름이 pacman 이지만 실제로는 package manager 의 줄임말이다.

아치 리눅스는 다른 운영체제들과 달리 릴리즈가 자주 나오지 않고 한 번에 크게 업데이트하는 대신 모든 패키지를 항상 최신 버전으로 유지하는 '롤링 릴리즈' 방식을 차용하여 `pacman -Syu` 명령어로 전체 시스템을 최신 상태로 업데이트 할 수 있다.

```
// 패키지 검색
pacman -Ss <package-name>

// 패키지 설치
sudo pacman -S <package-name>

// 패키지 제거 (혹은 의존성 까지 제거)
sudo pacman -R <package-name>
sudo pacman -Rns <package-name>

// 설치된 패키지 목록 확인
pacman -Q <package-name>

// 패키지 업그레이드
sudo pacman -Syu

// 패키지 강제 업그레이드
sudo pacman -Syyu
```