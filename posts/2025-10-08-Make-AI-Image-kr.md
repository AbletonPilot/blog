---
title: "외장 그래픽 없이 AI 이미지를 만들어보자!"
date: 2025-10-08T01:11:12Z
tags: [linux, ai, text-to-image, rocm, comfyui, kr]
description: "외장 그래픽 없이 AI 이미지 만드는 방법 A to Z 두번째 이야기"
---

## 어떻게 만들지?
<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/it-aint-stupid-if-it-works.jpg" alt="stupid computer" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

### **컴퓨터 = 멍청이**

이 공식은 코딩으로 개발하는 사람들에게는 알고 있는 공식이다. 그리고 AI에게 명령하는 것도 이 공식이 적용된다.

예를들어 AI 로 이미지를 그리고 싶을 때 "양 그리고 싶어!" 라고 하였을 때 실사의 양인지, 이전에 열풍이었던 지브리 느낌의 양인지, 
만화 느낌의 양인지, 애니메이션 양인지, 판타지 느낌의 양인지 컴퓨터는 모른다. 근데 모르면 물어봐야되는데 물어보지 않고 지 멋대로 
그림을 그리게 되니 내가 원하던 결과물에서 벗어나게 된다. 왜 물어보지 않는가? 하면 내가 "모르겠으면 물어봐라" 라는 명령을 안했기 때문인것이다.

그렇기 텍스트가 아닌 이미지를 생성해야 하는 단계에서 컴퓨터에게 설명해야 하는 것들이 참으로 많다. 하지만 한 모델에 전부 명령어를 작성하기에는 
당연하게도 만들기 힘든건 사실이다. 그렇기 때문에 각 파트별도 명령을 내리는 부분이 존재한다. 어떤건 포즈만, 어떤건 배경이나 사물만, 어떤건 그림체만 등등 
부분 별로 명령을 내리는 곳이 있는데 간략하게 알아보도록 하자.

<br><br><br>

## 사전 지식
<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/86prx80oj6t61.webp" alt="hard study" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

솔직히 전부 알아야 하거나 심지어 지금 말하려는 것들도 몰라도 되는 것들이지만 알고 있어야 내가 커스텀하며 가지고 놀기 쉬워지니 참고만 할 정도로 작성하겠다.

필수적으로 필요한 것들.
- `checkpoints`: 메인 모델 파일이다. 쉽게말해서 화가인 셈이다.(침착맨, 미야자키 하야오 등)
- `loras`: 메인은 아니지만 메인에게 "이 스타일로 그려줘" 라는 느낌이다. 쉽게말해서 화풍인 셈이다.(침착맨 혹은 미야자키 하야오 느낌의 이미지 스타일)
- `embeddings`: 그림에서 부가적인 사물이나 인물의 스타일이다. 예를들어 바다 그려줘 라고 하였을 때 ai 는 embeddings 를 확인하고 그에 적혀져 있는 바다를 보여준다
- `controlnet`: 이건 포즈, 구도, 깊이 등 밑그림 느낌을 제어해주는 역할을 한다. 예를들어 무릎 꿇어있는 사람을 그려달라 할 때 controlnet에 있는 무릎꿇는 스캐치를 가져와서 그려준다. (정자세, 발가락을 눕혔는지, 비스듬히 꿇었는지 등 controlent에 적혀있는대로 스케치 한다)
- `upscale_models`: 저해상도 이미지를 고해상도로 확대하고 디테일을 추가하는 모델이다.
- `vae`: 이미지의 색감과 최종 디테일을 담당해주는 모델이다.

실질적으로는 이렇게 필요하고 나머지 것들은 부가적인 기능을 해주는것들이다.



허나 사람들이 이렇게 나뉘어진걸 못마땅해 했는지 그냥 checkpoints 에 다 때려박은 모델들이 있다. 그게 바로 Stable Diffusion XL 이나 FLUX 를 기반으로 커스텀한 모델들이다. 기존 모델들 보다 용량도 적고 더욱 자유롭게 이미지를 생성할 수 있기에 이를 기반으로 만들어 볼까 한다.

그러한 모델들이 모인 커뮤니티가 대표적으로 civitai 이다. 이제 모델을 다운받으러 아래에 링크로 들어가보자.

