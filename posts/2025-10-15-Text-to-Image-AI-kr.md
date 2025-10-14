---
title: "외장 그래픽 없이 AI 이미지를 만들 수 있다고?! (kr)"
date: 2025-10-15
tags: [linux, ai, text-to-image, rocm, comfyui, kr]
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

```bash
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

![ROCm 6.4.4 Release Notes](/2025-10-15/2025-10-14-1.png "ROCm 6.4.4 Release Notes")

야호! 드디어 내 노트북으로 AI 이미지 만들수 있게 됬다! (참고로 저는 Ryzen AI 9 HX 370)

<br><br><br>

## 이전엔 뭐가 문제였는데?

<img src="https://thunderdungeon.com/wp-content/uploads/2025/06/coding-memes-19-6-9-2025-600x497.jpg" alt="That doesn't work for me!" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

님들이나 미래의 내가 궁금한건 아니니 짧게 말하자면

pytoch에서 원하는 버전이 있는데 그 버전 안에 내 컴퓨터 사양이 없었다. 이게 끝이다.

![PyTorch requires](/2025-10-15/2025-10-14-2.png "PyTorch requires")

위의 사진에서 볼 수 있듯이 Linux, pip, python, ROCm(AMD GPU) 을 사용하는 사람이라면 
rocm 6.4 를 설치하라고 되어있으며, 그 6.4버전 안에 해당 GPU 가 있어야된다. 하지만 이전에는 
내 사양의 GPU가 없었기 때문에 아무리 하더라도 컴퓨터는 "? 이게 뭐임?" 하고 안된다고 하던거였다.

<br><br><br>

## 설치 가이드를 위해 다시 설치부터 생각해보자!

<img src="https://en.meming.world/images/en/6/62/Ah_Shit%2C_Here_We_Go_Again.jpg" alt="here we go" style="width: 100%; display: block; margin: 0 auto;">

AI 이미지를 만들기 이전에 준비해야될 것들을 차근차근 해보도록 하겠다.

진행은 내 컴퓨터 사양 위주로 진행하겠지만, 최대한 범용적으로 작성하여 필요하신 분들은 자신의 사양과 함께 
"OOO하는 법" 을 잼미니(gemini) 나 GPT에게 물어보거나 구글검색(추천) 을 해서 알아보도록 하자. 그리고 만약 구글 검색을 하게 된다면 reddit 내용을 
먼저 확인하는 것을 강추한다. (나에겐 도움이 많이 됐다.)

> 만약 reddit 내용이 영어라서 알아보기 힘들다 하시면 해당 링크 맨 뒤에 `?tl=kr` 을 붙이면 kr 로 변환되는 포스트도 있으니 확인해보자!

<br>

### 1. VRAM 설정

이건 확실히 모르겠다.

모른다는 말이 모든 컴퓨터에서 VRAM 할당을 변경 할 수 있는 건지, 아니면 내 컴퓨터만 가능한건지 혹은 thinkpad 만 가능한거지를 모르겠다. 
나는 BIOS 환경에 들어가서 VRAM 을 변경하였다. 내 노트북은 VRAM 을 48GB까지 설정할 수 있었기에 가능하긴 하였지만, 다른 컴퓨터는 자신의 CPU, 
노트북 혹은 컴퓨터 회사 제품이 가능한지 찾아봐야 할것이다. 이게 필수는 아니기에 자신의 VRAM 이 높다면 상관없다.

> **VRAM 과 RAM**: VRAM 은 비디오 램이며 그래픽 연산에서 사용하는 메모리이다. 그러다보니 기존에 흔히 알고 있는 RAM 보다 연산 속도가 월등히 빠르다. 
> 내 컴퓨터는 RAM 을 VRAM 으로 할당해주는 방식이기에 RAM 이 높으면 변경할 수 있었다.
>
> 그리고 RAM 을 VRAM 으로 할당해준다 하더라도 속도는 외장그래픽 처럼 나오진 않기 때문에 VRAM 많이 있다고 빨라지거나 그런건 아니고 원활히 돌아갈 수 있게만 해준다는 점 참고하자.

<br>

BIOS 에 들어가서 캡쳐하는 걸 모르기에 어떻게 들어가서 설정했는지 설명만 하도록 하겠다.

```
BIOS 진입 후 왼쪽 사이드 바에서 configuration 진입.

