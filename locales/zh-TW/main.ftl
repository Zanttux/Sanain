# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = 檔案
file-open = 打開…
file-save = 儲存
status-tabs = { $count ->
   *[other] { $count } 個標籤頁
    }
error-open = 無法打開「{ $name }」。
menu-edit = 編輯
file-new = 新增標籤頁
file-save-as = 另存新檔…
edit-undo = 還原
edit-redo = 重做
edit-select-all = 全選
untitled = 未命名
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = 行 { $line }，欄 { $column }
status-selections = { $count ->
   *[other] { $count } 個選取範圍
    }
status-saved = 已儲存「{ $name }」
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8（含 BOM）
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = 混合換行字元
editor-accessible-name = 文字編輯器
button-save = 儲存
button-dont-save = 不要儲存
button-cancel = 取消
button-ok = 好
button-overwrite = 覆寫
button-save-as = 另存新檔…
dialog-unsaved-title = 要儲存對「{ $name }」的變更嗎？
dialog-unsaved-body = 若不儲存，你的變更將會遺失。
dialog-conflict-title = 「{ $name }」已在磁碟上變更
dialog-conflict-body = 你打開此檔案後，另一個程式修改了它。覆寫將取代這些變更。
dialog-readonly-title = 「{ $name }」為唯讀
dialog-readonly-body = 請改用其他名稱儲存拷貝。
error-open-encoding = 「{ $name }」不是 UTF-8 或 UTF-16 文字。檔案未打開，因此其內容未變更。請使用「檔案 > 以指定編碼重新打開」選擇編碼。
error-save = 無法儲存「{ $name }」：{ $reason }
menu-window = 視窗
edit-cut = 剪下
edit-copy = 拷貝
edit-paste = 貼上
app-quit = 結束 Sanain
app-hide = 隱藏 Sanain
app-hide-others = 隱藏其他
app-show-all = 顯示全部
window-minimize = 縮到最小
window-zoom = 縮放
status-recovered = 已回復上次工作階段中未儲存的變更
status-recovery-failed = 無法儲存回復拷貝：{ $reason }
edit-duplicate-line = 複製行
edit-delete-line = 刪除行
edit-move-line-up = 上移行
edit-move-line-down = 下移行
file-close-tab = 關閉標籤頁
file-reopen-tab = 重新打開已關閉的標籤頁
window-next-tab = 下一個標籤頁
window-prev-tab = 上一個標籤頁
tab-close = 關閉標籤頁
edit-find = 尋找…
edit-find-next = 尋找下一個
edit-find-prev = 尋找上一個
edit-replace = 取代…
find-placeholder = 尋找
replace-placeholder = 取代為
find-match-case = 大小寫須相符
find-whole-word = 全字拼寫須相符
find-regex = 規則運算式
find-no-results = 沒有結果
find-count = { $total ->
   *[other] { $total } 個符合項目
    }
find-count-current = 第 { $current } 個，共 { $total } 個
find-prev = 上一個符合項目 (Shift+Enter)
find-next = 下一個符合項目 (Enter)
find-close = 關閉 (Esc)
replace-one = 取代
replace-all = 全部取代
status-replaced = { $count ->
   *[other] 已取代 { $count } 個符合項目
    }
