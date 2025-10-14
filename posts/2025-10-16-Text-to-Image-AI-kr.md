---
title: "외장 그래픽 없이 AI 이미지를 만들 수 있다고?! (kr)"
date: 2025-10-16
tags: [linux, ai, text-to-image, rocm, kr]
description: "외장 그래픽 없이 AI 이미지 만드는 방법 A to Z"
---

## 오! 드디어 나왔군!

<img src="https://preview.redd.it/just-a-low-quality-meme-v0-t8kn2pa8v4zc1.png?width=1080&crop=smart&auto=webp&s=016c9c615c05be16b0706a2c50f9580a44d4cb2b" alt="Thinkpad = CHAD" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

이번년도 8월 초 즈음에 노트북 썸머 이벤트로 thinkpad가 할인하길래 '이왕 이렇게 된거 리눅스로 갈아타야겠다'는 마인드로 구매하고 
세팅만 1달을 하며 지냈었다. (아치리눅스, hyprland는 진짜...)

할인하여 구매하였지만 약 3000AUD 라는 거금이 들었기에 이 노트북의 하나하나 모든걸 사용해서 뽕을 뽑겠다는 생각에 이것저것 설치해보고, 
커스텀하며 

그러면서 '이것저것' 안에 AI를 내 컴퓨터에 설치하는 방법도 찾아보고 어떻게든 적용해보려고 하였지만 결론이 아직 내 컴퓨터의 사양을 지원하지 
않는다는 결론으로 좌절하며 업데이트를 기다리고 있었다. 그도 그럴게 구매한 노트북에는 외장그래픽이 탑재되있지 않았기 때문이다.

그러다 이번주 화요일날 여느때와 같이 1주일에 2-3번 내 PC에서 진행하는 yay 으로 패키지들 업데이트 하는 도중 눈에 띄는 변경점이 발견되었다.

```
rocm-core 6.4.4-1
rocm-cmake 6.4.4-1
rocm-hip-sdk 6.4.4-1
....
```

오..! 드디어 rocm 버전업이 떴다..!

>**ROCm**: AMD 자사에서 GPU 프로그래밍하기 위해서 개발된 오픈소스 소프트웨어. 쉽게말해서 
>AMD GPU를 어떻게든 활용하도록 하는 소프트웨어.


하지만 버전이 업데이이트 되었다고 내 사양이 적용되지 않았을 수도 있기에 큰 기대는 하지 않으며 
릴리즈 노트를 확인했는데...

![ROCm 6.4.4 Release Notes](/2025-10-16/2025-10-14-1.png "ROCm 6.4.4 Release Notes")

야호! 드디어 내 노트북으로 AI 이미지 만들수 있게 됬다! (참고로 저는 Ryzen AI 9 HX 370)

<br><br><br>

## 이전엔 뭐가 문제였는데?

<img src="https://thunderdungeon.com/wp-content/uploads/2025/06/coding-memes-19-6-9-2025-600x497.jpg" alt="That doesn't work for me!" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

님들이나 미래의 내가 궁금한건 아니니 짧게 말하자면

pytoch에서 원하는 버전이 있는데 그 버전 안에 내 컴퓨터 사양이 없었다. 이게 끝이다.

![PyTorch requires](/2025-10-16/2025-10-14-2.png "PyTorch requires")

위의 사진에서 볼 수 있듯이 Linux, pip, python, ROCm(AMD GPU) 을 사용하는 사람이라면 
rocm 6.4 를 설치하라고 되어있으며, 그 6.4버전 안에 해당 GPU 가 있어야된다. 하지만 이전에는 
내 사양의 GPU가 없었기 때문에 아무리 하더라도 컴퓨터는 "? 이게 뭐임?" 하고 안된다고 하던거였다.

<br><br><br>

## 좋아, 이제 AI 이미지를 만들어볼까!

AI 이미지를 만들기 이전에 준비해야될 것들을 차근차근 해보도록 하겠다.

진행은 내 컴퓨터 사양 위주로 진행하겠지만, 최대한 범용적으로 작성하여 필요하신 분들은 자신의 사양, 
OOO하는 법 을 잼미니(gemini) 나 GPT에게 물어보거나 구글검색(추천) 을 해서 알아보도록 하자.