UMA Frame Buffer Size 선택창을 클릭하여 원하는 VRAM 할당해주기.
```

나는 램이 64GB 라서 32GB 정도 할당해주었다. VRAM이 최소 12GB 이상이기만 해도 돌아가긴 하지만 나중에 다른 AI 모델을 위해 넉넉히 할당해주었다.

```bash
// VRAM check (linux)
sudo dmesg | grep VRAM

// result
[    1.765377] amdgpu 0000:c4:00.0: amdgpu: VRAM: 32768M 0x0000008000000000 - 0x00000087FFFFFFFF (32768M used)
[    1.765402] [drm] Detected VRAM RAM=32768M, BAR=32768M
[    1.766517] amdgpu 0000:c4:00.0: amdgpu: amdgpu: 32768M of VRAM memory ready
```
```
// MAC
왼쪽 상단에 애플 아이콘 누르고 About This Mac -> More Info... -> System Report... -> 그래픽/디스플레이(Graphics/Displays) -> VRAM (총량) 항목 확인

// WINDOW
Ctrl + Shift + Esc 로 작업관리자 -> 성능 탭 -> GPU 선택 -> 전용 GPU 메모리(VRAM) 확인
```

<br>

### 2. ENV 세팅

다음으로는 환경세팅해야되는데 이건 리눅스만 해당한다 (ㅠㅠ)

```bash
// setting
sudo gpasswd -a $USER render

sudo gpasswd -a $USER video

// check
groups
id

//result
❯ groups
yourname video render

❯ id
uid=1000(yourname) gid=1000(yourname) groups=1000(logan),985(video),989(render)
```
이건 진짜 리눅스만 해당한다. 기본적으로 MAC/WINDOW 는 컴퓨터를 키면 자동으로 잡기 때문이다. 만약 안된다면 
재부팅 해보길 권장한다.

<br>

### 3. HIP/OpenCL 패키지 설치

이것 또한 linux 만 해당... (ㅠㅠ)

HIP은 CUDA 코드를 최소한의 수정으로 AMD GPU에서도 실행할 수 있도록 실행해 주는것이다. 그러니 만약 자신의 그래픽 카드가 NVIDIA 라면 CUDA를 설치해야 될 것이다.

참고로 리눅스 배포판(distro) 마다 설치 명령어가 다르기 때문에 찾아보길 바란다.

```bash
sudo pacman -S rocm-hip-sdk rocm-opencl-sdk
```

또한 python 도 설치가 되있어야 한다. 기본적으로 mac 혹은 윈도우는 설치되어있을것이며, 아니라고 한다면 공식 사이트에서 최신 버전 설치하면 된다. 만약 있음에도 안된다 하면 버전차이일 수 있다.
```bash
//install python and pip
sudo pacman -S python python-pip

//check version
❯ python --version
Python 3.13.7

❯ pip --version
pip 25.2 from /usr/lib/python3.13/site-packages/pip (python 3.13)
```

MAC은 `XCode` 만 설치하면 된다. 그리고 WINDOW는 번거롭지만 "AMD ROCm for Windows" 또는 "AMD HIP SDK for Windows" 등으로 검색하여 
AMD 공식 개발자 사이트로 들어가서 최신 버전의 설치 프로그램을 다운받고 설치 프로그램 지시에 따라서 설치하면 된다.

<br>

### 4. ComfyUI 설치 (Manual Install)

드디어 이미지 생성 프로그램을 설치한다. 내가 ComfyUI 를 선택한 이유로는 

- 메타몽 마냥 유연함: 실제로 사용해보면 상당한 제어를 내 맘대로 할 수 있음.
- 성능 좋고 효율 좋음: VRAM 을 많이 사용한다는건 막말로 사이버펑크 2077 초고사양으로 3~5개 돌리는 거랑 비슷하다. 하지만 ComfyUI로 원하는 작업만 돌릴 수 있기 때문에 그나마 덜어낼 수 있다.
- 쉬움: GUI 형식이라 시각적으로 작업할 수 있기 때문에 한번 보고 알아볼 수 있다.

<br>

아래의 링크를 들어가서 밑으로 좀 내려보면 데스크탑 설치 링크 및 수동 설치 가이드가 있다.

[ComfyUI Github](https://github.com/comfyanonymous/ComfyUI)

수동 설치 방식은 설명과 함께 진행할 거니까 필요한 부분만 쏙쏙 골라서 보도록 하자. (설명을 이해하시길 권장드립니다.)


```bash
// 터미널로 원하는 폴더쪽으로 가서 수동 설치 git