language-plain = 純文字
menu-view = 顯示方式
app-settings = 設定…
view-zoom-in = 放大
view-zoom-out = 縮小
view-zoom-reset = 實際大小
settings-title = 設定
settings-font-size = 字體大小
settings-tab-width = 定位點寬度
settings-insert-spaces = 按下 Tab 鍵時插入空格
settings-theme = 外觀
settings-theme-system = 跟隨系統
settings-theme-light = 淺色
settings-theme-dark = 深色
settings-restore-session = 重新打開上次工作階段中的檔案
settings-language = 語言
settings-language-system = 跟隨系統
settings-language-note = 語言變更將在重新啟動 Sanain 後生效。
settings-save-failed = 無法儲存設定：{ $reason }
view-command-palette = 命令面板…
palette-placeholder = 輸入命令
view-markdown-source = Markdown 原始碼
view-markdown-split = Markdown 原始碼與預覽
view-markdown-preview = Markdown 預覽
preview-too-large = 此文件過大，無法預覽。
preview-mermaid-unsupported-type = 不支援此圖表類型；正在顯示原始碼。
preview-mermaid-unsupported-syntax = 圖表使用了無法繪製的語法；正在顯示原始碼。
preview-mermaid-too-large = 圖表過大，無法繪製；正在顯示原始碼。
menu-tools = 工具
tools-json-validate = 驗證 JSON
tools-json-format = 格式化 JSON
tools-json-minify = 壓縮 JSON
tools-sort-lines = 排序行
tools-sort-lines-desc = 遞減排序行
tools-unique-lines = 移除重複行
tools-trim-trailing = 刪除行尾空白
tools-upper-case = 轉為大寫
tools-lower-case = 轉為小寫
tools-to-lf = 將換行字元轉換為 LF
tools-to-crlf = 將換行字元轉換為 CRLF
tools-json-valid = JSON 有效
tools-done = 完成。還原可復原變更。
tools-no-change = 沒有需要變更的內容
tools-json-error = JSON 無效，位於第 { $line } 行第 { $column } 欄：{ $reason }
json-reason-unterminated-string = 字串未結束
json-reason-invalid-u-escape = 無效的 \u 跳脫字元
json-reason-invalid-escape = 無效的跳脫字元
json-reason-control-character-in-string = 字串中含有控制字元
json-reason-invalid-number = 無效的數字
json-reason-unexpected-character = 非預期的字元
json-reason-unexpected-content-after-the-value = 值之後有非預期的內容
json-reason-unexpected-comma = 非預期的逗號
json-reason-expected-a-string-key = 應為字串鍵
json-reason-expected = 應為 ':'
json-reason-expected-or-a-closing-bracket = 應為 ',' 或右括號
json-reason-expected-a-value = 應為值
json-reason-empty-document = 空白文件
json-reason-unexpected-end-of-input = 輸入意外結束
json-reason-too-deeply-nested-to-format = 巢狀層級過深，無法格式化
tools-compare = 比較…
compare-need-two = 請打開另一個標籤頁進行比較。
compare-pick-title = 將「{ $name }」與以下項目比較
compare-count = { $count ->
    [0] 沒有差異
   *[other] { $count } 處變更
    }
compare-close = 關閉比較
compare-to-left = 將此變更拷貝到左側
compare-to-right = 將此變更拷貝到右側
compare-stale = 比較期間文件已變更。比較已重新整理；請再試一次。
view-structured = 結構化顯示方式 (CSV, JSON, XML, YAML)
csv-delimiter = 分隔字元
csv-rows = { $count ->
   *[other] { $count } 列
    }
csv-unterminated = 從第 { $line } 行開始的引號未閉合
csv-truncated = 僅顯示前 200,000 列
csv-edit-cell = 編輯儲存格
csv-insert-row-below = 在下方插入列
csv-delete-row = 刪除列
csv-read-only = 唯讀檔案：無法編輯表格
json-more = … 還有 { $count } 項
json-edit-value = 編輯值
json-rename-key = 重新命名鍵
json-add-after = 在後面新增
json-add-inside = 在內部新增
json-delete-node = 刪除
json-read-only = 唯讀檔案：無法編輯樹狀結構
json-invalid-value = 不是有效的 JSON 值
json-duplicate-key = 物件中已有同名的鍵
dialog-open-link-title = 要在網頁瀏覽器中打開此連結嗎？
button-open = 打開
link-blocked = 基於安全考量，不會打開此類連結。
link-not-found = 找不到連結的檔案：{ $target }
enc-windows-1252 = 西歐語系
enc-iso-8859-15 = 西歐語系
enc-windows-1250 = 中歐語系
enc-iso-8859-2 = 中歐語系
enc-windows-1251 = 斯拉夫語系
enc-koi8-r = 斯拉夫語系
enc-windows-1253 = 希臘文
enc-windows-1254 = 土耳其文
enc-windows-1255 = 希伯來文
enc-windows-1256 = 阿拉伯文
enc-windows-1257 = 波羅的海語系
enc-windows-874 = 泰文
enc-shift_jis = 日文
enc-euc-jp = 日文
enc-gb18030 = 簡體中文
enc-big5 = 繁體中文
enc-euc-kr = 韓文
file-reopen-encoding = 以指定編碼重新打開…
file-save-encoding = 以指定編碼儲存…
encoding-reopen-title = 以指定編碼重新打開
encoding-save-title = 以指定編碼儲存
encoding-not-exact = 使用此編碼無法在不變更檔案的情況下打開它：{ $reason }
encoding-unencodable = 「{ $character }」無法以 { $encoding } 儲存。請選擇其他編碼或修改文字；未寫入任何內容。
encoding-save-first = 以其他編碼重新打開前，請先儲存或還原你的變更。
image-remote = 不載入遠端影像。
image-blocked = 僅顯示此文件所在檔案夾中的影像。
image-unsupported = 不支援的影像格式。
image-too-large = 影像過大，無法顯示。
image-unreadable = 無法讀取影像。
settings-theme-high-contrast = 高對比
view-fold = 收合
view-unfold = 展開
view-unfold-all = 全部展開
settings-shortcuts = 鍵盤快速鍵
settings-keymap-preset = 預設組
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = 更改
settings-shortcut-reset = 重置
settings-shortcut-press = 請按下快速鍵…（按 Esc 取消）
settings-shortcut-moved = 快速鍵原先指定給：
edit-find-in-folder = 在檔案夾中尋找…
folder-search-placeholder = 在檔案夾中尋找
folder-search-run = 搜尋
folder-search-stop = 停止
folder-search-no-folder = 選擇檔案夾
folder-search-summary = { $files ->
   *[other] { $files } 個檔案中
    }共 { $count ->
   *[other] { $count } 個符合項目
    }
