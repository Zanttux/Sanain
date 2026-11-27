# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Tệp
file-open = Mở…
file-save = Lưu
status-tabs = { $count ->
   *[other] { $count } thẻ
    }
error-open = Không thể mở { $name }.
menu-edit = Sửa
file-new = Thẻ mới
file-save-as = Lưu thành…
edit-undo = Hoàn tác
edit-redo = Làm lại
edit-select-all = Chọn tất cả
untitled = Không tên
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Dòng { $line }, Cột { $column }
status-selections = { $count ->
   *[other] { $count } vùng chọn
    }
status-saved = Đã lưu { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 có BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Kết thúc dòng hỗn hợp
editor-accessible-name = Trình soạn thảo văn bản
button-save = Lưu
button-dont-save = Không lưu
button-cancel = Hủy
button-ok = OK
button-overwrite = Ghi đè
button-save-as = Lưu thành…
dialog-unsaved-title = Lưu thay đổi cho { $name }?
dialog-unsaved-body = Các thay đổi sẽ bị mất nếu bạn không lưu.
dialog-conflict-title = { $name } đã thay đổi trên đĩa
dialog-conflict-body = Một chương trình khác đã sửa tệp này sau khi bạn mở. Ghi đè sẽ thay thế những thay đổi đó.
dialog-readonly-title = { $name } chỉ đọc
dialog-readonly-body = Hãy lưu một bản sao với tên khác.
error-open-encoding = { $name } không phải văn bản UTF-8 hoặc UTF-16. Tệp chưa được mở nên nội dung không thay đổi. Dùng Tệp > Mở lại với mã hóa để chọn mã hóa cho tệp.
error-save = Không thể lưu { $name }: { $reason }
menu-window = Cửa sổ
edit-cut = Cắt
edit-copy = Sao chép
edit-paste = Dán
app-quit = Thoát Sanain
app-hide = Ẩn Sanain
app-hide-others = Ẩn ứng dụng khác
app-show-all = Hiện tất cả
window-minimize = Thu nhỏ
window-zoom = Phóng to
status-recovered = Đã khôi phục các thay đổi chưa lưu từ phiên trước
status-recovery-failed = Không thể lưu bản sao khôi phục: { $reason }
edit-duplicate-line = Nhân đôi dòng
edit-delete-line = Xóa dòng
edit-move-line-up = Di chuyển dòng lên
edit-move-line-down = Di chuyển dòng xuống
file-close-tab = Đóng thẻ
file-reopen-tab = Mở lại thẻ đã đóng
window-next-tab = Thẻ tiếp theo
window-prev-tab = Thẻ trước
tab-close = Đóng thẻ
edit-find = Tìm…
edit-find-next = Tìm tiếp
edit-find-prev = Tìm trước
edit-replace = Thay thế…
find-placeholder = Tìm
replace-placeholder = Thay bằng
find-match-case = Phân biệt chữ hoa/thường
find-whole-word = Nguyên từ
find-regex = Biểu thức chính quy
find-no-results = Không có kết quả
find-count = { $total ->
   *[other] { $total } kết quả khớp
    }
find-count-current = { $current } / { $total }
find-prev = Kết quả trước (Shift+Enter)
find-next = Kết quả tiếp theo (Enter)
find-close = Đóng (Esc)
replace-one = Thay thế
replace-all = Thay thế tất cả
status-replaced = { $count ->
   *[other] Đã thay thế { $count } kết quả khớp
    }
language-plain = Văn bản thuần
menu-view = Xem
app-settings = Cài đặt…
view-zoom-in = Phóng to
view-zoom-out = Thu nhỏ
view-zoom-reset = Kích thước thực
settings-title = Cài đặt
settings-font-size = Cỡ chữ
settings-tab-width = Độ rộng Tab
settings-insert-spaces = Chèn khoảng trắng khi nhấn Tab
settings-theme = Giao diện
settings-theme-system = Theo hệ thống
settings-theme-light = Sáng
settings-theme-dark = Tối
settings-restore-session = Mở lại các tệp từ phiên trước
settings-language = Ngôn ngữ
settings-language-system = Theo hệ thống
settings-language-note = Thay đổi ngôn ngữ có hiệu lực sau khi khởi động lại Sanain.
settings-save-failed = Không thể lưu cài đặt: { $reason }
view-command-palette = Bảng lệnh…
palette-placeholder = Nhập lệnh
view-markdown-source = Mã nguồn Markdown
view-markdown-split = Mã nguồn và xem trước Markdown
view-markdown-preview = Xem trước Markdown
preview-too-large = Tài liệu này quá lớn để xem trước.
preview-mermaid-unsupported-type = Không hỗ trợ loại sơ đồ này; đang hiện mã nguồn.
preview-mermaid-unsupported-syntax = Sơ đồ dùng cú pháp không thể vẽ được; đang hiện mã nguồn.
preview-mermaid-too-large = Sơ đồ quá lớn để vẽ; đang hiện mã nguồn.
menu-tools = Công cụ
tools-json-validate = Kiểm tra JSON
tools-json-format = Định dạng JSON
tools-json-minify = Rút gọn JSON
tools-sort-lines = Sắp xếp dòng
tools-sort-lines-desc = Sắp xếp dòng giảm dần
tools-unique-lines = Xóa dòng trùng lặp
tools-trim-trailing = Cắt khoảng trắng cuối dòng
tools-upper-case = Chữ hoa
tools-lower-case = Chữ thường
tools-to-lf = Chuyển kết thúc dòng sang LF
tools-to-crlf = Chuyển kết thúc dòng sang CRLF
tools-json-valid = JSON hợp lệ
tools-done = Đã xong. Có thể hoàn tác bằng Hoàn tác.
tools-no-change = Không có gì để thay đổi
tools-json-error = JSON không hợp lệ tại dòng { $line }, cột { $column }: { $reason }
json-reason-unterminated-string = chuỗi chưa được đóng
json-reason-invalid-u-escape = chuỗi thoát \u không hợp lệ
json-reason-invalid-escape = chuỗi thoát không hợp lệ
json-reason-control-character-in-string = ký tự điều khiển trong chuỗi
json-reason-invalid-number = số không hợp lệ
json-reason-unexpected-character = ký tự không mong đợi
json-reason-unexpected-content-after-the-value = nội dung không mong đợi sau giá trị
json-reason-unexpected-comma = dấu phẩy không mong đợi
json-reason-expected-a-string-key = cần một khóa chuỗi
json-reason-expected = cần ':'
json-reason-expected-or-a-closing-bracket = cần ',' hoặc dấu ngoặc đóng
json-reason-expected-a-value = cần một giá trị
json-reason-empty-document = tài liệu trống
json-reason-unexpected-end-of-input = kết thúc dữ liệu đầu vào không mong đợi
json-reason-too-deeply-nested-to-format = lồng nhau quá sâu để định dạng
tools-compare = So sánh với…
compare-need-two = Mở một thẻ khác để so sánh.
compare-pick-title = So sánh { $name } với
compare-count = { $count ->
    [0] Không có khác biệt
   *[other] { $count } thay đổi
    }
compare-close = Đóng so sánh
compare-to-left = Sao chép thay đổi này sang trái
compare-to-right = Sao chép thay đổi này sang phải
compare-stale = Một tài liệu đã thay đổi trong khi so sánh. Kết quả so sánh đã được làm mới; hãy thử lại.
view-structured = Chế độ xem có cấu trúc (CSV, JSON, XML, YAML)
csv-delimiter = Dấu phân cách
csv-rows = { $count ->
   *[other] { $count } hàng
    }
csv-unterminated = Dấu ngoặc kép chưa đóng bắt đầu từ dòng { $line }
csv-truncated = Đang hiện 200.000 hàng đầu tiên
csv-edit-cell = Sửa ô
csv-insert-row-below = Chèn hàng bên dưới
csv-delete-row = Xóa hàng
csv-read-only = Tệp chỉ đọc: không thể sửa lưới
json-more = … còn { $count } mục
json-edit-value = Sửa giá trị
json-rename-key = Đổi tên khóa
json-add-after = Thêm phía sau
json-add-inside = Thêm vào bên trong
json-delete-node = Xóa
json-read-only = Tệp chỉ đọc: không thể sửa cây
json-invalid-value = Không phải giá trị JSON hợp lệ
json-duplicate-key = Đối tượng đã có khóa với tên này
xml-edit-value = Sửa giá trị
xml-edit-text = Sửa văn bản
xml-rename = Đổi tên
xml-add-attribute = Thêm thuộc tính
xml-add-child = Thêm phần tử con
xml-add-after = Thêm phía sau
xml-delete = Xóa
xml-invalid-name = Không phải tên XML hợp lệ
xml-duplicate-attribute = Phần tử đã có thuộc tính với tên này
xml-not-well-formed = Thay đổi này sẽ khiến XML không hợp lệ
yaml-not-preserved = Thay đổi này sẽ làm thay đổi các phần khác của YAML
yaml-key-too-long = Khóa chỉ được dài tối đa 1024 ký tự
yaml-read-only-flow = Kiểu flow (viết trong dấu ngoặc) chưa thể chỉnh sửa trong cây
yaml-read-only-anchor = Không thể chỉnh sửa giá trị có anchor (&) trong cây
yaml-read-only-tag = Không thể chỉnh sửa giá trị có tag (!) trong cây
yaml-read-only-alias = Không thể chỉnh sửa alias (*) trong cây
yaml-read-only-block-scalar = Khối văn bản (| hoặc >) chưa thể chỉnh sửa trong cây
yaml-read-only-complex-key = Không thể chỉnh sửa khóa phức hợp trong cây
yaml-read-only-document = Chỉ có thể chỉnh sửa tài liệu đầu tiên trong cây
yaml-read-only-layout = Mục này dùng chung dòng với nội dung khác nên không thể thay đổi ở đây
yaml-read-only-only-child = Không thể xóa mục duy nhất
dialog-open-link-title = Mở liên kết này trong trình duyệt web?
button-open = Mở
link-blocked = Loại liên kết này không được mở vì lý do an toàn.
link-not-found = Không tìm thấy tệp được liên kết: { $target }
enc-windows-1252 = Tây Âu
enc-iso-8859-15 = Tây Âu
enc-windows-1250 = Trung Âu
enc-iso-8859-2 = Trung Âu
enc-windows-1251 = Kirin
enc-koi8-r = Kirin
enc-windows-1253 = Tiếng Hy Lạp
enc-windows-1254 = Tiếng Thổ Nhĩ Kỳ
enc-windows-1255 = Tiếng Do Thái
enc-windows-1256 = Tiếng Ả Rập
enc-windows-1257 = Baltic
enc-windows-874 = Tiếng Thái
enc-shift_jis = Tiếng Nhật
enc-euc-jp = Tiếng Nhật
enc-gb18030 = Tiếng Trung, Giản thể
enc-big5 = Tiếng Trung, Phồn thể
enc-euc-kr = Tiếng Hàn
file-reopen-encoding = Mở lại với mã hóa…
file-save-encoding = Lưu với mã hóa…
encoding-reopen-title = Mở lại với mã hóa
encoding-save-title = Lưu với mã hóa
encoding-not-exact = Mã hóa này không thể mở tệp mà không thay đổi tệp: { $reason }
encoding-unencodable = Không thể lưu “{ $character }” dưới dạng { $encoding }. Hãy chọn mã hóa khác hoặc sửa văn bản; chưa có gì được ghi.
encoding-save-first = Hãy lưu hoặc hoàn tác các thay đổi trước khi mở lại với mã hóa khác.
image-remote = Hình ảnh từ xa không được tải.
image-blocked = Chỉ hiển thị hình ảnh trong thư mục của tài liệu này.
image-unsupported = Định dạng hình ảnh không được hỗ trợ.
image-too-large = Hình ảnh quá lớn để hiển thị.
image-unreadable = Không thể đọc hình ảnh.
settings-theme-high-contrast = Tương phản cao
view-fold = Thu gọn
view-unfold = Mở rộng
view-unfold-all = Mở rộng tất cả
settings-shortcuts = Phím tắt
settings-keymap-preset = Cấu hình sẵn
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Thay đổi
settings-shortcut-reset = Đặt lại
settings-shortcut-press = Nhấn tổ hợp phím… (Esc để hủy)
settings-shortcut-moved = Phím tắt đã được chuyển từ:
edit-find-in-folder = Tìm trong thư mục…
folder-search-placeholder = Tìm trong thư mục
folder-search-run = Tìm kiếm
folder-search-stop = Dừng
folder-search-no-folder = Chọn thư mục
folder-search-summary = { $count ->
   *[other] { $count } kết quả khớp
    } trong { $files ->
   *[other] { $files } tệp
    }
app-check-updates = Kiểm tra bản cập nhật…
update-checking = Đang kiểm tra bản cập nhật…
update-up-to-date = Sanain { $version } là phiên bản mới nhất.
update-no-releases = Chưa có bản phát hành Sanain nào được công bố.
update-failed = Không thể kiểm tra bản cập nhật: { $reason }
update-invalid = phản hồi không mong đợi
update-available-title = Đã có Sanain { $version }
update-available-body = Mở trang tải xuống để tải về. Sanain không tự tải xuống hay cài đặt bất cứ thứ gì.
update-open-page = Mở trang tải xuống
update-later = Để sau
settings-check-updates = Kiểm tra bản cập nhật mỗi ngày một lần
settings-check-updates-detail = Hỏi github.com về bản phát hành mới nhất. Không gửi gì khác.
file-local-history = Lịch sử cục bộ…
file-clear-history = Xóa lịch sử cục bộ…
history-title = Lịch sử cục bộ của { $name }
history-open = Mở
history-restore = Khôi phục
history-empty = Chưa có phiên bản nào được lưu. Mỗi lần bạn lưu, một bản sao sẽ được giữ lại.
history-unreadable = Không thể đọc phiên bản này.
history-just-now = Vừa xong
history-minutes-ago = { $count ->
   *[other] { $count } phút trước
    }
history-hours-ago = { $count ->
   *[other] { $count } giờ trước
    }
history-days-ago = { $count ->
   *[other] { $count } ngày trước
    }
history-purge-title = Xóa toàn bộ lịch sử cục bộ?
history-purge-body = Các bản sao đã lưu của mọi tệp sẽ bị xóa khỏi máy tính này. Bản thân các tệp của bạn không bị thay đổi.
history-purge-confirm = Xóa lịch sử
history-purged = Đã xóa lịch sử cục bộ.
settings-local-history = Giữ lịch sử cục bộ của các tệp đã lưu
settings-local-history-detail = Giữ tối đa 50 phiên bản cho mỗi tệp trong 30 ngày, chỉ trên máy tính này.
file-open-log = Mở trong trình xem nhật ký…
log-lines = { $count ->
   *[other] { $count } dòng
    }
log-indexing = Đang đọc… { $percent }%
log-matches = { $count ->
   *[other] { $count } dòng khớp
    }
log-filter-hint = Lọc dòng
log-bad-pattern = Mẫu không hợp lệ
log-follow = Theo dõi
log-close = Đóng trình xem
log-opened-large = { $name } quá lớn nên đã được mở ở chế độ chỉ đọc trong trình xem nhật ký.
tab-pin = Ghim thẻ
tab-unpin = Bỏ ghim thẻ
edit-add-cursor-above = Thêm con trỏ phía trên
edit-add-cursor-below = Thêm con trỏ phía dưới
view-word-wrap = Ngắt dòng tự động
status-wrap-on = Các dòng dài giờ được ngắt theo chiều rộng cửa sổ.
status-wrap-off = Các dòng dài không còn được ngắt nữa.
settings-word-wrap = Ngắt các dòng dài
edit-go-to-line = Đi tới dòng…
edit-go-to-heading = Đi tới tiêu đề…
goto-line-hint = Dòng hoặc dòng:cột
goto-line-note = { $count ->
   *[other] Tài liệu có { $count } dòng.
    }
goto-line-invalid = Nhập số dòng, có thể kèm theo :cột.
goto-heading-hint = Nhập để lọc tiêu đề
goto-heading-none = Tài liệu này không có tiêu đề.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } không hợp lệ tại dòng { $line }, cột { $column }: { $reason }
edit-select-next-occurrence = Chọn lần xuất hiện tiếp theo
tools-record-macro = Bắt đầu hoặc dừng ghi macro
tools-play-macro = Phát macro
macro-recording = Đang ghi macro
macro-recorded = { $count ->
   *[other] Đã ghi macro: { $count } bước.
    }
