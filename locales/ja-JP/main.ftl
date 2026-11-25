# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = ファイル
file-open = 開く…
file-save = 保存
status-tabs = { $count ->
   *[other] { $count } 個のタブ
    }
error-open = { $name } を開けませんでした。
menu-edit = 編集
file-new = 新規タブ
file-save-as = 別名で保存…
edit-undo = 取り消す
edit-redo = やり直す
edit-select-all = すべてを選択
untitled = 名称未設定
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = 行 { $line }、列 { $column }
status-selections = { $count ->
   *[other] { $count } 個の選択範囲
    }
status-saved = { $name } を保存しました
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8（BOM 付き）
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = 改行コードが混在
editor-accessible-name = テキストエディタ
button-save = 保存
button-dont-save = 保存しない
button-cancel = キャンセル
button-ok = OK
button-overwrite = 上書き
button-save-as = 別名で保存…
dialog-unsaved-title = { $name } への変更を保存しますか？
dialog-unsaved-body = 保存しないと、変更内容は失われます。
dialog-conflict-title = { $name } はディスク上で変更されました
dialog-conflict-body = このファイルを開いた後に、別のプログラムが変更しました。上書きするとその変更は失われます。
dialog-readonly-title = { $name } は読み出し専用です
dialog-readonly-body = 別の名前でコピーを保存してください。
error-open-encoding = { $name } は UTF-8 または UTF-16 のテキストではありません。ファイルは開かれず、内容は変更されていません。「ファイル > エンコーディングを指定して再度開く」でエンコーディングを選択してください。
error-save = { $name } を保存できませんでした: { $reason }
menu-window = ウインドウ
edit-cut = カット
edit-copy = コピー
edit-paste = ペースト
app-quit = Sanain を終了
app-hide = Sanain を非表示
app-hide-others = ほかを非表示
app-show-all = すべてを表示
window-minimize = しまう
window-zoom = 拡大/縮小
status-recovered = 前回のセッションの未保存の変更を復元しました
status-recovery-failed = 復元用コピーを保存できませんでした: { $reason }
edit-duplicate-line = 行を複製
edit-delete-line = 行を削除
edit-move-line-up = 行を上へ移動
edit-move-line-down = 行を下へ移動
file-close-tab = タブを閉じる
file-reopen-tab = 閉じたタブを再度開く
window-next-tab = 次のタブ
window-prev-tab = 前のタブ
tab-close = タブを閉じる
edit-find = 検索…
edit-find-next = 次を検索
edit-find-prev = 前を検索
edit-replace = 置換…
find-placeholder = 検索
replace-placeholder = 置換後の文字列
find-match-case = 大文字と小文字を区別
find-whole-word = 単語単位
find-regex = 正規表現
find-no-results = 結果なし
find-count = { $total ->
   *[other] { $total } 件一致
    }
find-count-current = { $current } / { $total }
find-prev = 前の一致 (Shift+Enter)
find-next = 次の一致 (Enter)
find-close = 閉じる (Esc)
replace-one = 置換
replace-all = すべてを置換
status-replaced = { $count ->
   *[other] { $count } 件を置換しました
    }