app-check-updates = 檢查更新項目…
update-checking = 正在檢查更新項目…
update-up-to-date = Sanain { $version } 是最新版本。
update-no-releases = Sanain 尚未發布任何版本。
update-failed = 無法檢查更新項目：{ $reason }
update-invalid = 非預期的回應
update-available-title = Sanain { $version } 現已推出
update-available-body = 請開啟下載頁面取得。Sanain 不會自行下載或安裝任何內容。
update-open-page = 開啟下載頁面
update-later = 稍後
settings-check-updates = 每天檢查一次更新項目
settings-check-updates-detail = 向 github.com 查詢最新版本。不會傳送其他任何資訊。
file-local-history = 本機歷史記錄…
file-clear-history = 清除本機歷史記錄…
history-title = { $name } 的本機歷史記錄
history-open = 打開
history-restore = 回復
history-empty = 尚無已儲存的版本。每次儲存時都會保留一份副本。
history-unreadable = 無法讀取此版本。
history-just-now = 剛剛
history-minutes-ago = { $count ->
   *[other] { $count } 分鐘前
    }
history-hours-ago = { $count ->
   *[other] { $count } 小時前
    }
history-days-ago = { $count ->
   *[other] { $count } 天前
    }
history-purge-title = 要刪除全部本機歷史記錄嗎？
history-purge-body = 所有檔案的已儲存副本將從這部電腦中移除。檔案本身不會變更。
history-purge-confirm = 刪除歷史記錄
history-purged = 已刪除本機歷史記錄。
settings-local-history = 保留已儲存檔案的本機歷史記錄
settings-local-history-detail = 每個檔案最多保留 50 個版本，保留 30 天，僅存於這部電腦。
file-open-log = 在記錄檢視器中開啟…
log-lines = { $count ->
   *[other] { $count } 行
    }
log-indexing = 正在讀取… { $percent }%
log-matches = { $count ->
   *[other] { $count } 行符合
    }
log-filter-hint = 篩選行
log-bad-pattern = 無效的模式
log-follow = 跟隨
log-close = 關閉檢視器
log-opened-large = 「{ $name }」較大，因此已在記錄檢視器中以唯讀方式開啟。
tab-pin = 釘選標籤頁
tab-unpin = 取消釘選標籤頁
edit-add-cursor-above = 在上方新增游標
edit-add-cursor-below = 在下方新增游標
view-word-wrap = 自動換行
status-wrap-on = 長行現在會依視窗寬度自動換行。
status-wrap-off = 長行不再自動換行。
settings-word-wrap = 長行自動換行
edit-go-to-line = 移至行…
edit-go-to-heading = 移至標題…
goto-line-hint = 行 或 行:欄
goto-line-note = { $count ->
   *[other] 文件共有 { $count } 行。
    }
goto-line-invalid = 請輸入行號，可選擇在後面加上 :欄。
goto-heading-hint = 輸入以篩選標題
goto-heading-none = 此文件沒有標題。
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } 無效，位於第 { $line } 行第 { $column } 欄：{ $reason }
edit-select-next-occurrence = 選取下一個符合項目
tools-record-macro = 開始或停止錄製巨集
tools-play-macro = 播放巨集
macro-recording = 正在錄製巨集
macro-recorded = { $count ->
   *[other] 巨集已錄製：{ $count } 個步驟。
    }
