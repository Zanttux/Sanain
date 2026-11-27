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
preview-mermaid-unsupported-type = この種類の図には対応していません。ソースを表示しています。
preview-mermaid-unsupported-syntax = 描画できない構文が図に含まれています。ソースを表示しています。
preview-mermaid-too-large = 図が大きすぎて描画できません。ソースを表示しています。
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
view-structured = 構造化表示 (CSV, JSON, XML, YAML)
csv-delimiter = 区切り文字
csv-rows = { $count ->
   *[other] { $count } 行
    }
csv-unterminated = { $line } 行目から始まる引用符が閉じられていません
csv-truncated = 最初の 200,000 行を表示しています
csv-edit-cell = セルを編集
csv-insert-row-below = 下に行を挿入
csv-delete-row = 行を削除
csv-read-only = 読み取り専用ファイル: グリッドは編集できません
json-more = … ほか { $count } 件
json-edit-value = 値を編集
json-rename-key = キー名を変更
json-add-after = 後ろに追加
json-add-inside = 中に追加
json-delete-node = 削除
json-read-only = 読み取り専用ファイル: ツリーは編集できません
json-invalid-value = 有効な JSON 値ではありません
json-duplicate-key = オブジェクトには同じ名前のキーが既にあります
xml-edit-value = 値を編集
xml-edit-text = テキストを編集
xml-rename = 名前を変更
xml-add-attribute = 属性を追加
xml-add-child = 子要素を追加
xml-add-after = 後ろに追加
xml-delete = 削除
xml-invalid-name = 有効な XML 名ではありません
xml-duplicate-attribute = この要素には同じ名前の属性が既にあります
xml-not-well-formed = この変更を行うと XML が無効になります
yaml-not-preserved = この変更は YAML の他の部分も変えてしまいます
yaml-key-too-long = キーは 1024 文字までです
yaml-read-only-flow = フロースタイル（括弧内の記述）はまだツリーで編集できません
yaml-read-only-anchor = アンカー (&) 付きの値はツリーで編集できません
yaml-read-only-tag = タグ (!) 付きの値はツリーで編集できません
yaml-read-only-alias = エイリアス (*) はツリーで編集できません
yaml-read-only-block-scalar = テキストブロック (| または >) はまだツリーで編集できません
yaml-read-only-complex-key = 複合キーはツリーで編集できません
yaml-read-only-document = ツリーで編集できるのは最初のドキュメントだけです
yaml-read-only-layout = この項目は他の内容と行を共有しているため、ここでは変更できません
yaml-read-only-only-child = 唯一の項目は削除できません
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
edit-add-cursor-above = 上にカーソルを追加
edit-add-cursor-below = 下にカーソルを追加
view-word-wrap = 折り返し
status-wrap-on = 長い行をウィンドウの幅で折り返すようになりました。
status-wrap-off = 長い行を折り返さなくなりました。
settings-word-wrap = 長い行を折り返す
edit-go-to-line = 行へ移動…
edit-go-to-heading = 見出しへ移動…
goto-line-hint = 行 または 行:列
goto-line-note = { $count ->
   *[other] この文書は { $count } 行です。
    }
goto-line-invalid = 行番号を入力してください（必要に応じて :列 を付けます）。
goto-heading-hint = 入力して見出しを絞り込む
goto-heading-none = この文書には見出しがありません。
format-xml = XML
format-yaml = YAML
tools-tree-error = 無効な { $format }（{ $line } 行、{ $column } 列）: { $reason }
edit-select-next-occurrence = 次の出現箇所を選択
tools-record-macro = マクロの記録を開始/停止
tools-play-macro = マクロを再生
macro-recording = マクロを記録中
macro-recorded = { $count ->
   *[other] マクロを記録しました: { $count } ステップ。
    }
tools-compare-saved = 保存済みのバージョンと比較
tools-compare-clipboard = クリップボードと比較
compare-saved-label = { $name }（ディスク上）
compare-clipboard-label = クリップボード
compare-saved-failed = 保存済みのファイルを読み込めませんでした: { $reason }
compare-clipboard-empty = クリップボードに比較するテキストがありません。
error-save-disk-full = { $name } を保存できませんでした: ディスクがいっぱいです。ディスク上のファイルは変更されておらず、テキストはこのまま残っています。空き容量を確保するか、「別名で保存」で別の場所に保存してください。
error-save-permission = { $name } を保存できませんでした: その場所に書き込む権限がありません。ディスク上のファイルは変更されていません。「別名で保存」で別の場所に保存してください。
error-save-read-only-volume = { $name } を保存できませんでした: ディスクは読み取り専用です。ディスク上のファイルは変更されていません。「別名で保存」で別の場所に保存してください。
replace-preview-title = { $count ->
   *[other] { $count } 件を置換しますか？
    }
replace-preview-confirm = { $count ->
   *[other] { $count } 件を置換
    }
