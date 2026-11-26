# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = 파일
file-open = 열기…
file-save = 저장
status-tabs = { $count ->
   *[other] 탭 { $count }개
    }
error-open = { $name }을(를) 열 수 없습니다.
menu-edit = 편집
file-new = 새 탭
file-save-as = 다른 이름으로 저장…
edit-undo = 실행 취소
edit-redo = 다시 실행
edit-select-all = 모두 선택
untitled = 제목 없음
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = 줄 { $line }, 열 { $column }
status-selections = { $count ->
   *[other] 선택 영역 { $count }개
    }
status-saved = { $name } 저장됨
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 (BOM 포함)
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = 혼합된 줄 끝
editor-accessible-name = 텍스트 편집기
button-save = 저장
button-dont-save = 저장 안 함
button-cancel = 취소
button-ok = 확인
button-overwrite = 덮어쓰기
button-save-as = 다른 이름으로 저장…
dialog-unsaved-title = { $name }의 변경 사항을 저장하시겠습니까?
dialog-unsaved-body = 저장하지 않으면 변경 사항이 손실됩니다.
dialog-conflict-title = 디스크에서 { $name }이(가) 변경됨
dialog-conflict-body = 파일을 연 후 다른 프로그램이 이 파일을 수정했습니다. 덮어쓰면 해당 변경 사항이 대체됩니다.
dialog-readonly-title = { $name }은(는) 읽기 전용입니다
dialog-readonly-body = 대신 다른 이름으로 사본을 저장하십시오.
error-open-encoding = { $name }은(는) UTF-8 또는 UTF-16 텍스트가 아닙니다. 파일을 열지 않았으므로 내용은 변경되지 않았습니다. 파일 > 인코딩 지정하여 다시 열기에서 인코딩을 선택하십시오.
error-save = { $name }을(를) 저장할 수 없습니다: { $reason }
menu-window = 윈도우
edit-cut = 잘라내기
edit-copy = 복사하기
edit-paste = 붙여넣기
app-quit = Sanain 종료
app-hide = Sanain 가리기
app-hide-others = 기타 가리기
app-show-all = 모두 보기
window-minimize = 최소화
window-zoom = 확대/축소
status-recovered = 지난 세션의 저장되지 않은 변경 사항을 복원했습니다
status-recovery-failed = 복구 사본을 저장할 수 없습니다: { $reason }
edit-duplicate-line = 줄 복제
edit-delete-line = 줄 삭제
edit-move-line-up = 줄을 위로 이동
edit-move-line-down = 줄을 아래로 이동
file-close-tab = 탭 닫기
file-reopen-tab = 닫은 탭 다시 열기
window-next-tab = 다음 탭
window-prev-tab = 이전 탭
tab-close = 탭 닫기
edit-find = 찾기…
edit-find-next = 다음 찾기
edit-find-prev = 이전 찾기
edit-replace = 바꾸기…
find-placeholder = 찾기
replace-placeholder = 바꿀 내용
find-match-case = 대소문자 구분
find-whole-word = 단어 단위로
find-regex = 정규 표현식
find-no-results = 결과 없음
find-count = { $total ->
   *[other] 일치 항목 { $total }개
    }
find-count-current = { $current } / { $total }
find-prev = 이전 일치 항목 (Shift+Enter)
find-next = 다음 일치 항목 (Enter)
find-close = 닫기 (Esc)
replace-one = 바꾸기
replace-all = 모두 바꾸기
status-replaced = { $count ->
   *[other] 일치 항목 { $count }개를 바꿨습니다
    }