tools-compare-saved = 與已儲存的版本比較
tools-compare-clipboard = 與剪貼簿比較
compare-saved-label = 「{ $name }」（磁碟上）
compare-clipboard-label = 剪貼簿
compare-saved-failed = 無法讀取已儲存的檔案：{ $reason }
compare-clipboard-empty = 剪貼簿中沒有可比較的文字。
error-save-disk-full = 未儲存 { $name }：磁碟已滿。磁碟上的檔案未變更，你的文字仍在這裡。請釋放一些空間，或使用「另存新檔」將其儲存到其他位置。
error-save-permission = 未儲存 { $name }：你沒有在該位置寫入的權限。磁碟上的檔案未變更。請使用「另存新檔」將其儲存到其他位置。
error-save-read-only-volume = 未儲存 { $name }：磁碟為唯讀。磁碟上的檔案未變更。請使用「另存新檔」將其儲存到其他位置。
replace-preview-title = { $count ->
   *[other] 取代 { $count } 個符合項目？
    }
replace-preview-confirm = { $count ->
   *[other] 取代 { $count } 個
    }
replace-preview-more = 僅顯示前 200 行變更。
replace-preview-stale = 文件已變更，未取代任何內容。請重新執行「全部取代」。
encoding-detected = 已以 { $encoding } 打開（根據檔案內容偵測）。
encoding-suggested = 根據檔案內容建議的編碼：{ $encoding }。打開前請檢查預覽。
app-about = 關於 Sanain
menu-help = 說明
about-tagline = 文字的力量。
about-version = 版本 { $version }
about-license = 依 Mozilla Public License 2.0 授權的自由開放原始碼軟體。
about-ai = AI 打造，人類主導，社群驅動。
about-notices = 第三方元件及其授權條款列於 THIRD_PARTY_NOTICES，隨每個版本一併提供。
about-source = GitHub 上的原始碼與問題回報
tools-xml-validate = 驗證 XML
tools-yaml-validate = 驗證 YAML
tools-markup-valid = { $format } 有效
file-open-recent = 打開最近使用的項目…
file-save-all = 全部儲存
file-revert = 回復至已儲存的版本
recent-hint = 輸入以篩選最近使用的檔案
recent-none = 尚無最近使用的檔案。
revert-title = 要丟棄「{ $name }」的變更嗎？
revert-body = 將從磁碟重新載入檔案。未儲存的變更會遺失且無法還原。
revert-confirm = 丟棄並重新載入
revert-done = 已從磁碟重新載入「{ $name }」。
status-saved-all = { $count ->
    [0] 沒有需要儲存的內容。
   *[other] 已儲存 { $count } 個檔案。
    }
edit-toggle-comment = 切換行註解
edit-indent = 縮排行
edit-outdent = 減少行縮排
comment-unsupported = 此檔案類型不支援行註解。
tab-close-others = 關閉其他標籤頁
tab-close-all = 關閉所有標籤頁
file-copy-path = 複製檔案路徑
file-reveal-macos = 在 Finder 中顯示
file-reveal-windows = 在檔案總管中顯示
file-reveal-other = 在檔案管理員中顯示
status-path-copied = 已複製檔案路徑。
tabs-kept-unsaved = { $count ->
   *[other] 已保留 { $count } 個含未儲存變更的標籤頁。
    }
status-indent-tabs = 定位字元
status-indent-spaces = 空格: { $count }
find-searching = 正在搜尋…
compare-ignore-whitespace = 忽略空白
tools-compare-file = 與檔案比較…
compare-file-failed = 無法與 { $name } 比較：{ $reason }
cli-help =
    用法：
      sanain [檔案…]         開啟檔案
      sanain --diff 左 右    並排比較兩個檔案
      sanain --version       顯示版本
      sanain --help          顯示此說明
    以連字號開頭的檔案名稱前請使用 --。