civitai 홈페이지: [civitai](https://civitai.com/)

<br><br><br>


## 뭐 만들지?

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/unnamed.jpg" alt="thinking" style="width: 100%; max-width: 500px; display: block; margin: 0 auto;">

<br>

그렇다. 막상 "AI로 이미지 만들어봐야지!" 라고 생각하였을 때 '무엇' 이라는 게 고민이 되기 쉽상이다. 그럴 땐 위의 사이트에 들어가서 image 탭을 눌러보도록 하자.

![civitai images](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-221523_hyprshot.png)

그러면 위와 같은 이미지 들이 있을 것이다. 이것들은 커뮤니티에 자신이 만든 이미지를 업로드 하여 사람들에게 보여주는 이미지들이다. 이 중에 자신이 마음에 드는 이미지를 클릭해보자.

![choice image](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-221938_hyprshot.png)

이 이미지를 보았을 때 알아낼 수 있는 정보로는

- checkpoints 하나만 있어도 ai 이미지를 만들 수 있다.
- 밑에 프롬프트가 두 종류가 있다. (부정: negative, 긍정: positive)
- 메타 데이터를 입력하는게 있다. (cfgScale, steps, sampler 등등)

이 세가지만 알아도 AI 이미지를 만들 수 있는것이다. 이걸 토대로 하나 만들어 볼까한다.

![choice image](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-222202_hyprshot.png)

몇 시간 서칭해본 결과 WAI-illustrious-SDXL 라는 체크포인트가 많다는걸 알게 되었다. 그러면 이거 하나로 만드는걸 해보도록 하겠다. 오른쪽에 있는 WAI-illustrious-SDXL 라는걸 클릭해보자.

![WAI-Illustrious-SDXL page](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-222406_hyprshot.png)

클릭하면 현재 버전에 대한 디테일과 설명이 나온다. 오른쪽 다운로드 모양의 아이콘이나 오른쪽 밑에 `Pruned Model fp16 (6.46 GB)` 옆에 다운로드를 클릭하여 다운받아보자. 그리고 이전에 다운받은 ComfyUI 디렉토리에서 `models/checkpoints` 로 다운받은 모델을 넣는다.

![show more](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-222715_hyprshot.png)

그리고 밑에 `Show More`을 클릭하여 내려보면 15버전을 사용할 때 프롬프트나 메타데이터 추천 방식이 나와있다. 그리고 `Character Select Saa - a Hugging Face Space by flagrantia`를 클릭하면 허깅페이스로 연결된 사이트가 나온다.

![flagrantia](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-224010_hyprshot.png)

보니까 이 WAI-illustrious-SDXL 라는 모델의 특징은 프롬프트에 정해진 캐릭터 이름을 작성하면 그 캐릭터의 모습이 나오게 설계된 것 같다. 즉, **애니나 2D 캐릭터의 얼굴이 나올 수 있도록 하는 프롬프트 집약체 모델**이라고 생각하면 된다. 그리고 이 사이트는 프롬프트에 적을 수 있는 캐릭터를 시각적으로 찾을 수 있게 해주는 프로그램인것 같다. (참고로 12버전까지만 업데이트 된 것 같으니 14버전까지 확인하려면 github로 배포하고있는걸 설치해서 실행할 수 있다. [여기](https://github.com/mirabarukaso/character_select_stand_alone_app))

![choice charactor](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-223707_hyprshot.png)

대충 아무런 캐릭터를 선택하면 이렇게 밑에 프롬프트로 나오는걸 알 수 있게 된다. 이걸 프롬프트에 적으면 이미지처럼 나오게 된다는 뜻인가 보다.

그리고 좀 더 밑에 보면 추천하는 조절식이 있다. 이제 진짜 만들어보자!

<br><br><br>

## ComfyUI 이미지 생성

![start command](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-224206_hyprshot.png)

일단 recap 해보자.

```bash
// ComfyUI 위치로 이동
cd ~/Documents/github/ComfyUI

// venv 활성화
source venv/bin/activate

// 자신에 맞는 버전 오버라이드 및 실행
HSA_OVERRIDE_GFX_VERSION=11.5.1 python main.py

// ComfyUI 열기
http://127.0.0.1:8188
```

![checkpoint](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-225543_hyprshot.png)

첫번째로 우리는 왼쪽 사이드바에서 모델을 클릭하고 체크포인트를 열어서 다운받았던 모델을 열거나, `우클릭 -> Add Node -> loaders -> Load Checkpoint`으로 체크포인트 노드를 생성해보자. 그러면 화면과 같이 나올것이다.

![strech](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-225549_hyprshot.png)

옆에 보면 MODEL, CLIP, VAE 라고 나오는걸 확인할 수 있다. 이제 아무거나 클릭하고 드래그 해보면 주욱 늘어나고, 놓으면 또 노드를 생성할 수 있다. 이런식으로 생성을 할텐데 전부 다 하면 너어무 길어지니 텍스트로 상세하게 작성해주도록 하겠다. (필수적으로 필요한 것들만 사용하였다.)

1. CLIP Test Encode (Prompt): 제목에서 보이듯 프롬프트를 작성하는 곳이다. 이건 두개 만들어서 Positive, Negative 를 만들어야 한다. 나는 [여기](https://civitai.com/images/106435106) 에 있는 프롬프트를 전부 복사 붙여넣기 하고 캐릭터만 적용되도록 바꿨다. (최대한 WAI-illustrious-SDXL 모델 하나만 사용한 이미지를 레퍼런스 하였다.)

2. KSampler: 제일 중요한 부분이다. 이 값들을 조절함에 따라서 결과가 많이 달라진다. 
   - seed: 이미지의 고유번호이다. 마인크래프트 시드와 같다. 따로 설정할 필요없이 그대로 넘기면 된다.
   - control_after_generate: 한마디로 이미지 한 장을 만든 후 다음 시드 값을 어떻게 할지 정하는 방식이다. 그냥 랜덤으로 해놓으면 된다. 그래야 생성 후 시드값이 무작위로 변해서 다양한 이미지를 계속 뽑을 수 있다.
   - steps: 이미지를 얼마나 정교가ㅔ 다듬을지 정하는 값이다. 값이 높을 수록 디테일이 좀 더 생겨지긴 하나 낮게 할 때가 더 좋은 이미지를 생성할 수 있으니 너무 높게할 필요 없다.
   - cfg: 한마디로 AI가 프롬프트를 얼마나 엄격하게 따를지 정하는 값이다. 이 값이 낮을 수록 프롬프트를 무시하여 지 멋대로 이미지를 만들고, 값이 높을 수록 프롬프트를 엄격하게 지켜지게 되지만 과하면 이미지가 부자연스러워질 수 있다.
   - sampler_name: 노이즈를 제거하는 방식 또는 알고리즘을 어떤걸 선택할건지 고르는 곳이다. euler가 빠르고 단순하게 결과값이 나오고 공홈에서 추천하는 방식이라 선택했다. (dpmpp_2m 같은건 고품질 샘플러)
   - scheduler: step을 하면서 노이즈를 얼마나 빠르게 줄여나갈지 정하는 계획표 같은 친구이다. 
   - denoise: 기존 이미지를 얼마나 무시하고 새로 그릴지 정하는 강도이다. 이 값에 따라서 기존 모델이 프롬프트를 기반으로 만들어지는지, 아니면 스타일이나 구도 같은것만 유지되는지, 혹은 그냥 디테일이나 스타일만 살짝 변경되는지 등 많이 달라지므로 제일 중요하다.

1. Empty Latent Image: 이미지 크기를 정하는 값이다. 자신이 원하는 값으로 하면 된다.

2. VAE Decode: 만들고 따로 할 필요 없다. WAI-illustrious-SDXL 페이지 상세보기 에서도 "이미 적용되어있으니까 그만좀 물어봐라" 라고 되있으니 연결만 하자.

3. Save Image: 결과 이미지를 보여주고 저장하게 해주는 노드이다. 그냥 만들어만 놓으면 된다.

나는 WAI-illustrious-SDXL 페이지에 나온 추천값을 사용하였다.

![workflow](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-17-235917_hyprshot.png)

위의 환경을 전부 적용한다면 이렇게 된다. 혹시 모르니 잘 보이도록 배열을 하였으니 보고 참고하도록 하자. 그리고 `Ctrl + S`를 누르면 이 상태 그대로 저장을 할 수 있고, 왼쪽 사이드바에 workflow로 불러올 수 있다.

그리고 RUN을 실행한다면

![result](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-001704_hyprshot.png)

짜잔~

결과가 이렇게 나온다. 외장그래픽 없이 생성하는 것만으로도 만족하는데 꽤나 만족스러운 이미지가 나오니 괜찮은 장난감이 생긴거 같다.

저장은 따로 할 필요 없이 ComfyUI 경로에서 `output`폴더로 자동 저장이 되므로 여러 이미지를 생성하고 나중에 확인하면 된다. 그리고 step를 다른 값을 주면서도 만들어봤다.

### step 20
![step20](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-002209_hyprshot.png)

### step 25
![step25](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-002635_hyprshot.png)

### step 30
![step30](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-002904_hyprshot.png)

각각 결과가 다르게 나오면서 약간의 디테일 부분이 변화되는걸 볼 수 있다.


![time](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-002922_hyprshot.png)

그리고 step을 늘릴 수록 GPU 사용이 늘어나서인지 결과가 나올 때 까지의 시간이 늘어나는걸 확인할 수 있다. 위에서부터 15, 20, 25, 30 이다.

![btop result](https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/2025-10-18-011933_hyprshot.png)

또한 `btop`을 확인해본 결과 VRAM 은 약 12GB 정도 사용하는것을 확인하였으니 자신의 VRAM 용량을 확인하고 만들어보도록 하자.

<br><br><br>

## 만약에

<img src="https://pub-9fab8c462d8d4428bf45385586df9f1a.r2.dev/pepe-the-frog-error-meme-custom-cursor.png" alt="pepe error" style="width: 100%; display: block; margin: 0 auto;">

<br>

만약 위와 똑같이 만들었는데도 에러가 발생하게 된다면 내가 이전에 겪었던 에러와 같은 에러일것이다.

1. GPU 가 호환이 안되거나
2. HSA_OVERRIDE_GFX_VERSION 값을 자신의 GPU와 다르게 했거나

이러한 에러가 발생한다면 자신의 GPU에 맞는 버전을 잘 확인하도록 하자.

그리고 마지막으로 내가 받은 모델이 검열 없이 나오는 모델이기 때문에 프롬프트를 잘 작성해야 모두가 볼 수 있는 AI 이미지가 나올 것이다. (조심)