language-plain = 일반 텍스트
menu-view = 보기
app-settings = 설정…
view-zoom-in = 확대
view-zoom-out = 축소
view-zoom-reset = 실제 크기
settings-title = 설정
settings-font-size = 글꼴 크기
settings-tab-width = 탭 너비
settings-insert-spaces = Tab 키를 누르면 공백 삽입
settings-theme = 모양
settings-theme-system = 시스템 설정 따르기
settings-theme-light = 라이트
settings-theme-dark = 다크
settings-restore-session = 지난 세션의 파일 다시 열기
settings-language = 언어
settings-language-system = 시스템 설정 따르기
settings-language-note = 언어 변경은 Sanain을 다시 시작한 후 적용됩니다.
settings-save-failed = 설정을 저장할 수 없습니다: { $reason }
view-command-palette = 명령 팔레트…
palette-placeholder = 명령 입력
view-markdown-source = Markdown 소스
view-markdown-split = Markdown 소스 및 미리보기
view-markdown-preview = Markdown 미리보기
preview-too-large = 이 문서는 너무 커서 미리 볼 수 없습니다.
preview-mermaid-unsupported = Mermaid 다이어그램은 아직 렌더링되지 않습니다. 소스를 표시합니다.
menu-tools = 도구
tools-json-validate = JSON 검사
tools-json-format = JSON 서식 지정
tools-json-minify = JSON 축소
tools-sort-lines = 줄 정렬
tools-sort-lines-desc = 줄 내림차순 정렬
tools-unique-lines = 중복 줄 제거
tools-trim-trailing = 줄 끝 공백 제거
tools-upper-case = 대문자로
tools-lower-case = 소문자로
tools-to-lf = 줄 끝을 LF로 변환
tools-to-crlf = 줄 끝을 CRLF로 변환
tools-json-valid = 유효한 JSON입니다
tools-done = 완료되었습니다. 실행 취소로 되돌릴 수 있습니다.
tools-no-change = 변경할 내용 없음
tools-json-error = 잘못된 JSON, { $line }줄 { $column }열: { $reason }
json-reason-unterminated-string = 종료되지 않은 문자열
json-reason-invalid-u-escape = 잘못된 \u 이스케이프
json-reason-invalid-escape = 잘못된 이스케이프
json-reason-control-character-in-string = 문자열에 제어 문자가 있음
json-reason-invalid-number = 잘못된 숫자
json-reason-unexpected-character = 예기치 않은 문자
json-reason-unexpected-content-after-the-value = 값 뒤에 예기치 않은 내용이 있음
json-reason-unexpected-comma = 예기치 않은 쉼표
json-reason-expected-a-string-key = 문자열 키가 필요함
json-reason-expected = ':'이(가) 필요함
json-reason-expected-or-a-closing-bracket = ',' 또는 닫는 괄호가 필요함
json-reason-expected-a-value = 값이 필요함
json-reason-empty-document = 빈 문서
json-reason-unexpected-end-of-input = 입력이 예기치 않게 끝남
json-reason-too-deeply-nested-to-format = 중첩이 너무 깊어 서식을 지정할 수 없음
tools-compare = 비교 대상…
compare-need-two = 비교할 다른 탭을 여십시오.
compare-pick-title = { $name }과(와) 비교할 대상
compare-count = { $count ->
    [0] 차이 없음
   *[other] 변경 사항 { $count }개
    }
compare-close = 비교 닫기
compare-to-left = 이 변경 사항을 왼쪽으로 복사
compare-to-right = 이 변경 사항을 오른쪽으로 복사
compare-stale = 비교 중에 문서가 변경되었습니다. 비교를 새로 고쳤으니 다시 시도하십시오.
view-structured = 구조화된 보기 (CSV, JSON, XML, YAML)
csv-delimiter = 구분 기호
csv-rows = { $count ->
   *[other] { $count }행
    }