cli-unknown-option = 未知的選項 { $option }。
cli-diff-needs-two = --diff 需要兩個檔案：sanain --diff 左 右
window-move-tab-left = 向左移動標籤頁
window-move-tab-right = 向右移動標籤頁
edit-skip-occurrence = 跳至下一個符合項目
tools-play-macro-times = 多次播放巨集…
macro-times-hint = 次數
macro-times-note = 1 到 { $max }
macro-times-invalid = 請輸入 1 到 { $max } 之間的數字。
view-outline = 大綱
outline-empty = 此文件中沒有標題。
outline-close = 關閉大綱
folder-replace-placeholder = 取代為
folder-replace-run = 在檔案中取代…
replace-files-title = 要取代 { $files ->
   *[other] { $files } 個檔案中
    }的 { $count ->
   *[other] { $count } 個符合項目
    }嗎？
replace-files-body = 在標籤頁中開啟的檔案會在標籤頁中變更，且可以復原；請自行儲存這些檔案。其他檔案會立即儲存，若已啟用本機歷史記錄，其先前內容會保留在本機歷史記錄中。在此期間已於磁碟上變更的檔案將會略過。
replace-files-confirm = 取代
replace-files-done = 已取代 { $files ->
   *[other] { $files } 個檔案中
    }的 { $count ->
   *[other] { $count } 個符合項目
    }（在標籤頁中開啟：{ $open } 個，未儲存）。
replace-files-failed = { $failed ->
   *[other] { $failed } 個檔案未變更；第一個：
    } { $first }
image-outside = 此影像位於文件所在檔案夾之外。
image-allow = 顯示此檔案夾中的影像
cli-help-integration =
    與檔案管理器整合（僅限目前使用者）：
      sanain --register           將 Sanain 加入「開啟檔案」（Linux、Windows）
      sanain --unregister         再次移除
cli-registered = 已將 Sanain 加入「開啟檔案」。{ $place }
cli-unregistered = 已從「開啟檔案」中移除 Sanain。
cli-register-failed = 無法變更檔案管理器整合：{ $reason }
cli-register-macos = macOS 上無需註冊：只要 Sanain.app 位於「應用程式」檔案夾中，Finder 就會在「開啟檔案」中提供它。
settings-tab-general = 一般
settings-tab-editor = 編輯器
settings-tab-appearance = 外觀
settings-shortcut-search = 搜尋命令或按鍵
settings-font = 字體
settings-font-default = 內建等寬字體
settings-font-loading = 正在尋找已安裝的字體…
settings-font-missing = 找不到字體 { $font }，改用內建字體。
settings-theme-more = 更多主題
settings-theme-user-note = 自訂主題：{ $folder } 資料夾中的 .theme 檔案（請參閱 docs/THEMES.md）。
language-auto = 自動偵測
language-choose = 選擇此分頁的語言
tools-xml-format = 格式化 XML
tools-save-macro = 儲存巨集…
tools-play-saved-macro = 播放已儲存的巨集…
tools-delete-saved-macro = 刪除已儲存的巨集…
macro-save-hint = 巨集名稱
macro-save-note = 同名的已儲存巨集將被取代。
macro-save-empty = 沒有可儲存的內容：請先錄製巨集。
macro-saved = 巨集已儲存：{ $name }
macro-deleted = 巨集已刪除：{ $name }
macro-name-invalid = 請輸入 1 到 { $max } 個字元的單行名稱。
macro-save-too-large = 此巨集太長，無法儲存。
macro-save-full = 最多可儲存 { $max } 個巨集。請先刪除一個。
macro-save-failed = 無法儲存巨集：{ $error }
macro-saved-hint = 輸入以篩選已儲存的巨集
macro-saved-none = 尚無已儲存的巨集。
settings-shortcut-import-npp = 從 Notepad++ 匯入…
settings-shortcut-import-npp-hint = 選擇 Notepad++ 的 shortcuts.xml。在 Windows 上它位於 %AppData%\Notepad++\shortcuts.xml。
settings-shortcut-import-npp-done = 已從 Notepad++ 匯入：{ $imported }。已略過：{ $skipped }。剩餘衝突：{ $conflicts }。
settings-shortcut-import-npp-failed = 無法匯入 { $name }：{ $reason }
settings-shortcut-import-npp-too-large = 檔案大於 1 MiB。
settings-shortcut-import-npp-invalid = 此檔案不是有效的 Notepad++ shortcuts.xml。
view-split-right = 向右分割
view-split-down = 向下分割
view-close-pane = 關閉窗格
view-focus-next-pane = 聚焦下一個窗格
view-focus-prev-pane = 聚焦上一個窗格
status-pane-limit = 已達窗格上限（最多 { $max } 個）。