tools-compare-saved = So sánh với phiên bản đã lưu
tools-compare-clipboard = So sánh với bảng nhớ tạm
compare-saved-label = { $name } (trên đĩa)
compare-clipboard-label = Bảng nhớ tạm
compare-saved-failed = Không thể đọc tệp đã lưu: { $reason }
compare-clipboard-empty = Bảng nhớ tạm không có văn bản để so sánh.
error-save-disk-full = Chưa lưu { $name }: ổ đĩa đã đầy. Tệp trên ổ đĩa không thay đổi và văn bản của bạn vẫn còn ở đây. Hãy giải phóng dung lượng hoặc dùng Lưu thành để lưu vào nơi khác.
error-save-permission = Chưa lưu { $name }: bạn không có quyền ghi vào vị trí đó. Tệp trên ổ đĩa không thay đổi. Hãy dùng Lưu thành để lưu vào nơi khác.
error-save-read-only-volume = Chưa lưu { $name }: ổ đĩa ở chế độ chỉ đọc. Tệp trên ổ đĩa không thay đổi. Hãy dùng Lưu thành để lưu vào nơi khác.
replace-preview-title = { $count ->
   *[other] Thay thế { $count } kết quả khớp?
    }
replace-preview-confirm = { $count ->
   *[other] Thay thế { $count }
    }