csv-unterminated = { $line }번째 줄에서 시작된 따옴표가 닫히지 않음
csv-truncated = 처음 200,000행 표시 중
json-more = … { $count }개 더
dialog-open-link-title = 웹 브라우저에서 이 링크를 여시겠습니까?
button-open = 열기
link-blocked = 안전을 위해 이 종류의 링크는 열지 않습니다.
link-not-found = 링크된 파일을 찾을 수 없음: { $target }
enc-windows-1252 = 서유럽어
enc-iso-8859-15 = 서유럽어
enc-windows-1250 = 중앙 유럽어
enc-iso-8859-2 = 중앙 유럽어
enc-windows-1251 = 키릴 자모
enc-koi8-r = 키릴 자모
enc-windows-1253 = 그리스어
enc-windows-1254 = 터키어
enc-windows-1255 = 히브리어
enc-windows-1256 = 아랍어
enc-windows-1257 = 발트어
enc-windows-874 = 태국어
enc-shift_jis = 일본어
enc-euc-jp = 일본어
enc-gb18030 = 중국어 간체
enc-big5 = 중국어 번체
enc-euc-kr = 한국어
file-reopen-encoding = 인코딩 지정하여 다시 열기…
file-save-encoding = 인코딩 지정하여 저장…
encoding-reopen-title = 인코딩 지정하여 다시 열기
encoding-save-title = 인코딩 지정하여 저장
encoding-not-exact = 이 인코딩으로는 파일을 변경하지 않고 열 수 없습니다: { $reason }
encoding-unencodable = “{ $character }”은(는) { $encoding }(으)로 저장할 수 없습니다. 다른 인코딩을 선택하거나 텍스트를 변경하십시오. 아무것도 기록되지 않았습니다.
encoding-save-first = 다른 인코딩으로 다시 열기 전에 변경 사항을 저장하거나 실행 취소하십시오.
image-remote = 원격 이미지는 불러오지 않습니다.
image-blocked = 이 문서의 폴더에 있는 이미지만 표시됩니다.
image-unsupported = 지원되지 않는 이미지 형식입니다.
image-too-large = 이미지가 너무 커서 표시할 수 없습니다.
image-unreadable = 이미지를 읽을 수 없습니다.
settings-theme-high-contrast = 고대비
view-fold = 접기
view-unfold = 펼치기
view-unfold-all = 모두 펼치기
settings-shortcuts = 키보드 단축키
settings-keymap-preset = 사전 설정
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = 변경
settings-shortcut-reset = 재설정
settings-shortcut-press = 단축키를 누르십시오… (Esc로 취소)
settings-shortcut-moved = 단축키를 다음에서 이동함:
edit-find-in-folder = 폴더에서 찾기…
folder-search-placeholder = 폴더에서 찾기
folder-search-run = 검색
folder-search-stop = 중지
folder-search-no-folder = 폴더 선택
folder-search-summary = { $files ->
   *[other] 파일 { $files }개에서
    } { $count ->
   *[other] 일치 항목 { $count }개
    }
app-check-updates = 업데이트 확인…
update-checking = 업데이트 확인 중…
update-up-to-date = Sanain { $version }이(가) 최신 버전입니다.
update-no-releases = 아직 게시된 Sanain 릴리스가 없습니다.
update-failed = 업데이트를 확인할 수 없음: { $reason }
update-invalid = 예기치 않은 응답
update-available-title = Sanain { $version } 사용 가능
update-available-body = 다운로드 페이지를 열어 받으세요. Sanain은 스스로 아무것도 다운로드하거나 설치하지 않습니다.
update-open-page = 다운로드 페이지 열기
update-later = 나중에
settings-check-updates = 하루에 한 번 업데이트 확인
settings-check-updates-detail = github.com에 최신 릴리스를 요청합니다. 그 외에는 아무것도 전송하지 않습니다.
file-local-history = 로컬 기록…
file-clear-history = 로컬 기록 지우기…
history-title = { $name }의 로컬 기록
history-open = 열기
history-restore = 복원
history-empty = 저장된 버전이 아직 없습니다. 저장할 때마다 사본이 보관됩니다.
history-unreadable = 이 버전을 읽을 수 없습니다.
history-just-now = 방금
history-minutes-ago = { $count ->
   *[other] { $count }분 전
    }
