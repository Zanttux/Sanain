# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = 文件
file-open = 打开…
file-save = 存储
status-tabs = { $count ->
   *[other] { $count } 个标签页
    }
error-open = 无法打开“{ $name }”。
menu-edit = 编辑
file-new = 新建标签页
file-save-as = 存储为…
edit-undo = 撤销
edit-redo = 重做
edit-select-all = 全选
untitled = 未命名
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = 行 { $line }，列 { $column }
status-selections = { $count ->
   *[other] { $count } 个选区
    }
status-saved = 已存储“{ $name }”
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8（带 BOM）
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = 混合换行符
editor-accessible-name = 文本编辑器
button-save = 存储
button-dont-save = 不存储
button-cancel = 取消
button-ok = 好
button-overwrite = 覆盖
button-save-as = 存储为…
dialog-unsaved-title = 要存储对“{ $name }”的更改吗？
dialog-unsaved-body = 如果不存储，你的更改将会丢失。
dialog-conflict-title = “{ $name }”已在磁盘上更改
dialog-conflict-body = 你打开此文件后，另一个程序修改了它。覆盖将替换这些更改。
dialog-readonly-title = “{ $name }”为只读
dialog-readonly-body = 请改用其他名称存储副本。
error-open-encoding = “{ $name }”不是 UTF-8 或 UTF-16 文本。文件未打开，因此其内容未更改。请使用“文件 > 以指定编码重新打开”选择编码。
error-save = 无法存储“{ $name }”：{ $reason }
menu-window = 窗口
edit-cut = 剪切
edit-copy = 拷贝
edit-paste = 粘贴
app-quit = 退出 Sanain
app-hide = 隐藏 Sanain
app-hide-others = 隐藏其他
app-show-all = 全部显示
window-minimize = 最小化
window-zoom = 缩放
status-recovered = 已恢复上次会话中未存储的更改
status-recovery-failed = 无法存储恢复副本：{ $reason }
edit-duplicate-line = 复制行
edit-delete-line = 删除行
edit-move-line-up = 上移行
edit-move-line-down = 下移行
file-close-tab = 关闭标签页
file-reopen-tab = 重新打开已关闭的标签页
window-next-tab = 下一个标签页
window-prev-tab = 上一个标签页
tab-close = 关闭标签页
edit-find = 查找…
edit-find-next = 查找下一个
edit-find-prev = 查找上一个
edit-replace = 替换…
find-placeholder = 查找
replace-placeholder = 替换为
find-match-case = 区分大小写
find-whole-word = 全字匹配
find-regex = 正则表达式
find-no-results = 无结果
find-count = { $total ->
   *[other] { $total } 个匹配项
    }
find-count-current = 第 { $current } 个，共 { $total } 个
find-prev = 上一个匹配项 (Shift+Enter)
find-next = 下一个匹配项 (Enter)
find-close = 关闭 (Esc)
replace-one = 替换
replace-all = 全部替换
status-replaced = { $count ->
   *[other] 已替换 { $count } 个匹配项
    }
