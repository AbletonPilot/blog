---
title: "Markdown Rendering Test"
date: 2025-10-14
tags: [test, markdown, syntax]
description: "Complete markdown syntax test with edge cases"
---

# Markdown Comprehensive Test

이 포스트는 모든 마크다운 문법과 엣지 케이스를 테스트하기 위한 문서입니다.

## 1. Headers (제목)

# H1 제목
## H2 제목
### H3 제목
#### H4 제목
##### H5 제목
###### H6 제목

---

## 2. Text Formatting (텍스트 서식)

일반 텍스트입니다.

**굵은 텍스트 (Bold)**

*기울임 텍스트 (Italic)*

***굵고 기울임 (Bold + Italic)***

~~취소선 (Strikethrough)~~

`인라인 코드`는 이렇게 표시됩니다.

여러 줄에 걸친  
텍스트는 두 칸 띄어쓰기로  
줄바꿈할 수 있습니다.

---

## 3. Lists (리스트)

### Unordered List (순서 없는 리스트)

* 첫 번째 항목
* 두 번째 항목
  * 중첩된 항목 2-1
  * 중첩된 항목 2-2
    * 더 깊은 중첩 2-2-1
* 세 번째 항목

### Ordered List (순서 있는 리스트)

1. 첫 번째 단계
2. 두 번째 단계
   1. 서브 단계 2-1
   2. 서브 단계 2-2
3. 세 번째 단계

### Mixed List (혼합 리스트)

1. 순서 있는 항목
   - 순서 없는 서브 항목
   - 또 다른 서브 항목
2. 다음 항목
   1. 순서 있는 서브 항목
   2. 또 다른 순서 항목

---

## 4. Links (링크)