replace-preview-more = Chỉ hiển thị 200 dòng thay đổi đầu tiên.
replace-preview-stale = Tài liệu đã thay đổi; không có gì được thay thế. Hãy chạy lại Thay thế tất cả.
encoding-detected = Đã mở dưới dạng { $encoding } (phát hiện từ nội dung tệp).
encoding-suggested = Đề xuất dựa trên nội dung tệp: { $encoding }. Hãy kiểm tra bản xem trước trước khi mở.
app-about = Giới thiệu về Sanain
menu-help = Trợ giúp
about-tagline = Sức mạnh của ngôn từ.
about-version = Phiên bản { $version }
about-license = Phần mềm tự do và mã nguồn mở theo Mozilla Public License 2.0.
about-ai = AI xây dựng. Con người dẫn dắt. Cộng đồng phát triển.
about-notices = Các thành phần của bên thứ ba và giấy phép của chúng được liệt kê trong THIRD_PARTY_NOTICES, đi kèm mỗi bản phát hành.
about-source = Mã nguồn và báo lỗi trên GitHub
tools-xml-validate = Kiểm tra XML
tools-yaml-validate = Kiểm tra YAML
tools-markup-valid = { $format } hợp lệ
file-open-recent = Mở gần đây…
file-save-all = Lưu tất cả
file-revert = Hoàn nguyên về bản đã lưu
recent-hint = Nhập để lọc các tệp gần đây
recent-none = Chưa có tệp gần đây nào.
revert-title = Hủy các thay đổi trong { $name }?
revert-body = Tệp được tải lại từ đĩa. Các thay đổi chưa lưu sẽ bị mất và không thể hoàn tác.
revert-confirm = Hủy và tải lại
revert-done = Đã tải lại { $name } từ đĩa.
status-saved-all = { $count ->
    [0] Không có gì để lưu.
   *[other] Đã lưu { $count } tệp.
    }