git clone https://github.com/comfyanonymous/ComfyUI.git

// ComfyUI 폴더로 이동

cd ComfyUI
```

```bash
// venv 가상환경 만들기: 파이썬 로컬 패키지 설치. 쉽게 말해서 창고 하나 만들기

python -m venv venv

// venv 활성화하기: 이건 터미널 종료 후 나중에 다시 할 때 다시 activate 해줘야 한다. 쉽게 말해서 창고 문 열기

source venv/bin/activate

// 이 부분이 업데이트 되고나서 사용할 수 있게 된 중요한 부분이다. 자신에게 맞는 걸 설치하면된다. 위 ComfyUI 깃허브 밑에 적혀있으며, 내 사양에는 이걸 설치하라고 명시되어있다.
// 이건 쉽게 말해서 창고에 넣을 물건들

pip install --pre torch torchvision torchaudio --index-url https://rocm.nightlies.amd.com/v2/gfx1151/

// 다른 의존성 설치: 위에 명령어와 똑같이 설치하는 명령어이다. 위의 경우는 하드웨어에 따라서 설치 방식이 다르기 때문에 지정해서 설치하는 것일 뿐 이건 그냥 나머지 설치이다.

pip install -r requirements.txt
```

### 5. 실행하기

실행하는 것도 차이가 있을 수 있다. 나처럼 실질적인 ROCm 을 설치하는게 아닌 약간의 우회방식으로 설치하는 경우 실행 명령어도 다를 수 있다.

위의 명령어를 보면 gfx1151 라는 게 있다. 이걸 설치하였기 때문에 컴퓨터에게 "내 그래픽에 맞는 버전은 gfx1151 이거야!" 라고 말을 해줘야 하기 때문에 오버라이드를 해야한다.

참고로 위의 깃허브에 있는 가이드에도 내 사양에 맞는 커맨더가 나와있지 않기 때문에 실제 실행하는데 1시간 삽질했다. (ㅠㅠ)

```bash
// gfx1151 이기 때문에 11.5.1
HSA_OVERRIDE_GFX_VERSION=11.5.1 python main.py

// 만약 VRAM 이 간당간당 하거나 실행했을 때 너무 버벅인거 같다! 하면
HSA_OVERRIDE_GFX_VERSION=11.5.1 python main.py --lowvram
```
### 6. 실행 결과

![result](/2025-10-15/2025-10-14-4.png "result")

이건 당연하게 실행은 잘 되었다.

왜 "당연하게" 라고 말하는거지? 라고 생각할 수 있는데 실제로 버전이 안 맞을 때도 실행은 잘 됐었다. 허나 실행을 하면 무수한 에러와 함께 브라우저가 멈추고 터미널도 난리났었다.

그리고 이제 저 127.0.0.1:8188 사이트로 들어가면 

![comfyui gui](/2025-10-15/2025-10-14-5.png "comfyui gui")

이제 이 위에서 AI 이미지를 생성하고 저장할 수 있다. 심지어 오프라인으로! 이걸로 내 심심했던 일상에서 즐길거리가 하나 추가 되었다 ㅎㅅㅎ

![need to active venv](/2025-10-15/2025-10-14-3.png "need to active venv")

그리고 이건 위에서 말했듯 venv 를 activate 하지 않았을 때 나오는 에러다. 당황하지 말고 위에 있는 명령어를 사용하고 실행하면 된다.

<br><br><br>

## 그래서 결과물은?

<img src="https://i.pinimg.com/originals/2c/3c/e2/2c3ce2285740ac78b6c2d49c62c0cc67.gif" alt="This is not an excuse" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

너무 길어졌기 때문에 다음 포스트에 실제로 어떤 모델을 사용하는지, 어떻게 만드는지, 어떤결과가 나오는지 작성해보려고 한다. 의외로 간단하면서 결과물이 예상보다 빠르게 나오니 다들 할 수 있도록 차근차근 설명해보도록 하겠다.

<br><br>

만약 궁금한 사항이나 지적이 필요한 부분이 있다면 자유롭게 댓글 달아주면 피드백 받도록 하겠다. :)