language-plain = プレーンテキスト
menu-view = 表示
app-settings = 設定…
view-zoom-in = 拡大
view-zoom-out = 縮小
view-zoom-reset = 実際のサイズ
settings-title = 設定
settings-font-size = フォントサイズ
settings-tab-width = タブ幅
settings-insert-spaces = Tab キーでスペースを挿入
settings-theme = 外観
settings-theme-system = システムに合わせる
settings-theme-light = ライト
settings-theme-dark = ダーク
settings-restore-session = 前回のセッションのファイルを再度開く
settings-language = 言語
settings-language-system = システムに合わせる
settings-language-note = 言語の変更は Sanain の再起動後に反映されます。
settings-save-failed = 設定を保存できませんでした: { $reason }
view-command-palette = コマンドパレット…
palette-placeholder = コマンドを入力
view-markdown-source = Markdown ソース
view-markdown-split = Markdown ソースとプレビュー
view-markdown-preview = Markdown プレビュー
preview-too-large = この書類は大きすぎるためプレビューできません。
preview-mermaid-unsupported = Mermaid 図はまだ描画されません。ソースを表示しています。
menu-tools = ツール
tools-json-validate = JSON を検証
tools-json-format = JSON を整形
tools-json-minify = JSON を圧縮
tools-sort-lines = 行を並べ替え
tools-sort-lines-desc = 行を降順で並べ替え
tools-unique-lines = 重複行を削除
tools-trim-trailing = 行末の空白を削除
tools-upper-case = 大文字に変換
tools-lower-case = 小文字に変換
tools-to-lf = 改行コードを LF に変換
tools-to-crlf = 改行コードを CRLF に変換
tools-json-valid = JSON は有効です
tools-done = 完了しました。取り消すと元に戻せます。
tools-no-change = 変更する箇所はありません
tools-json-error = 無効な JSON（{ $line } 行、{ $column } 列）: { $reason }
json-reason-unterminated-string = 文字列が閉じられていません
json-reason-invalid-u-escape = 無効な \u エスケープ
json-reason-invalid-escape = 無効なエスケープ
json-reason-control-character-in-string = 文字列内に制御文字があります
json-reason-invalid-number = 無効な数値
json-reason-unexpected-character = 予期しない文字
json-reason-unexpected-content-after-the-value = 値の後に予期しない内容があります
json-reason-unexpected-comma = 予期しないコンマ
json-reason-expected-a-string-key = 文字列のキーが必要です
json-reason-expected = ':' が必要です
json-reason-expected-or-a-closing-bracket = ',' または閉じ括弧が必要です
json-reason-expected-a-value = 値が必要です
json-reason-empty-document = 空の書類
json-reason-unexpected-end-of-input = 入力が途中で終了しています
json-reason-too-deeply-nested-to-format = ネストが深すぎて整形できません
tools-compare = 比較…
compare-need-two = 比較するには別のタブを開いてください。
compare-pick-title = { $name } と比較する対象
compare-count = { $count ->
    [0] 差分なし
   *[other] { $count } 件の変更
    }
compare-close = 比較を閉じる
compare-to-left = この変更を左側にコピー
compare-to-right = この変更を右側にコピー
compare-stale = 比較中に書類が変更されました。比較を更新しました。もう一度実行してください。
view-structured = 構造化表示 (CSV, JSON)
csv-delimiter = 区切り文字
csv-rows = { $count ->
   *[other] { $count } 行
    }