[일반 링크](https://www.rust-lang.org/)

[타이틀이 있는 링크](https://www.rust-lang.org/ "Rust 공식 사이트")

<https://www.rust-lang.org/>

[레퍼런스 스타일 링크][1]

[1]: https://www.rust-lang.org/ "Rust"

---

## 5. Images (이미지)

### 외부 이미지

![대체 텍스트](https://via.placeholder.com/400x200 "이미지 제목")

### 로컬 이미지

![로컬 이미지 테스트](/2025-10-12/2025-10-14-1.png "로컬 이미지")

---

## 5.5 Videos (영상)

### 로컬 영상 (HTML video 태그)

<video controls style="width: 100%; height: auto;">
  <source src="/2025-10-12/2025-10-14-2.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

### 유튜브 영상 (iframe 임베드)

<div style="position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden; max-width: 100%;">
  <iframe 
    style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;" 
    src="https://www.youtube.com/embed/ScdlYSAhFoI?si=Bj2zVJq8rU6Y9QnB" 
    title="YouTube video player" 
    frameborder="0" 
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
    referrerpolicy="strict-origin-when-cross-origin" 
    allowfullscreen>
  </iframe>
</div>

---

## 6. Blockquotes (인용)

> 이것은 인용문입니다.
> 여러 줄에 걸쳐 작성할 수 있습니다.

> 중첩된 인용문:
>> 이렇게 중첩할 수 있습니다.
>>> 더 깊게도 가능합니다.

> **인용문 안에서도** *서식*을 사용할 수 있고 `코드`도 넣을 수 있습니다.

---

## 7. Code Blocks (코드 블록)

### Rust

```rust
// Rust 예제
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci(10) = {}", fibonacci(10));
}
```

### JavaScript

```javascript
// JavaScript 예제
function quickSort(arr) {
  if (arr.length <= 1) return arr;
  
  const pivot = arr[Math.floor(arr.length / 2)];
  const left = arr.filter(x => x < pivot);
  const middle = arr.filter(x => x === pivot);
  const right = arr.filter(x => x > pivot);
  
  return [...quickSort(left), ...middle, ...quickSort(right)];
}

console.log(quickSort([3, 6, 8, 10, 1, 2, 1]));
```

### Python

```python
# Python 예제
def merge_sort(arr):
    if len(arr) <= 1:
        return arr
    
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    
    return merge(left, right)

def merge(left, right):
    result = []
    i = j = 0
    
    while i < len(left) and j < len(right):
        if left[i] < right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    
    result.extend(left[i:])
    result.extend(right[j:])
    return result
```

### SQL

```sql
-- SQL 예제
SELECT 
    users.name,
    COUNT(posts.id) as post_count
FROM users
LEFT JOIN posts ON users.id = posts.user_id
WHERE users.created_at > '2024-01-01'
GROUP BY users.id, users.name
HAVING post_count > 5
ORDER BY post_count DESC
LIMIT 10;
```

### 언어 지정 없는 코드

```
이것은 언어가 지정되지 않은 코드 블록입니다.
특별한 하이라이팅이 없습니다.
```

---

## 8. Tables (표)

### 기본 표

| 이름 | 나이 | 직업 |
|------|------|------|
| 홍길동 | 30 | 개발자 |
| 김철수 | 25 | 디자이너 |
| 이영희 | 28 | 기획자 |

### 정렬이 있는 표

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| 왼쪽 정렬 | 중앙 정렬 | 오른쪽 정렬 |
| Left | Center | Right |
| L | C | R |

### 복잡한 표

| 언어 | 타입 | 특징 | 난이도 |
|------|------|------|--------|
| Rust | 컴파일 | 메모리 안전, 성능 | 높음 |
| Python | 인터프리터 | 간결한 문법 | 낮음 |
| JavaScript | 인터프리터 | 웹 표준 | 중간 |
| Go | 컴파일 | 동시성 | 중간 |

---

## 9. Horizontal Rules (수평선)

세 개 이상의 하이픈, 별표, 언더스코어로 만들 수 있습니다:

---

***

___

---

## 10. Nested and Complex Structures (중첩 구조)

1. **첫 번째 주제: 데이터 구조**
   
   데이터 구조는 프로그래밍의 기초입니다.
   
   - 배열 (Array)
     ```rust
     let arr = [1, 2, 3, 4, 5];
     ```
   
   - 벡터 (Vector)
     ```rust
     let mut vec = Vec::new();
     vec.push(1);
     ```

2. **두 번째 주제: 알고리즘**
   
   > 좋은 알고리즘은 시간 복잡도와 공간 복잡도를 모두 고려해야 합니다.
   
   | 알고리즘 | 시간 복잡도 | 공간 복잡도 |
   |---------|------------|------------|
   | 버블 정렬 | O(n²) | O(1) |
   | 퀵 정렬 | O(n log n) | O(log n) |
   | 병합 정렬 | O(n log n) | O(n) |

---

## 11. Special Characters (특수 문자)

특수 문자 이스케이프:

\* 별표
\_ 언더스코어
\# 해시
\[ 대괄호
\] 닫는 대괄호
\( 소괄호
\) 닫는 소괄호

---

## 12. Task Lists (체크리스트)

- [x] 완료된 작업
- [x] 또 다른 완료된 작업
- [ ] 진행 중인 작업
- [ ] 아직 시작 안 한 작업
  - [ ] 서브 작업 1
  - [x] 서브 작업 2 (완료)

---

## 13. Blockquote Styling (인용문 고급 활용)

> **💡 팁**: 인용문 안에 마크다운 문법을 사용할 수 있습니다.
>
> ```rust
> // 인용문 안의 코드
> fn example() {
>     println!("Hello!");
> }
> ```
>
> - 리스트도 가능
> - **굵게** *기울임*도 가능

---

## 14. Multiple Code Blocks (여러 코드 블록)

첫 번째 코드 블록:

```rust
// Rust
fn main() {
    println!("First");
}
```

두 번째 코드 블록 (다른 언어):

```python
# Python
def main():
    print("Second")
```

---

## 15. Direct Emojis (직접 이모지)

😀 💻 🚀 ⭐ 🎉 👍 ❤️ 🔥 📝 ✨

---

## 16. Edge Cases (엣지 케이스)

### 매우 긴 단어 (자동 줄바꿈 테스트)

Supercalifragilisticexpialidocious_Pneumonoultramicroscopicsilicovolcanoconiosis_Hippopotomonstrosesquippedaliophobia

이렇게 긴 단어는 자동으로 줄바꿈되어야 합니다.

### 긴 URL (자동 줄바꿈 테스트)

링크 문법을 사용하면 클릭 가능한 링크가 됩니다:

[https://www.verylongdomainname.com/very/long/path/to/resource?with=many&query=parameters](https://www.verylongdomainname.com/very/long/path/to/resource?with=many&query=parameters)

짧은 텍스트로 표시: [긴 링크 예시](https://www.verylongdomainname.com/very/long/path/to/resource?with=many&query=parameters&and=more&stuff=here)

### 연속된 백틱

일반 텍스트 `` 두 개의 백틱 `` 일반 텍스트

### 특수문자 조합

`code` **bold** *italic* ~~strike~~ 모두 함께!

---

## 결론

이 문서는 마크다운의 대부분의 문법과 엣지 케이스를 포함하고 있습니다. 
모든 요소가 올바르게 렌더링되는지 확인해보세요!

**테스트 항목:**
- ✅ 제목 (H1-H6)
- ✅ 텍스트 서식 (굵게, 기울임, 취소선)
- ✅ 리스트 (순서, 비순서, 중첩)
- ✅ 링크와 이미지
- ✅ 인용문
- ✅ 코드 블록 (여러 언어)
- ✅ 표
- ✅ 수평선
- ✅ 중첩 구조
- ✅ 특수 문자
- ✅ 엣지 케이스