edit-toggle-comment = Bật/tắt chú thích dòng
edit-indent = Thụt lề dòng
edit-outdent = Giảm thụt lề dòng
comment-unsupported = Loại tệp này không có chú thích dòng.
tab-close-others = Đóng các thẻ khác
tab-close-all = Đóng tất cả thẻ
file-copy-path = Sao chép đường dẫn tệp
file-reveal-macos = Hiển thị trong Finder
file-reveal-windows = Hiển thị trong File Explorer
file-reveal-other = Hiển thị trong trình quản lý tệp
status-path-copied = Đã sao chép đường dẫn tệp.
tabs-kept-unsaved = { $count ->
   *[other] Đã giữ mở { $count } thẻ có thay đổi chưa lưu.
    }
status-indent-tabs = Tab
status-indent-spaces = Khoảng trắng: { $count }
find-searching = Đang tìm kiếm…
compare-ignore-whitespace = Bỏ qua khoảng trắng
tools-compare-file = So sánh với tệp…
compare-file-failed = Không thể so sánh với { $name }: { $reason }
cli-help =
    Cách dùng:
      sanain [TỆP…]              mở tệp
      sanain --diff TRÁI PHẢI    so sánh hai tệp cạnh nhau
      sanain --version           hiển thị phiên bản
      sanain --help              hiển thị trợ giúp này
    Dùng -- trước tên tệp bắt đầu bằng dấu gạch ngang.
