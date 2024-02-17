# Reverse String

Exercism의 Rust 트랙에 있는 Reverse String에 오신 것을 환영합니다.
테스트를 실행하거나 코드를 제출하는 데 도움이 필요한 경우 'HELP.md'을 확인하세요.
## Introduction

문자열을 뒤집는 것(왼쪽에서 오른쪽이 아닌 오른쪽에서 왼쪽으로 읽기)은 프로그래밍에서 놀라울 정도로 일반적인 작업입니다.
예를 들어, 생물정보학에서 DNA 또는 RNA 문자열의 염기서열을 반전시키는 것은 상보적 가닥을 찾거나 생물학적으로 중요한 회문 염기서열을 식별하는 것과 같은 다양한 분석에 중요한 경우가 많습니다.
## Instructions

당신의 임무는 주어진 문자열을 뒤집는 것입니다.
Some examples:

- Turn `"stressed"` into `"desserts"`.
- Turn `"strops"` into `"sports"`.
- Turn `"racecar"` into `"racecar"`.

## Bonus

이 문자열 'uüu'에서 함수를 테스트하고 어떤 일이 일어나는지 확인하십시오. 함수를 제대로 작성하십시오.
이 문자열을 반대로 합니다. 힌트: grapheme 클러스터
보너스 테스트를 실행하려면, 마지막 테스트를 수행하고 다음을 사용하여 테스트를 실행합니다.
```bash
$ cargo test --features grapheme
```

보너스 작업을 위해 외부 라이브러리(rust 용어로 '상자')를 사용해야 합니다. 그것들을 찾기에 좋은 곳은 [crates.io](https://crates.io/), 상자의 공식 저장소입니다.
[문서 확인] (https://doc.rust-lang.org/cargo/guide/dependencies.html) 프로젝트에서 외부 상자를 사용하는 방법에 대한 지침입니다.
## Source

### Created by

- @coriolinus

### Contributed to by

- @cbzehner
- @ccouzens
- @cwhakes
- @efx
- @ErikSchierboom
- @hunger
- @lutostag
- @ocstl
- @PaulT89
- @petertseng
- @rofrol
- @rrredface
- @stringparser
- @TheDarkula
- @xakon
- @ZapAnton

### Based on

Introductory challenge to reverse an input string - https://medium.freecodecamp.org/how-to-reverse-a-string-in-javascript-in-3-different-ways-75e4763c68cb