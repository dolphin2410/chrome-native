# chrome-native
크롬과 네이티브 코드의 상호작용을 위한 라이브러리

## 구조
`rt` 프로그램이 컴파일된 라이브러리를 플러그인 처럼 로드하고 실행합니다.

## chrome-native 설치 방법
1. [Releases](/releases)에서 최신 msi 패키지 다운로드
2. 설치
3. [개발 문서](./docs)를 참고해 플러그인을 제작하세요

### 플러그인 추가
```bash
chrome-native add-library path/to/plugin.dll
```

### 추가된 플러그인 확인
```bash
chrome-native list-libraries
```

### 플러그인 제거
```bash
chrome-native remove-library path/to/plugin.dll
```

## 크롬에서 실행하기
[네이티브 예시](./example)
[확장프로그램 예시](./ext) - ***WIP***