cli-unknown-option = Tùy chọn không xác định { $option }.
cli-diff-needs-two = --diff cần hai tệp: sanain --diff TRÁI PHẢI
window-move-tab-left = Di chuyển thẻ sang trái
window-move-tab-right = Di chuyển thẻ sang phải
edit-skip-occurrence = Bỏ qua đến lần xuất hiện tiếp theo
tools-play-macro-times = Phát macro nhiều lần…
macro-times-hint = Số lần
macro-times-note = Từ 1 đến { $max }
macro-times-invalid = Nhập một số từ 1 đến { $max }.
view-outline = Dàn ý
outline-empty = Tài liệu này không có tiêu đề.
outline-close = Đóng dàn ý
folder-replace-placeholder = Thay bằng
folder-replace-run = Thay thế trong tệp…
replace-files-title = Thay thế { $count ->
   *[other] { $count } kết quả khớp
    } trong { $files ->
   *[other] { $files } tệp
    }?
replace-files-body = Các tệp đang mở trong thẻ sẽ được thay đổi tại đó và có thể hoàn tác; hãy tự lưu các tệp này. Các tệp khác được lưu ngay, và nội dung trước đó được giữ trong lịch sử cục bộ nếu tính năng này đang bật. Tệp đã thay đổi trên đĩa trong lúc đó sẽ bị bỏ qua.
replace-files-confirm = Thay thế
replace-files-done = Đã thay thế { $count ->
   *[other] { $count } kết quả khớp
    } trong { $files ->
   *[other] { $files } tệp
    } (đang mở trong thẻ: { $open }, chưa lưu).
