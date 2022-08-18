# chrome-native
크롬과 네이티브 코드의 상호작용을 위한 라이브러리

## 구조
`rt` 프로그램이 컴파일된 라이브러리를 플러그인 처럼 로드하고 실행합니다.

## chrome-native 설치 방법
1. [Releases](./releases)에서 최신 msi 패키지 다운로드
2. 설치
3. [플러그인 설정하기](#플러그인-준비-및-설정)

## 플러그인 준비 및 설정
***예시 프로그램은 [Releases](./releases)에서 준비되어 있습니다. (`plugin.dll`)***

[개발 문서](./example) - ***TODO!***

### 플러그인 추가
```bash
chrome-native --add-library path/to/plugin.dll
```

### 추가된 플러그인 확인
```bash
chrome-native --list-libraries
```

### 플러그인 제거
```bash
chrome-native --remove-library path/to/plugin.dll
```

## 크롬에서 실행하기
[예시 코드 및 설명](./ext) - ***TODO!***