replace-preview-more = 変更される最初の 200 行のみを表示しています。
replace-preview-stale = ドキュメントが変更されたため、何も置換されませんでした。もう一度「すべてを置換」を実行してください。
encoding-detected = { $encoding } で開きました（ファイルの内容から検出）。
encoding-suggested = ファイルの内容から推定: { $encoding }。開く前にプレビューを確認してください。
app-about = Sanain について
menu-help = ヘルプ
about-tagline = 言葉の力。
about-version = バージョン { $version }
about-license = Mozilla Public License 2.0 に基づく自由なオープンソースソフトウェアです。
about-ai = AI が作り、人が導き、コミュニティが育てる。
about-notices = サードパーティ製コンポーネントとそのライセンスは、各リリースに含まれる THIRD_PARTY_NOTICES に記載されています。
about-source = GitHub のソースコードと Issue
tools-xml-validate = XML を検証
tools-yaml-validate = YAML を検証
tools-markup-valid = { $format } は有効です
file-open-recent = 最近使った項目を開く…
file-save-all = すべてを保存
file-revert = 保存された状態に戻す
recent-hint = 入力して最近使ったファイルを絞り込み
recent-none = 最近使ったファイルはまだありません。
revert-title = { $name } への変更を破棄しますか?
revert-body = ファイルをディスクから再読み込みします。保存されていない変更は失われ、取り消すことはできません。
revert-confirm = 破棄して再読み込み
revert-done = { $name } をディスクから再読み込みしました。
status-saved-all = { $count ->
    [0] 保存するものはありません。
   *[other] { $count } 個のファイルを保存しました。
    }
edit-toggle-comment = 行コメントの切り替え
edit-indent = 行のインデント
edit-outdent = 行のインデント解除
comment-unsupported = このファイル形式には行コメントがありません。
tab-close-others = 他のタブを閉じる
tab-close-all = すべてのタブを閉じる
file-copy-path = ファイルパスをコピー
file-reveal-macos = Finderで表示
file-reveal-windows = エクスプローラーで表示
file-reveal-other = ファイルマネージャーで表示
status-path-copied = ファイルパスをコピーしました。
tabs-kept-unsaved = { $count ->
   *[other] 未保存の変更があるタブ { $count } 個を開いたままにしました。
    }
status-indent-tabs = タブ
status-indent-spaces = スペース: { $count }
find-searching = 検索中…
compare-ignore-whitespace = 空白を無視
tools-compare-file = ファイルと比較…
compare-file-failed = { $name } と比較できませんでした: { $reason }
cli-help =
    使い方:
      sanain [ファイル…]     ファイルを開く
      sanain --diff 左 右    2 つのファイルを並べて比較
      sanain --version       バージョンを表示
      sanain --help          このヘルプを表示
    ダッシュで始まるファイル名の前には -- を付けてください。
cli-unknown-option = 不明なオプション { $option }。
cli-diff-needs-two = --diff には 2 つのファイルが必要です: sanain --diff 左 右
window-move-tab-left = タブを左に移動
window-move-tab-right = タブを右に移動
edit-skip-occurrence = 次の出現箇所へスキップ
tools-play-macro-times = マクロを指定回数再生…
macro-times-hint = 回数
macro-times-note = 1～{ $max }
macro-times-invalid = 1～{ $max } の数値を入力してください。
view-outline = アウトライン
outline-empty = このドキュメントには見出しがありません。
outline-close = アウトラインを閉じる
folder-replace-placeholder = 置換後の文字列
folder-replace-run = ファイル内で置換…
replace-files-title = { $files ->
   *[other] { $files } 個のファイル
    }で { $count ->
   *[other] { $count } 件
    }を置換しますか？
replace-files-body = タブで開いているファイルはタブ内で変更され、元に戻すことができます。これらはご自身で保存してください。その他のファイルはすぐに保存され、ローカル履歴が有効な場合は以前の内容がローカル履歴に保存されます。その間にディスク上で変更されたファイルはスキップされます。
replace-files-confirm = 置換
replace-files-done = { $files ->
   *[other] { $files } 個のファイル
    }で { $count ->
   *[other] { $count } 件
    }を置換しました（タブで開いているもの: { $open }、未保存）。
replace-files-failed = { $failed ->
   *[other] { $failed } 個のファイルは変更されませんでした。最初のファイル:
    } { $first }
image-outside = この画像は書類のフォルダの外にあります。
image-allow = このフォルダの画像を表示
cli-help-integration =
    ファイルマネージャーとの統合（現在のユーザーのみ）:
      sanain --register           Sanain を「プログラムから開く」に追加（Linux、Windows）
      sanain --unregister         再び削除
