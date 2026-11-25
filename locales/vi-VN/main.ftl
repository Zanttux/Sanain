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
preview-mermaid-unsupported = Chưa hỗ trợ hiển thị sơ đồ Mermaid; đang hiện mã nguồn.
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
json-more = … còn { $count } mục
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