language-plain = 纯文本
menu-view = 显示
app-settings = 设置…
view-zoom-in = 放大
view-zoom-out = 缩小
view-zoom-reset = 实际大小
settings-title = 设置
settings-font-size = 字体大小
settings-tab-width = 制表符宽度
settings-insert-spaces = 按 Tab 键时插入空格
settings-theme = 外观
settings-theme-system = 跟随系统
settings-theme-light = 浅色
settings-theme-dark = 深色
settings-restore-session = 重新打开上次会话中的文件
settings-language = 语言
settings-language-system = 跟随系统
settings-language-note = 语言更改将在重新启动 Sanain 后生效。
settings-save-failed = 无法存储设置：{ $reason }
view-command-palette = 命令面板…
palette-placeholder = 输入命令
view-markdown-source = Markdown 源代码
view-markdown-split = Markdown 源代码和预览
view-markdown-preview = Markdown 预览
preview-too-large = 此文稿过大，无法预览。
preview-mermaid-unsupported = 尚不支持渲染 Mermaid 图表；正在显示源代码。
menu-tools = 工具
tools-json-validate = 验证 JSON
tools-json-format = 格式化 JSON
tools-json-minify = 压缩 JSON
tools-sort-lines = 排序行
tools-sort-lines-desc = 降序排序行
tools-unique-lines = 移除重复行
tools-trim-trailing = 删除行尾空白
tools-upper-case = 转为大写
tools-lower-case = 转为小写
tools-to-lf = 将换行符转换为 LF
tools-to-crlf = 将换行符转换为 CRLF
tools-json-valid = JSON 有效
tools-done = 完成。撤销可还原。
tools-no-change = 无需更改
tools-json-error = JSON 无效，位于第 { $line } 行第 { $column } 列：{ $reason }
json-reason-unterminated-string = 字符串未结束
json-reason-invalid-u-escape = 无效的 \u 转义
json-reason-invalid-escape = 无效的转义
json-reason-control-character-in-string = 字符串中含有控制字符
json-reason-invalid-number = 无效的数字
json-reason-unexpected-character = 意外的字符
json-reason-unexpected-content-after-the-value = 值后有意外内容
json-reason-unexpected-comma = 意外的逗号
json-reason-expected-a-string-key = 应为字符串键
json-reason-expected = 应为 ':'
json-reason-expected-or-a-closing-bracket = 应为 ',' 或右括号
json-reason-expected-a-value = 应为值
json-reason-empty-document = 空文稿
json-reason-unexpected-end-of-input = 输入意外结束
json-reason-too-deeply-nested-to-format = 嵌套过深，无法格式化
tools-compare = 比较…
compare-need-two = 请打开另一个标签页进行比较。
compare-pick-title = 将“{ $name }”与以下项比较
compare-count = { $count ->
    [0] 无差异
   *[other] { $count } 处更改
    }
compare-close = 关闭比较
compare-to-left = 将此更改拷贝到左侧
compare-to-right = 将此更改拷贝到右侧
compare-stale = 比较期间文稿已更改。比较已刷新；请重试。
view-structured = 结构化视图 (CSV, JSON, XML, YAML)
csv-delimiter = 分隔符
csv-rows = { $count ->
   *[other] { $count } 行
    }
csv-unterminated = 从第 { $line } 行开始的引号未闭合
csv-truncated = 仅显示前 200,000 行
json-more = … 还有 { $count } 项
dialog-open-link-title = 要在网页浏览器中打开此链接吗？
button-open = 打开
link-blocked = 为安全起见，不会打开此类链接。
link-not-found = 找不到链接的文件：{ $target }
enc-windows-1252 = 西欧语言
enc-iso-8859-15 = 西欧语言
enc-windows-1250 = 中欧语言
enc-iso-8859-2 = 中欧语言
enc-windows-1251 = 西里尔文
enc-koi8-r = 西里尔文
enc-windows-1253 = 希腊语
enc-windows-1254 = 土耳其语
enc-windows-1255 = 希伯来语
enc-windows-1256 = 阿拉伯语
enc-windows-1257 = 波罗的海语言
enc-windows-874 = 泰语
enc-shift_jis = 日语
enc-euc-jp = 日语
enc-gb18030 = 简体中文
enc-big5 = 繁体中文
enc-euc-kr = 韩语
file-reopen-encoding = 以指定编码重新打开…
file-save-encoding = 以指定编码存储…
encoding-reopen-title = 以指定编码重新打开
encoding-save-title = 以指定编码存储
encoding-not-exact = 使用此编码无法在不更改文件的情况下打开它：{ $reason }
encoding-unencodable = “{ $character }”无法以 { $encoding } 存储。请选择其他编码或修改文本；未写入任何内容。
encoding-save-first = 以其他编码重新打开前，请先存储或撤销你的更改。
image-remote = 不加载远程图像。
image-blocked = 仅显示此文稿所在文件夹中的图像。
image-unsupported = 不支持的图像格式。
image-too-large = 图像过大，无法显示。
image-unreadable = 无法读取图像。
settings-theme-high-contrast = 高对比度
view-fold = 折叠
view-unfold = 展开
view-unfold-all = 全部展开
settings-shortcuts = 键盘快捷键
settings-keymap-preset = 预设
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = 更改
settings-shortcut-reset = 还原
settings-shortcut-press = 请按下快捷键…（按 Esc 取消）
settings-shortcut-moved = 快捷键原先分配给：
edit-find-in-folder = 在文件夹中查找…
folder-search-placeholder = 在文件夹中查找
folder-search-run = 搜索
folder-search-stop = 停止
folder-search-no-folder = 选择文件夹
folder-search-summary = { $files ->
   *[other] { $files } 个文件中
    }共 { $count ->
   *[other] { $count } 个匹配项
    }
