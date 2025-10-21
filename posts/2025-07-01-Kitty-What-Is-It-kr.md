---
title: "Kitty? 그게 뭔데?"
date: 2025-07-01T01:43:26Z
tags: [linux, kitty, terminal, kr]
description: "kitty 터미널을 알아보자"
---

## Kitty?

![kitty](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/Kitty-Terminal-Emulator.png)

흔히 생각하는 키티(Kitty)는 캐릭터로 생각할 수 있다. 나 또한 그랬다. 그러다 이전에 컴퓨터 커스터마이징을 하던 중 터미널을 커스터마이징할 때 알게된 녀석이 Kitty. 쉽게 말해 맥북의 Iterm2 같은 녀석이다.

왜 다른 수많은 서드파티 터미널 프로그램중 Kitty 를 선택하였는가 하면:

1. GPU 가속 가능
2. "Kitten" 의 존재
3. 커스터마이징

이제 각각의 특징들을 살펴보도록 하자.

- GPU 가속 가능: 터미널에서 랜더링 할 때 CPU 대신 GPU를 사용하여 매우 빠르고 부드로운 스크롤링과 입력 반응 속도를 보여준다. 특히나 텍스트가 많거나 여러 창의 터미널을 띄울 때 체감이 많이 난다.
  물론 Kitty 뿐만 아니라 GPU 가속을 하는 터미널이 있다. (Alacritty, WezTerm 등)

- "Kitten": 이 녀석은 Kitty 내에 내장되어있는 작은 프로그램들이다. icat, diff, theme, Unicode input 등등  다양한 기능들이 탑재되어 있다. 간단한 예시로 이렇게 있다

이미지 kitten

- 커스터마이징: 참으로 놀라울 정도로 커스터마이징이 자유롭다. 기본적으로 폰트, 폰트 크기, 폰트 색상(명령어, url 등등 각각), 터미널 색상, 터미널 창 불투명도, 커서 모양, 테마, 단축키 등등 모든 기능들을 내 마음대로 각각 다르게 커스터마이징 할 수 있다는 점이 매우 뛰어나다. 특히나 linux distro 같은 경우에 window 나 mac 처럼 기본적으로 테마를 제공해주는게 부족하다 보니 터미널, 브라우저, 배경, 앱 등등 각기 다른 테마를 일체화 시킬 수 있기에 사용자 입장에서 마음대로 커스터마이징 할 수 있다는 장점이 매우 크다.

<br><br><br>

## 설치 방법

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/b10b9faad2a9c9efa7353a5d69cb7ab8.jpg" alt="install" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

설치는 매우 간단하다. 그저 명령어 하나면 끝난다.

```bash
// macOS
brew install kitty

Linux/WSL (Ubuntu/Debian)
sudo apt install kitty

Linux (Arch)
sudo pacman -S kitty
```

```bash
// 내 환경에서의 커마 세팅
// ~/.config/kitty/kitty.conf
```

커마 하는 방법은 밑에 링크에서 확인할 수 있다.

[kitty.conf setting](https://sw.kovidgoyal.net/kitty/conf/)

위의 링크에서 확인할 수 있듯이 터미널 세팅 주제에 매우 길고 매우 길고 매우 길다. 그러니 자신에게 맞는 기능만 쏙쏙 찾아서 하면 된다.

<br><br><br>

## 그래서 왜 Kitty를 골랐냐고

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/553irk7ruyt91.jpg" alt="why choose this" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

물론 위에서 언급했던 Alacritty, WezTerm 등 더욱 강력한 기능들을 가진 터미널들도 있는데 왜 kitty를 골랐냐고 한다면 "Kitten" 의 존재때문이다. 솔직히 철학적인 관점으로 랜더링 부분은 Alacritty, 커스터마이징 부분은 WezTerm 에 집중하고 있어서 그 부분에 있어서는 더욱 강력하다고 할 수 있다.

Kitty 는 All-in-one 느낌의 터미널이기에 나는 Kitty 를 선택하였다. (물론 세팅 초기에 선택하느라 좀 더 깊게 생각은 하지 않았다.) 그리고 현재로서 불편한 점은 일절 없고, 앞으로도 kitty 를 사용하겠지만 Alacritty 도 사용해보고 싶다.

오늘은 짧고 간단하게 글을 작성해보았다. 애초에 터미널 프로그램 선택에 있어서 장황하게 설명할 필요가 없이 자신이 좋아하는 터미널을 선택하는게 답이기에 그저 내가 현재 사용하는 터미널에 대해서 짤막하게 끄적였다.