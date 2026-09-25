---
title: "초경량 리눅스 Distro 일때"
date: 2025-05-29T02:03:01Z
tags: [linux, arch, hyprland, command, kr]
description: "초경량 리눅스를 사용할 때 초기에 알아두면 좋은 명령어 모음 (Arch Linux)"
---

## 모든것은 터미널로부터

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/EXurVZDXkAQdMXU.jpg" alt="linux users use command line" style="width: 100%; display: block; margin: 0 auto;">

<br>

대부분의 리눅스 distro 는 윈도우나 맥보다는 가벼운 OS로 출시된다. 하지만 그 중에서도 초 경량화된 distro는 **없어도 돌아갈 수 있다면 제거되어 배포** 되는게 있다. 예를들어 파일 매니저, 메뉴 바, 심지어는 와이파이나 블루투스 연결 기능도 없는게 있다. 왜냐하면 터미널로 전부 다 할 수 있는 작업들이기 때문이다. 화면캡쳐? 그런것 자체를 고급기능으로 분류하는지 우클릭 또한 없고, USB 연결을 해도 수동으로 터미널 들어가서 연결 커맨드를 작성해야지 USB 내용을 볼 수 있다.

> "이전에도 리눅스 관련한 포스트 올렸는데 재탕이냐?"

이번 포스트는 말 그대로 초 경량 리눅스였을 때 내가 겪었던 일들과 해결방법등을 풀어볼 생각이다. 그러면 이제 내가 실제로 처음 Hyprland와 맞닥드린 문제와 이후 해결해낸 것들 부터 설명해보도록 하겠다.

<br><br><br>

## 와이파이 연결

![linux wifi connect](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/DgtcfA7JDskE2NAXEMJ4SkN3fUMLXqO7UJxR3GXHuEA.webp)

제일 중요했던 와이파이 연결. 이게 일단 되야 뭐라도 찾아보고 설치할 수 있었기에 제일 우선순위였다. 휴대폰을 이용해서 검색 또 검색하여 알아낸 해결책이다.

iwctl: 이녀석은 리눅스에서 와이파이를 관리하기 위한 커맨드 라인 도구이다. 최소한의 양심은 있었는지 이건 설치가 되어있었다.

```
// iwctl 시작
iwctl

// 사용 가능한 wifi 장치 확인 (wlan0 확인)
device list

// 주변 와이파이 스캔
station wlan0 scan

// 스캔된 네트워크 목록 보기
station wlan0 get-networks

// 네트워크 연결
station wlan0 connect "와이파이 이름"

// 연결 상태 확인
station wlan0 show

// 종료
exit
```

이짓거리를 첫 한달동안 와이파이 연결할 때마다 했었다. (ㅠㅠ)

현재는 networkmanager(nmcli) 와 eww 를 사용해서 자동으로 찾고, 자동으로 연결해주는 상단 바를 만들어서 사용하고 있다.

<br><br><br>

## 위젯

와이파이 다음으로 와이파이 연결됬는지, 현재 볼륨이 어느정도인지, 현재 모니터 밝기가 어느정도인지, 베터리는 얼마나 남았는지 등 이러한 걸 확인하는 기본적인 위젯도 없다는게 충격이었다.

특히 볼륨이나 밝기 같은 경우에는 내가 시각, 청각으로 느낄 수 있기에 대충 감으로 확인은 할 수 있었지만, 베터리는 충전기 연결된 상태가 아닌 이상 기도메타밖에 답이 없었다.

그나마 다행인건 역시 터미널 명령어로 현재 상태를 확인할 수 있었다.

```bash
// 현재 몇퍼?
cat /sys/class/power_supply/BAT0/capacity
```

<br><br><br>

## 한글

![square error](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/thread-214057525-7251051804450518141.jpg)

이후 firefox로 구글을 들어갔을 때 나에게 반기는건 네모네모 로직이었다. 다른게 더 급했지만 찾는데에 문제가 되다 보니 폰트 교체할 겸 타이핑도 한글작업 할 수 있게 만들었다.