app-check-updates = 检查更新…
update-checking = 正在检查更新…
update-up-to-date = Sanain { $version } 是最新版本。
update-no-releases = Sanain 尚未发布任何版本。
update-failed = 无法检查更新：{ $reason }
update-invalid = 意外的响应
update-available-title = Sanain { $version } 现已推出
update-available-body = 请打开下载页面获取。Sanain 不会自行下载或安装任何内容。
update-open-page = 打开下载页面
update-later = 稍后
settings-check-updates = 每天检查一次更新
settings-check-updates-detail = 向 github.com 查询最新版本。不会发送其他任何信息。
file-local-history = 本地历史记录…
file-clear-history = 清除本地历史记录…
history-title = { $name } 的本地历史记录
history-open = 打开
history-restore = 恢复
history-empty = 尚无已存储的版本。每次存储时都会保留一份副本。
history-unreadable = 无法读取此版本。
history-just-now = 刚刚
history-minutes-ago = { $count ->
   *[other] { $count } 分钟前
    }
history-hours-ago = { $count ->
   *[other] { $count } 小时前
    }
history-days-ago = { $count ->
   *[other] { $count } 天前
    }
history-purge-title = 要删除全部本地历史记录吗？
history-purge-body = 所有文件的已存储副本将从此电脑中移除。文件本身不会被更改。
history-purge-confirm = 删除历史记录
history-purged = 已删除本地历史记录。
settings-local-history = 保留已存储文件的本地历史记录
settings-local-history-detail = 每个文件最多保留 50 个版本，保留 30 天，仅存于此电脑。
file-open-log = 在日志查看器中打开…
log-lines = { $count ->
   *[other] { $count } 行
    }
log-indexing = 正在读取… { $percent }%
log-matches = { $count ->
   *[other] { $count } 行匹配
    }
log-filter-hint = 筛选行
log-bad-pattern = 无效的模式
log-follow = 跟随
log-close = 关闭查看器
log-opened-large = “{ $name }”较大，因此已在日志查看器中以只读方式打开。
tab-pin = 固定标签页
tab-unpin = 取消固定标签页
edit-add-cursor-above = 在上方添加光标
edit-add-cursor-below = 在下方添加光标
view-word-wrap = 自动换行
status-wrap-on = 长行现在会按窗口宽度自动换行。
status-wrap-off = 长行不再自动换行。
settings-word-wrap = 长行自动换行
edit-go-to-line = 转到行…
edit-go-to-heading = 转到标题…
goto-line-hint = 行 或 行:列
goto-line-note = { $count ->
   *[other] 文档共有 { $count } 行。
    }
goto-line-invalid = 请输入行号，可选择在后面加上 :列。
goto-heading-hint = 输入以筛选标题
goto-heading-none = 此文档没有标题。
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } 无效，位于第 { $line } 行第 { $column } 列：{ $reason }
edit-select-next-occurrence = 选择下一个匹配项
tools-record-macro = 开始或停止录制宏
tools-play-macro = 回放宏
macro-recording = 正在录制宏
macro-recorded = { $count ->
   *[other] 宏已录制：{ $count } 步。
    }