cli-registered = Sanain を「プログラムから開く」に追加しました。{ $place }
cli-unregistered = Sanain を「プログラムから開く」から削除しました。
cli-register-failed = ファイルマネージャーとの統合を変更できませんでした: { $reason }
cli-register-macos = macOS では登録は不要です。Sanain.app を「アプリケーション」フォルダに入れると、Finder の「プログラムから開く」に表示されます。
settings-tab-general = 一般
settings-tab-editor = エディタ
settings-tab-appearance = 外観
settings-shortcut-search = コマンドまたはキーを検索
settings-font = フォント
settings-font-default = 内蔵の等幅フォント
settings-font-loading = インストール済みのフォントを検索しています…
settings-font-missing = フォント { $font } が見つかりません。内蔵フォントを使用します。
settings-theme-more = その他のテーマ
settings-theme-user-note = 独自のテーマ: フォルダ { $folder } 内の .theme ファイル（docs/THEMES.md を参照）。
language-auto = 自動検出
language-choose = このタブの言語を選択
tools-xml-format = XML を整形
tools-save-macro = マクロを保存…
tools-play-saved-macro = 保存したマクロを再生…
tools-delete-saved-macro = 保存したマクロを削除…
macro-save-hint = マクロ名
macro-save-note = 同じ名前の保存済みマクロは置き換えられます。
macro-save-empty = 保存するものがありません。先にマクロを記録してください。
macro-saved = マクロを保存しました: { $name }
macro-deleted = マクロを削除しました: { $name }
macro-name-invalid = 1～{ $max } 文字の名前を 1 行で入力してください。
macro-save-too-large = このマクロは長すぎて保存できません。
macro-save-full = 保存できるマクロは最大 { $max } 個です。先にいずれかを削除してください。
macro-save-failed = マクロを保存できませんでした: { $error }
macro-saved-hint = 入力して保存したマクロを絞り込み
macro-saved-none = 保存したマクロはまだありません。
settings-shortcut-import-npp = Notepad++ からインポート…
settings-shortcut-import-npp-hint = Notepad++ の shortcuts.xml を選択してください。Windows では %AppData%\Notepad++\shortcuts.xml にあります。
settings-shortcut-import-npp-done = Notepad++ からインポート: { $imported }。スキップ: { $skipped }。残っている競合: { $conflicts }。
settings-shortcut-import-npp-failed = { $name } をインポートできませんでした: { $reason }
settings-shortcut-import-npp-too-large = ファイルが 1 MiB を超えています。
settings-shortcut-import-npp-invalid = このファイルは有効な Notepad++ の shortcuts.xml ではありません。
view-split-right = 右に分割
view-split-down = 下に分割
view-close-pane = ペインを閉じる
view-focus-next-pane = 次のペインにフォーカス
view-focus-prev-pane = 前のペインにフォーカス
view-focus-next-region = 次の領域にフォーカス
view-focus-prev-region = 前の領域にフォーカス
status-pane-limit = ペインの上限に達しました（最大 { $max }）。
a11y-csv-grid = CSV 表
a11y-grid-cell = { $row } 行目、{ $column } 列目: { $name }
a11y-grid-cell-plain = { $row } 行目、{ $column } 列目
a11y-grid-column-header = { $column } 列目の見出し
a11y-grid-row-header = { $row } 行目
a11y-edit-field = 編集中: { $item }
a11y-json-tree = JSON ツリー
a11y-xml-tree = XML ツリー
a11y-yaml-tree = YAML ツリー
a11y-tree-item = レベル { $level }
a11y-tree-item-expanded = レベル { $level }、展開
a11y-tree-item-collapsed = レベル { $level }、折りたたみ
a11y-tree-object = オブジェクト、項目数: { $count }
a11y-tree-list = リスト、要素数: { $count }
a11y-tree-element = 要素 { $name }
a11y-tree-attribute = 属性 { $name }
a11y-tree-text = テキスト
a11y-pane = ペイン { $pane }/{ $count }: { $name }
a11y-pane-active = ペイン { $pane }/{ $count }: { $name }、アクティブ
a11y-pane-editor = テキストエディタ、ペイン { $pane }/{ $count }: { $name }
a11y-mermaid-flowchart = フローチャート。ノード: { $nodes }、リンク: { $links }。
a11y-mermaid-sequence = シーケンス図。参加者: { $participants }、メッセージ: { $messages }。
a11y-mermaid-state = 状態遷移図。状態: { $states }、遷移: { $transitions }。
a11y-mermaid-class = クラス図。クラス: { $classes }、関係: { $relations }。
a11y-shortcut-change = { $command } のショートカットを変更
a11y-shortcut-change-current = { $command } のショートカットを変更、現在 { $keys }
a11y-shortcut-reset = { $command } のショートカットをリセット
status-line-ending-choose = このファイルの改行コードを変更
status-encoding-choose = 別のエンコーディングで開き直すか保存
status-indent-choose = インデント設定
a11y-settings-sections = 設定のセクション
a11y-tab-strip = 開いているファイル
a11y-tab-unsaved = { $name }、未保存の変更あり