replace-files-failed = { $failed ->
   *[other] { $failed } tệp không được thay đổi; tệp đầu tiên:
    } { $first }
image-outside = Hình ảnh này nằm ngoài thư mục của tài liệu.
image-allow = Hiển thị hình ảnh từ thư mục này
cli-help-integration =
    Tích hợp với trình quản lý tệp (chỉ người dùng hiện tại):
      sanain --register           thêm Sanain vào "Mở bằng" (Linux, Windows)
      sanain --unregister         gỡ lại khỏi đó
cli-registered = Đã thêm Sanain vào "Mở bằng". { $place }
cli-unregistered = Đã gỡ Sanain khỏi "Mở bằng".
cli-register-failed = Không thể thay đổi tích hợp với trình quản lý tệp: { $reason }
cli-register-macos = Trên macOS không cần đăng ký gì: khi Sanain.app nằm trong thư mục Ứng dụng, Finder sẽ hiển thị nó trong "Mở bằng".
settings-tab-general = Chung
settings-tab-editor = Trình soạn thảo
settings-tab-appearance = Giao diện
settings-shortcut-search = Tìm lệnh hoặc phím
settings-font = Phông chữ
settings-font-default = Phông đơn cách tích hợp
settings-font-loading = Đang tìm phông chữ đã cài đặt…
settings-font-missing = Không tìm thấy phông chữ { $font }; đang dùng phông chữ tích hợp.
settings-theme-more = Thêm chủ đề
settings-theme-user-note = Chủ đề của riêng bạn: tệp .theme trong thư mục { $folder } (xem docs/THEMES.md).
language-auto = Tự động nhận diện
language-choose = Chọn ngôn ngữ cho tab này
tools-xml-format = Định dạng XML
tools-save-macro = Lưu macro…
tools-play-saved-macro = Phát macro đã lưu…
tools-delete-saved-macro = Xóa macro đã lưu…
macro-save-hint = Tên macro
macro-save-note = Macro đã lưu có cùng tên sẽ bị thay thế.
macro-save-empty = Không có gì để lưu: hãy ghi một macro trước.
macro-saved = Đã lưu macro: { $name }
macro-deleted = Đã xóa macro: { $name }
macro-name-invalid = Nhập tên dài từ 1 đến { $max } ký tự trên một dòng.
macro-save-too-large = Macro này quá dài để lưu.
macro-save-full = Có thể lưu tối đa { $max } macro. Hãy xóa một macro trước.
macro-save-failed = Không thể lưu các macro: { $error }
macro-saved-hint = Nhập để lọc các macro đã lưu
macro-saved-none = Chưa có macro nào được lưu.
settings-shortcut-import-npp = Nhập từ Notepad++…
settings-shortcut-import-npp-hint = Chọn tệp shortcuts.xml của Notepad++. Trên Windows, tệp nằm ở %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Đã nhập từ Notepad++: { $imported }. Đã bỏ qua: { $skipped }. Xung đột còn lại: { $conflicts }.
settings-shortcut-import-npp-failed = Không thể nhập { $name }: { $reason }
settings-shortcut-import-npp-too-large = Tệp lớn hơn 1 MiB.
settings-shortcut-import-npp-invalid = Tệp không phải là shortcuts.xml hợp lệ của Notepad++.
view-split-right = Chia sang phải
view-split-down = Chia xuống dưới
view-close-pane = Đóng ngăn
view-focus-next-pane = Chuyển đến ngăn tiếp theo
view-focus-prev-pane = Chuyển đến ngăn trước
view-focus-next-region = Chuyển đến vùng tiếp theo
view-focus-prev-region = Chuyển đến vùng trước
status-pane-limit = Đã đạt giới hạn số ngăn (tối đa { $max }).
a11y-csv-grid = Bảng CSV
a11y-grid-cell = Hàng { $row }, cột { $column }: { $name }
a11y-grid-cell-plain = Hàng { $row }, cột { $column }
a11y-grid-column-header = Tiêu đề cột { $column }
a11y-grid-row-header = Hàng { $row }
a11y-edit-field = Đang sửa: { $item }
a11y-json-tree = Cây JSON
a11y-xml-tree = Cây XML
a11y-yaml-tree = Cây YAML
a11y-tree-item = Cấp { $level }
a11y-tree-item-expanded = Cấp { $level }, đã mở rộng
a11y-tree-item-collapsed = Cấp { $level }, đã thu gọn
a11y-tree-object = Đối tượng, số mục: { $count }
a11y-tree-list = Danh sách, số phần tử: { $count }
a11y-tree-element = Phần tử { $name }
a11y-tree-attribute = Thuộc tính { $name }
a11y-tree-text = Văn bản
a11y-pane = Ngăn { $pane } trên { $count }: { $name }
a11y-pane-active = Ngăn { $pane } trên { $count }: { $name }, đang hoạt động
a11y-pane-editor = Trình soạn thảo văn bản, ngăn { $pane } trên { $count }: { $name }
a11y-mermaid-flowchart = Lưu đồ. Nút: { $nodes }, liên kết: { $links }.
a11y-mermaid-sequence = Sơ đồ tuần tự. Thành phần tham gia: { $participants }, thông điệp: { $messages }.
a11y-mermaid-state = Sơ đồ trạng thái. Trạng thái: { $states }, chuyển tiếp: { $transitions }.
a11y-mermaid-class = Sơ đồ lớp. Lớp: { $classes }, quan hệ: { $relations }.
a11y-shortcut-change = Đổi phím tắt cho { $command }
a11y-shortcut-change-current = Đổi phím tắt cho { $command }, hiện tại { $keys }
a11y-shortcut-reset = Đặt lại phím tắt cho { $command }
status-line-ending-choose = Đổi ký tự xuống dòng của tệp này
status-encoding-choose = Mở lại hoặc lưu với bảng mã khác
status-indent-choose = Cài đặt thụt lề
a11y-settings-sections = Các mục cài đặt
a11y-tab-strip = Tệp đang mở
a11y-tab-unsaved = { $name }, có thay đổi chưa lưu