tools-compare-saved = 与已存储的版本比较
tools-compare-clipboard = 与剪贴板比较
compare-saved-label = “{ $name }”（磁盘上）
compare-clipboard-label = 剪贴板
compare-saved-failed = 无法读取已存储的文件：{ $reason }
compare-clipboard-empty = 剪贴板中没有可比较的文本。
error-save-disk-full = 未保存 { $name }：磁盘已满。磁盘上的文件未被更改，你的文本仍在这里。请释放一些空间，或使用“存储为”将其保存到其他位置。
error-save-permission = 未保存 { $name }：你没有在该位置写入的权限。磁盘上的文件未被更改。请使用“存储为”将其保存到其他位置。
error-save-read-only-volume = 未保存 { $name }：磁盘为只读。磁盘上的文件未被更改。请使用“存储为”将其保存到其他位置。
replace-preview-title = { $count ->
   *[other] 替换 { $count } 个匹配项？
    }
replace-preview-confirm = { $count ->
   *[other] 替换 { $count } 个
    }
replace-preview-more = 仅显示前 200 行更改。
replace-preview-stale = 文档已更改，未替换任何内容。请重新运行“全部替换”。
view-split-editor = 拆分编辑器
encoding-detected = 已按 { $encoding } 打开（根据文件内容检测）。
encoding-suggested = 根据文件内容建议的编码：{ $encoding }。打开前请检查预览。
app-about = 关于 Sanain
menu-help = 帮助
about-tagline = 文字的力量。
about-version = 版本 { $version }
about-license = 基于 Mozilla Public License 2.0 的自由开源软件。
about-ai = AI 构建，人类主导，社区驱动。
about-notices = 第三方组件及其许可证列于 THIRD_PARTY_NOTICES 中，每个版本均附带该文件。
about-source = GitHub 上的源代码和问题
tools-xml-validate = 验证 XML
tools-yaml-validate = 验证 YAML
tools-markup-valid = { $format } 有效
file-open-recent = 打开最近使用…
file-save-all = 全部存储
file-revert = 复原到存储的版本
recent-hint = 键入以过滤最近使用的文件
recent-none = 尚无最近使用的文件。
revert-title = 要放弃对“{ $name }”的更改吗？
revert-body = 将从磁盘重新载入文件。未存储的更改将丢失且无法撤销。
revert-confirm = 放弃并重新载入
revert-done = 已从磁盘重新载入“{ $name }”。
status-saved-all = { $count ->
    [0] 没有需要存储的内容。
   *[other] 已存储 { $count } 个文件。
    }
edit-toggle-comment = 切换行注释
edit-indent = 缩进行
edit-outdent = 减少行缩进
comment-unsupported = 此文件类型不支持行注释。
tab-close-others = 关闭其他标签页
tab-close-all = 关闭所有标签页
file-copy-path = 复制文件路径
file-reveal-macos = 在访达中显示
file-reveal-windows = 在资源管理器中显示
file-reveal-other = 在文件管理器中显示
status-path-copied = 已复制文件路径。
tabs-kept-unsaved = { $count ->
   *[other] 已保留 { $count } 个含未保存更改的标签页。
    }
status-indent-tabs = 制表符
status-indent-spaces = 空格: { $count }
find-searching = 正在搜索…
compare-ignore-whitespace = 忽略空白
tools-compare-file = 与文件比较…
compare-file-failed = 无法与 { $name } 比较：{ $reason }
cli-help =
    用法：
      sanain [文件…]         打开文件
      sanain --diff 左 右    并排比较两个文件
      sanain --version       显示版本
      sanain --help          显示此帮助
    以短横线开头的文件名前请使用 --。
cli-unknown-option = 未知选项 { $option }。
cli-diff-needs-two = --diff 需要两个文件：sanain --diff 左 右
window-move-tab-left = 向左移动标签页
window-move-tab-right = 向右移动标签页
edit-skip-occurrence = 跳到下一个匹配项
tools-play-macro-times = 多次回放宏…
macro-times-hint = 次数
macro-times-note = 1 到 { $max }
macro-times-invalid = 请输入 1 到 { $max } 之间的数字。
view-outline = 大纲
outline-empty = 此文档中没有标题。
outline-close = 关闭大纲