일단 폰트는 [nerd font](https://www.nerdfonts.com/font-downloads) 라는 곳에서 다운받을 수 있지만 pacman 으로 따로 설치하였다., 한글 타이핑을 위해서는 fcitx5 를 사용했다.

```bash
// JetBrains 폰트 설치
sudo pacman -S ttf-jetbrains-mono ttf-jetbrains-mono-nerd

// 폰트 설치 확인
fc-list | grep -i "JetBrains"

// 폰트 캐시 새로고침
fc-cache -fv

// (선택) 만약 폰트 우선순위를 정하고 싶다면
cd .config
mkdir fontconfig
cd fontconfig
touch fonts.conf

// 이후 fonts.conf 에 우선순위 작성 (예시)
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>emoji</family>
    <prefer>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>

  <alias>
    <family>monospace</family>
    <prefer>
      <family>JetBrainsMono Nerd Font</family>
    </prefer>
  </alias>

  <alias>
    <family>serif</family>
    <prefer>
      <family>JetBrainsMono Nerd Font</family>
    </prefer>
  </alias>

  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>JetBrainsMono Nerd Font</family>
    </prefer>
  </alias>
</fontconfig>
```

이후 한글 적용하기.

```bash
// 혹시 모르니 전부 필요한건 전부 다운받기
sudo pacman -S fcitx5 fcitx5-configtool fcitx5-hangul kcm-fcitx5

// gui 실행
fcitx5-configtool
```

![image](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-20-173909_hyprshot.png)

![image2](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-20-173949_hyprshot.png)

이후 Available Input Method 에서 Hangul 을 검색하여 왼쪽으로 넘기고, Global Options 에서 Right Alt 로 변경하여 오른쪽 알트키를 눌러서 한영 전환을 하게 하였다.

그리고 추가로 hyprland 에서는 hyprland.conf 로 컴퓨터를 켰을 때 fcitx5 가 자동으로 실행되도록 적용 코드를 작성 해야한다.

```
// hyprland.conf 열기
nvim .config/hypr/hyprland.conf

// auto start 부분에 추가
exec-once = fcitx5 -d

// environment variables 부분에 추가
env = QT_IM_MODULE,fcitx
env = XMODIFIERS,@im=fcitx
env = SDL_IM_MODULE,fcitx
env = GLFW_IM_MODULE,fcitx
```

물론 다른 배포판은 자동으로 적용하는 부분이 있을테니 찾아보고, 이런 경량화 배포판은 거의 비슷할것이기 때문에 코드 참고는 될 것이다.

<br><br><br>

## 외장 USB 연결

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/c5pk0vnpg3ty.webp" alt="usb" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

외장 하드 연결 및 해제 또한 내가 직접 제어해야한다. 열받는 점은 

```
// 연결 후 디스크 리스트 확인 (외장하드 sda1 확인)
lsblk

// 외장 하드를 위한 공간 만들기
mkdir /mnt/external

// 외장 하드를 위한 공간에 외장 하드 마운트 하기
sudo mount /dev/sda1 /mnt/external

// 연결 해제 하기 전에 언마운트 하기
sudo unmount /mnt/external
```

이제 연결하고 해제하기 전마다 `sudo mount /dev/sda1 /mnt/external`, `sudo umount /mnt/external` 를 하면 된다.

이 또한 첫 한달동안 이짓거리를 했었고, 현재는 thunar 라는 파일 매니저의 플러그인을 통해 자동으로 하게 만들었다.

참고로 저 `sudo unmount /mnt/external` 이 윈도우나 맥북에서 우클릭으로 "외장하드 안전하게 제거" 하는 거랑 똑같다고 보면 된다.

<br><br><br>

## 베터리 관리

![battery](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/qhvgnqd3khd61.webp)

이건 크게 복잡하지 않았다. "이게 뭐지?" 라고 생각할 수 있지만 흔히말해, 잠자기, cpu 부스트, gpu 부스트 등등 충전중일 때나 베터리 모드였을 때 각각 다른 성능을 제어할 수 있도록 하는 것이다.

즉, 이전에 베터리 상태를 확인하는 위젯과는 달리 전체적인 시스템 관리를 위한 프로그램이 필요하였다.

나는 TLP 를 사용하였다. TLP 는 설치하고 세팅만 해놓으면 그만이고, 베터리 수명 보호나 설정하는게 매우 포괄적이며, 가볍다는 장점이 있다. 하지만 다른 전원 관리 도구와 충돌할 수 있고, 설정이 복잡할 수 있으니까 주의해서 해야된다.

```
// tlp 설치
sudo pacman -S tlp tlp-rdw

// tlp 실행하기
sudo systemctl start tlp

// 컴퓨터 킬 때 자동으로 tlp 실행하기
sudo systemctl enable tlp.service

// 확인하기: enable, active 상태면 제대로 실행중이라는 뜻
systemctl status tlp
```

대부분 정상적으로 작동하겠지만, arch linux 처럼 경량화된 배포판이 아니라면 자동으로 설치되어 있는 전원관리도구와 충돌이 날 수 있으니 하나만 선택하는 결정을 내려야 한다.

또한 TLP 설정은 다음과 같다.

```
// tlp.conf 파일 에디터 실행 및 원하는 항목 변경
sudo vim /etc/tlp.conf
sudo nano /etc/tlp.conf

//ex

// 배터리 모드일 때는 CPU 파워 세이브 적용
#CPU_SCALING_GOVERNOR_ON_AC=powersave
CPU_SCALING_GOVERNOR_ON_BAT=powersave

// CPU 부스트 베터리모드에서 off
CPU_BOOST_ON_AC=1
CPU_BOOST_ON_BAT=0

// 70% 이하에서 충전 시작, 80% 되면 충전 멈춤
START_CHARGE_THRESH_BAT0=70
STOP_CHARGE_THRESH_BAT0=80

// 설정 저장 및 적용
sudo systemctl restart tlp

// 설정 확인
sudo tlp-stat -s
```

> 만약 TLP 말고 순정으로 변경하고 싶다면 이전에 베터리 상태 확인할 때 처럼 `/sys/class/power_supply/BAT0/`에서 파일들을 확인하고 수정할 수 있다.

<br><br><br>

## 외장 모니터 연결

![external monitor](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2fxbqc.jpg)

이것도 연결하거나 화면 비율 및 주사율 등등 모든걸 제어해야 내가 원하는 형태로 나온다. 다행히도 `hyprctl` 이라는 명령어로 쉽게 모니터 스펙을 알아볼 수 있었다.

```
// hyprland
hyprctl monitors

// arch linux
xrandr
```

이미지

이렇게 위에 결과를 보고 알 수 있듯이 픽셀과 주사율도 지정되어있는게 있기 때문에 맞춰서 해야된다. 그리고 나는 밑에 노트북 모니터, 위에 외장 모니터로 중앙에 맞춰서 위아래로 볼 수 있도록 배열을 만들었다.

```
// 차례대로 모니터 넘버, 픽셀@주사율, 위치, 배율
// 참고로 auto 기능도 있으니 찾아보자.
monitor = eDP-1, 1920x1200@60, 0x0, 1
monitor = DP-2, 2560x1440@60, -320x-1440, 1
```

참고로 다른 배포판은 잘 모르니까 linux 명령어로 확인하는 방법을 말해주도록 하겠다. 이건 연결 됬는지 안됬는지만 확인하는 명령어 이기 때문에 참고만 하도록 하자.

```bash
// 내 포트 확인하기
ls /sys/class/drm

// result
card1       card1-DP-2  card1-DP-4  card1-DP-6  card1-eDP-1     
card1-Writeback-1  version card1-DP-1  card1-DP-3  card1-DP-5
card1-DP-7  card1-HDMI-A-1  renderD128

// status 내용이 connected 라면 물리적으로 연결되어있는 뜻
// status 내용이 disconnected 라면 해당 포트에 아무것도 연결되어 있지 않다는 뜻
cat /sys/class/drm/card-DP-2/status
```

<br><br><br>

## 마치며

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/13dbbefb42f6790509f8234cebb6289e.gif" alt="end" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

솔직히 현 시점에서는 사용하지 않는 명령어들이 다수 존재한다. 허나 내가 처음 리눅스를 사용할 때 막막했었던 심정들과 명령어를 찾아보느라 고생했던 한달간의 여정이 고스란히 담겨 있다고 해도 과언이 아니다. 

만약 자신이 리눅스(특히 Arch)를 사용하면서 이러한 명령어가 필요한 순간이 있을 때 이 포스트가 도움이 되길 바란다.