history-hours-ago = { $count ->
   *[other] { $count }시간 전
    }
history-days-ago = { $count ->
   *[other] { $count }일 전
    }
history-purge-title = 로컬 기록을 모두 삭제하시겠습니까?
history-purge-body = 모든 파일의 저장된 사본이 이 컴퓨터에서 제거됩니다. 파일 자체는 변경되지 않습니다.
history-purge-confirm = 기록 삭제
history-purged = 로컬 기록을 삭제했습니다.
settings-local-history = 저장된 파일의 로컬 기록 보관
settings-local-history-detail = 파일당 최대 50개 버전을 30일 동안 이 컴퓨터에만 보관합니다.
file-open-log = 로그 뷰어에서 열기…
log-lines = { $count ->
   *[other] { $count }줄
    }
log-indexing = 읽는 중… { $percent }%
log-matches = { $count ->
   *[other] 일치하는 줄 { $count }개
    }
log-filter-hint = 줄 필터
log-bad-pattern = 잘못된 패턴
log-follow = 따라가기
log-close = 뷰어 닫기
log-opened-large = { $name }은(는) 크기가 커서 로그 뷰어에서 읽기 전용으로 열렸습니다.
tab-pin = 탭 고정
tab-unpin = 탭 고정 해제
edit-add-cursor-above = 위에 커서 추가
edit-add-cursor-below = 아래에 커서 추가
view-word-wrap = 자동 줄 바꿈
status-wrap-on = 이제 긴 줄이 창 너비에 맞춰 줄 바꿈됩니다.
status-wrap-off = 이제 긴 줄이 줄 바꿈되지 않습니다.
settings-word-wrap = 긴 줄 자동 줄 바꿈
edit-go-to-line = 줄로 이동…
edit-go-to-heading = 제목으로 이동…
goto-line-hint = 줄 또는 줄:열
goto-line-note = { $count ->
   *[other] 문서에 { $count }개의 줄이 있습니다.
    }
goto-line-invalid = 줄 번호를 입력하세요. 필요하면 뒤에 :열을 붙이세요.
goto-heading-hint = 입력하여 제목 필터링
goto-heading-none = 이 문서에는 제목이 없습니다.
format-xml = XML
format-yaml = YAML
tools-tree-error = 잘못된 { $format }, { $line }줄 { $column }열: { $reason }
edit-select-next-occurrence = 다음 항목 선택
tools-record-macro = 매크로 기록 시작/중지
tools-play-macro = 매크로 재생
macro-recording = 매크로 기록 중
macro-recorded = { $count ->
   *[other] 매크로 기록됨: { $count }단계.
    }
tools-compare-saved = 저장된 버전과 비교
tools-compare-clipboard = 클립보드와 비교
compare-saved-label = { $name } (디스크)
compare-clipboard-label = 클립보드
compare-saved-failed = 저장된 파일을 읽을 수 없습니다: { $reason }
compare-clipboard-empty = 클립보드에 비교할 텍스트가 없습니다.
error-save-disk-full = { $name }을(를) 저장하지 못했습니다: 디스크가 가득 찼습니다. 디스크의 파일은 변경되지 않았으며 텍스트는 그대로 남아 있습니다. 공간을 확보하거나 다른 이름으로 저장을 사용해 다른 위치에 저장하세요.
error-save-permission = { $name }을(를) 저장하지 못했습니다: 해당 위치에 쓸 권한이 없습니다. 디스크의 파일은 변경되지 않았습니다. 다른 이름으로 저장을 사용해 다른 위치에 저장하세요.
error-save-read-only-volume = { $name }을(를) 저장하지 못했습니다: 디스크가 읽기 전용입니다. 디스크의 파일은 변경되지 않았습니다. 다른 이름으로 저장을 사용해 다른 위치에 저장하세요.
replace-preview-title = { $count ->
   *[other] 일치 항목 { $count }개를 바꿀까요?
    }