csv-unterminated = { $line } 行目から始まる引用符が閉じられていません
csv-truncated = 最初の 200,000 行を表示しています
json-more = … ほか { $count } 件
dialog-open-link-title = このリンクを Web ブラウザで開きますか？
button-open = 開く
link-blocked = 安全のため、この種類のリンクは開きません。
link-not-found = リンク先のファイルが見つかりません: { $target }
enc-windows-1252 = 西ヨーロッパ言語
enc-iso-8859-15 = 西ヨーロッパ言語
enc-windows-1250 = 中央ヨーロッパ言語
enc-iso-8859-2 = 中央ヨーロッパ言語
enc-windows-1251 = キリル文字
enc-koi8-r = キリル文字
enc-windows-1253 = ギリシャ語
enc-windows-1254 = トルコ語
enc-windows-1255 = ヘブライ語
enc-windows-1256 = アラビア語
enc-windows-1257 = バルト言語
enc-windows-874 = タイ語
enc-shift_jis = 日本語
enc-euc-jp = 日本語
enc-gb18030 = 簡体中国語
enc-big5 = 繁体中国語
enc-euc-kr = 韓国語
file-reopen-encoding = エンコーディングを指定して再度開く…
file-save-encoding = エンコーディングを指定して保存…
encoding-reopen-title = エンコーディングを指定して再度開く
encoding-save-title = エンコーディングを指定して保存
encoding-not-exact = このエンコーディングでは、ファイルを変更せずに開くことはできません: { $reason }
encoding-unencodable = “{ $character }” は { $encoding } で保存できません。別のエンコーディングを選択するか、テキストを変更してください。何も書き込まれていません。
encoding-save-first = 別のエンコーディングで再度開く前に、変更を保存するか取り消してください。
image-remote = リモート画像は読み込まれません。
image-blocked = この書類と同じフォルダ内の画像のみ表示されます。
image-unsupported = サポートされていない画像形式です。
image-too-large = 画像が大きすぎるため表示できません。
image-unreadable = 画像を読み込めませんでした。
settings-theme-high-contrast = ハイコントラスト
view-fold = 折りたたむ
view-unfold = 展開
view-unfold-all = すべて展開
settings-shortcuts = キーボードショートカット
settings-keymap-preset = プリセット
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = 変更
settings-shortcut-reset = リセット
settings-shortcut-press = ショートカットを押してください… (Esc でキャンセル)
settings-shortcut-moved = ショートカットの移動元:
edit-find-in-folder = フォルダ内を検索…
folder-search-placeholder = フォルダ内を検索
folder-search-run = 検索
folder-search-stop = 停止
folder-search-no-folder = フォルダを選択
folder-search-summary = { $count ->
   *[other] { $count } 件一致
    }（{ $files ->
   *[other] { $files } 個のファイル
    }）
app-check-updates = アップデートを確認…
update-checking = アップデートを確認しています…
update-up-to-date = Sanain { $version } は最新バージョンです。
update-no-releases = Sanain のリリースはまだ公開されていません。
update-failed = アップデートを確認できませんでした: { $reason }
update-invalid = 予期しない応答
update-available-title = Sanain { $version } が利用可能です
update-available-body = ダウンロードページを開いて入手してください。Sanain が自動でダウンロードやインストールを行うことはありません。
update-open-page = ダウンロードページを開く
update-later = 後で
settings-check-updates = 1日に1回アップデートを確認
settings-check-updates-detail = github.com に最新リリースを問い合わせます。それ以外の情報は送信されません。
file-local-history = ローカル履歴…
file-clear-history = ローカル履歴を消去…
history-title = { $name } のローカル履歴
history-open = 開く
history-restore = 復元
history-empty = 保存されたバージョンはまだありません。保存するたびにコピーが保持されます。
history-unreadable = このバージョンを読み込めませんでした。
history-just-now = たった今
history-minutes-ago = { $count ->
   *[other] { $count } 分前
    }
history-hours-ago = { $count ->
   *[other] { $count } 時間前
    }
history-days-ago = { $count ->
   *[other] { $count } 日前
    }
history-purge-title = ローカル履歴をすべて削除しますか？
history-purge-body = すべてのファイルの保存済みコピーがこのコンピュータから削除されます。ファイル自体は変更されません。
history-purge-confirm = 履歴を削除
history-purged = ローカル履歴を削除しました。
settings-local-history = 保存したファイルのローカル履歴を保持
settings-local-history-detail = ファイルごとに最大 50 バージョンを 30 日間、このコンピュータ上にのみ保持します。
file-open-log = ログビューアで開く…
log-lines = { $count ->
   *[other] { $count } 行
    }
log-indexing = 読み込み中… { $percent }%
log-matches = { $count ->
   *[other] 一致する行: { $count }
    }
log-filter-hint = 行を絞り込む
log-bad-pattern = 無効なパターン
log-follow = 追従
log-close = ビューアを閉じる
log-opened-large = { $name } は大きいため、ログビューアで読み出し専用で開きました。
tab-pin = タブを固定
tab-unpin = タブの固定を解除