replace-preview-confirm = { $count ->
   *[other] { $count }개 바꾸기
    }
replace-preview-more = 변경되는 처음 200줄만 표시됩니다.
replace-preview-stale = 문서가 변경되어 아무것도 바꾸지 않았습니다. 모두 바꾸기를 다시 실행하세요.
view-split-editor = 편집기 분할
encoding-detected = { $encoding }(으)로 열었습니다(파일 내용에서 감지됨).
encoding-suggested = 파일 내용으로 추천된 인코딩: { $encoding }. 열기 전에 미리 보기를 확인하세요.
app-about = Sanain 정보
menu-help = 도움말
about-tagline = 말의 힘.
about-version = 버전 { $version }
about-license = Mozilla Public License 2.0에 따른 자유 오픈 소스 소프트웨어입니다.
about-ai = AI가 만들고, 사람이 이끌고, 커뮤니티가 키웁니다.
about-notices = 타사 구성 요소와 해당 라이선스는 모든 릴리스에 포함된 THIRD_PARTY_NOTICES에 나와 있습니다.
about-source = GitHub의 소스 코드 및 이슈
tools-xml-validate = XML 검사
tools-yaml-validate = YAML 검사
tools-markup-valid = 유효한 { $format }입니다
file-open-recent = 최근 항목 열기…
file-save-all = 모두 저장
file-revert = 저장된 상태로 되돌리기
recent-hint = 입력하여 최근 파일 필터링
recent-none = 아직 최근 파일이 없습니다.
revert-title = { $name }의 변경 사항을 버리시겠습니까?
revert-body = 파일을 디스크에서 다시 불러옵니다. 저장하지 않은 변경 사항은 사라지며 실행 취소할 수 없습니다.
revert-confirm = 버리고 다시 불러오기
revert-done = 디스크에서 { $name }을(를) 다시 불러왔습니다.
status-saved-all = { $count ->
    [0] 저장할 항목이 없습니다.
   *[other] 파일 { $count }개를 저장했습니다.
    }
edit-toggle-comment = 줄 주석 전환
edit-indent = 줄 들여쓰기
edit-outdent = 줄 내어쓰기
comment-unsupported = 이 파일 형식에는 줄 주석이 없습니다.
tab-close-others = 다른 탭 닫기
tab-close-all = 모든 탭 닫기
file-copy-path = 파일 경로 복사
file-reveal-macos = Finder에서 보기
file-reveal-windows = 파일 탐색기에서 보기
file-reveal-other = 파일 관리자에서 보기
status-path-copied = 파일 경로를 복사했습니다.
tabs-kept-unsaved = { $count ->
   *[other] 저장하지 않은 변경 사항이 있는 탭 { $count }개를 열어 두었습니다.
    }
status-indent-tabs = 탭
status-indent-spaces = 공백: { $count }
find-searching = 검색 중…
compare-ignore-whitespace = 공백 무시
tools-compare-file = 파일과 비교…
compare-file-failed = { $name }와(과) 비교할 수 없습니다: { $reason }
cli-help =
    사용법:
      sanain [파일…]               파일 열기
      sanain --diff 왼쪽 오른쪽    두 파일을 나란히 비교
      sanain --version             버전 표시
      sanain --help                이 도움말 표시
    대시로 시작하는 파일 이름 앞에는 --를 사용하세요.
cli-unknown-option = 알 수 없는 옵션 { $option }.
cli-diff-needs-two = --diff에는 두 파일이 필요합니다: sanain --diff 왼쪽 오른쪽
window-move-tab-left = 탭을 왼쪽으로 이동
window-move-tab-right = 탭을 오른쪽으로 이동
edit-skip-occurrence = 다음 항목으로 건너뛰기
tools-play-macro-times = 매크로 여러 번 재생…
macro-times-hint = 반복 횟수
macro-times-note = 1~{ $max }
macro-times-invalid = 1~{ $max } 사이의 숫자를 입력하세요.
view-outline = 개요
outline-empty = 이 문서에는 제목이 없습니다.
outline-close = 개요 닫기
folder-replace-placeholder = 바꿀 내용
folder-replace-run = 파일에서 바꾸기…
replace-files-title = { $files ->
   *[other] 파일 { $files }개에서
    } { $count ->
   *[other] 일치 항목 { $count }개를
    } 바꿀까요?
replace-files-body = 탭에서 열린 파일은 탭에서 변경되며 실행 취소할 수 있습니다. 이 파일은 직접 저장하세요. 다른 파일은 지금 저장되며, 로컬 기록이 켜져 있으면 이전 내용이 로컬 기록에 보관됩니다. 그동안 디스크에서 변경된 파일은 건너뜁니다.
replace-files-confirm = 바꾸기
replace-files-done = { $files ->
   *[other] 파일 { $files }개에서
    } { $count ->
   *[other] 일치 항목 { $count }개를
    } 바꿨습니다(탭에서 열림: { $open }개, 저장 안 됨).
replace-files-failed = { $failed ->
   *[other] 파일 { $failed }개를 변경하지 못했습니다. 첫 번째 파일:
    } { $first }
image-outside = 이 이미지는 문서의 폴더 밖에 있습니다.
image-allow = 이 폴더의 이미지 표시
cli-help-integration =
    파일 관리자 통합 (현재 사용자만):
      sanain --register           "연결 프로그램"에 Sanain 추가 (Linux, Windows)
      sanain --unregister         다시 제거
cli-registered = Sanain을 "연결 프로그램"에 추가했습니다. { $place }
cli-unregistered = Sanain을 "연결 프로그램"에서 제거했습니다.
cli-register-failed = 파일 관리자 통합을 변경할 수 없습니다: { $reason }
cli-register-macos = macOS에서는 등록할 필요가 없습니다. Sanain.app이 응용 프로그램 폴더에 있으면 Finder가 "연결 프로그램"에 표시합니다.
settings-tab-general = 일반
settings-tab-editor = 편집기
settings-tab-appearance = 모양
settings-shortcut-search = 명령 또는 키 검색
settings-font = 글꼴
settings-font-default = 내장 고정폭 글꼴
settings-font-loading = 설치된 글꼴을 찾는 중…
settings-font-missing = { $font } 글꼴을 찾을 수 없어 내장 글꼴을 사용합니다.
settings-theme-more = 테마 더 보기
settings-theme-user-note = 사용자 테마: { $folder } 폴더의 .theme 파일 (docs/THEMES.md 참조)
language-auto = 자동 감지
language-choose = 이 탭의 언어 선택
tools-xml-format = XML 서식 지정
tools-save-macro = 매크로 저장…
tools-play-saved-macro = 저장된 매크로 재생…
tools-delete-saved-macro = 저장된 매크로 삭제…
macro-save-hint = 매크로 이름
macro-save-note = 같은 이름의 저장된 매크로는 대체됩니다.
macro-save-empty = 저장할 내용이 없습니다. 먼저 매크로를 기록하세요.
macro-saved = 매크로 저장됨: { $name }
macro-deleted = 매크로 삭제됨: { $name }
macro-name-invalid = 한 줄에 1~{ $max }자의 이름을 입력하세요.
macro-save-too-large = 이 매크로는 너무 길어서 저장할 수 없습니다.
macro-save-full = 매크로는 최대 { $max }개까지 저장할 수 있습니다. 먼저 하나를 삭제하세요.
macro-save-failed = 매크로를 저장할 수 없습니다: { $error }
macro-saved-hint = 입력하여 저장된 매크로 필터링
macro-saved-none = 아직 저장된 매크로가 없습니다.
