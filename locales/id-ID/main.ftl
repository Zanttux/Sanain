# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Berkas
file-open = Buka…
file-save = Simpan
status-tabs = { $count ->
   *[other] { $count } tab
    }
error-open = Tidak dapat membuka { $name }.
menu-edit = Edit
file-new = Tab Baru
file-save-as = Simpan Sebagai…
edit-undo = Urungkan
edit-redo = Ulangi
edit-select-all = Pilih Semua
untitled = Tanpa Judul
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Brs { $line }, Kol { $column }
status-selections = { $count ->
   *[other] { $count } pilihan
    }
status-saved = { $name } disimpan
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 dengan BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Akhir baris campuran
editor-accessible-name = Editor teks
button-save = Simpan
button-dont-save = Jangan Simpan
button-cancel = Batal
button-ok = OK
button-overwrite = Timpa
button-save-as = Simpan Sebagai…
dialog-unsaved-title = Simpan perubahan pada { $name }?
dialog-unsaved-body = Perubahan Anda akan hilang jika tidak disimpan.
dialog-conflict-title = { $name } berubah di disk
dialog-conflict-body = Program lain mengubah berkas ini setelah Anda membukanya. Menimpa akan menggantikan perubahan tersebut.
dialog-readonly-title = { $name } hanya-baca
dialog-readonly-body = Simpan salinan dengan nama lain.
error-open-encoding = { $name } bukan teks UTF-8 atau UTF-16. Berkas tidak dibuka, sehingga isinya tidak berubah. Gunakan Berkas > Buka Ulang dengan Enkode untuk memilih enkodenya.
error-save = Tidak dapat menyimpan { $name }: { $reason }
menu-window = Jendela
edit-cut = Potong
edit-copy = Salin
edit-paste = Tempel
app-quit = Keluar dari Sanain
app-hide = Sembunyikan Sanain
app-hide-others = Sembunyikan Lainnya
app-show-all = Tampilkan Semua
window-minimize = Minimalkan
window-zoom = Zoom
status-recovered = Perubahan yang belum disimpan dari sesi terakhir telah dipulihkan
status-recovery-failed = Tidak dapat menyimpan salinan pemulihan: { $reason }
edit-duplicate-line = Duplikat Baris
edit-delete-line = Hapus Baris
edit-move-line-up = Pindahkan Baris ke Atas
edit-move-line-down = Pindahkan Baris ke Bawah
file-close-tab = Tutup Tab
file-reopen-tab = Buka Ulang Tab yang Ditutup
window-next-tab = Tab Berikutnya
window-prev-tab = Tab Sebelumnya
tab-close = Tutup tab
edit-find = Cari…
edit-find-next = Cari Berikutnya
edit-find-prev = Cari Sebelumnya
edit-replace = Ganti…
find-placeholder = Cari
replace-placeholder = Ganti dengan
find-match-case = Cocokkan huruf besar/kecil
find-whole-word = Seluruh kata
find-regex = Ekspresi reguler
find-no-results = Tidak ada hasil
find-count = { $total ->
   *[other] { $total } kecocokan
    }
find-count-current = { $current } dari { $total }
find-prev = Kecocokan sebelumnya (Shift+Enter)
find-next = Kecocokan berikutnya (Enter)
find-close = Tutup (Esc)
replace-one = Ganti
replace-all = Ganti Semua
status-replaced = { $count ->
   *[other] { $count } kecocokan diganti
    }
language-plain = Teks Biasa
menu-view = Tampilan
app-settings = Pengaturan…
view-zoom-in = Perbesar
view-zoom-out = Perkecil
view-zoom-reset = Ukuran Sebenarnya
settings-title = Pengaturan
settings-font-size = Ukuran font
settings-tab-width = Lebar Tab
settings-insert-spaces = Sisipkan spasi saat menekan Tab
settings-theme = Tampilan
settings-theme-system = Ikuti sistem
settings-theme-light = Terang
settings-theme-dark = Gelap
settings-restore-session = Buka ulang berkas dari sesi terakhir
settings-language = Bahasa
settings-language-system = Ikuti sistem
settings-language-note = Perubahan bahasa berlaku setelah Sanain dimulai ulang.
settings-save-failed = Tidak dapat menyimpan pengaturan: { $reason }
view-command-palette = Palet Perintah…
palette-placeholder = Ketik perintah
view-markdown-source = Sumber Markdown
view-markdown-split = Sumber dan Pratinjau Markdown
view-markdown-preview = Pratinjau Markdown
preview-too-large = Dokumen ini terlalu besar untuk dipratinjau.
preview-mermaid-unsupported = Diagram Mermaid belum dirender; menampilkan sumbernya.
menu-tools = Alat
tools-json-validate = Validasi JSON
tools-json-format = Format JSON
tools-json-minify = Perkecil JSON
tools-sort-lines = Urutkan Baris
tools-sort-lines-desc = Urutkan Baris Menurun
tools-unique-lines = Hapus Baris Duplikat
tools-trim-trailing = Pangkas Spasi di Akhir Baris
tools-upper-case = Huruf Besar
tools-lower-case = Huruf Kecil
tools-to-lf = Ubah Akhir Baris ke LF
tools-to-crlf = Ubah Akhir Baris ke CRLF
tools-json-valid = JSON valid
tools-done = Selesai. Urungkan akan membatalkannya.
tools-no-change = Tidak ada yang perlu diubah
tools-json-error = JSON tidak valid pada baris { $line }, kolom { $column }: { $reason }
json-reason-unterminated-string = string tidak ditutup
json-reason-invalid-u-escape = escape \u tidak valid
json-reason-invalid-escape = escape tidak valid
json-reason-control-character-in-string = karakter kontrol dalam string
json-reason-invalid-number = angka tidak valid
json-reason-unexpected-character = karakter tak terduga
json-reason-unexpected-content-after-the-value = konten tak terduga setelah nilai
json-reason-unexpected-comma = koma tak terduga
json-reason-expected-a-string-key = diharapkan kunci string
json-reason-expected = diharapkan ':'
json-reason-expected-or-a-closing-bracket = diharapkan ',' atau kurung tutup
json-reason-expected-a-value = diharapkan sebuah nilai
json-reason-empty-document = dokumen kosong
json-reason-unexpected-end-of-input = akhir input tak terduga
json-reason-too-deeply-nested-to-format = terlalu bersarang dalam untuk diformat
tools-compare = Bandingkan Dengan…
compare-need-two = Buka tab lain untuk dibandingkan.
compare-pick-title = Bandingkan { $name } dengan
compare-count = { $count ->
    [0] Tidak ada perbedaan
   *[other] { $count } perubahan
    }
compare-close = Tutup Perbandingan
compare-to-left = Salin perubahan ini ke kiri
compare-to-right = Salin perubahan ini ke kanan
compare-stale = Sebuah dokumen berubah selama perbandingan. Perbandingan telah diperbarui; coba lagi.
view-structured = Tampilan Terstruktur (CSV, JSON, XML, YAML)
csv-delimiter = Pemisah
csv-rows = { $count ->
   *[other] { $count } baris
    }
csv-unterminated = Tanda kutip tidak ditutup mulai baris { $line }
csv-truncated = Menampilkan 200.000 baris pertama
json-more = … { $count } lagi
dialog-open-link-title = Buka tautan ini di browser web Anda?
button-open = Buka
link-blocked = Jenis tautan ini tidak dibuka demi keamanan.
link-not-found = Berkas yang ditautkan tidak ditemukan: { $target }
enc-windows-1252 = Eropa Barat
enc-iso-8859-15 = Eropa Barat
enc-windows-1250 = Eropa Tengah
enc-iso-8859-2 = Eropa Tengah
enc-windows-1251 = Sirilik
enc-koi8-r = Sirilik
enc-windows-1253 = Yunani
enc-windows-1254 = Turki
enc-windows-1255 = Ibrani
enc-windows-1256 = Arab
enc-windows-1257 = Baltik
enc-windows-874 = Thai
enc-shift_jis = Jepang
enc-euc-jp = Jepang
enc-gb18030 = Tionghoa, Sederhana
enc-big5 = Tionghoa, Tradisional
enc-euc-kr = Korea
file-reopen-encoding = Buka Ulang dengan Enkode…
file-save-encoding = Simpan dengan Enkode…
encoding-reopen-title = Buka Ulang dengan Enkode
encoding-save-title = Simpan dengan Enkode
encoding-not-exact = Enkode ini tidak dapat membuka berkas tanpa mengubahnya: { $reason }
encoding-unencodable = “{ $character }” tidak dapat disimpan sebagai { $encoding }. Pilih enkode lain atau ubah teksnya; tidak ada yang ditulis.
encoding-save-first = Simpan atau urungkan perubahan Anda sebelum membuka ulang dengan enkode lain.
image-remote = Gambar jarak jauh tidak dimuat.
image-blocked = Hanya gambar di folder dokumen ini yang ditampilkan.
image-unsupported = Format gambar tidak didukung.
image-too-large = Gambar terlalu besar untuk ditampilkan.
image-unreadable = Gambar tidak dapat dibaca.
settings-theme-high-contrast = Kontras tinggi
view-fold = Lipat
view-unfold = Buka Lipatan
view-unfold-all = Buka Semua Lipatan
settings-shortcuts = Pintasan keyboard
settings-keymap-preset = Prasetel
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Ubah
settings-shortcut-reset = Atur Ulang
settings-shortcut-press = Tekan pintasan… (Esc untuk batal)
settings-shortcut-moved = Pintasan dipindahkan dari:
edit-find-in-folder = Cari di Folder…
folder-search-placeholder = Cari di folder
folder-search-run = Cari
folder-search-stop = Hentikan
folder-search-no-folder = Pilih folder
folder-search-summary = { $count ->
   *[other] { $count } kecocokan
    } dalam { $files ->
   *[other] { $files } berkas
    }
app-check-updates = Periksa Pembaruan…
update-checking = Memeriksa pembaruan…
update-up-to-date = Sanain { $version } adalah versi terbaru.
update-no-releases = Belum ada rilis Sanain yang diterbitkan.
update-failed = Tidak dapat memeriksa pembaruan: { $reason }
update-invalid = respons tak terduga
update-available-title = Sanain { $version } tersedia
update-available-body = Buka halaman unduhan untuk mendapatkannya. Sanain tidak mengunduh atau memasang apa pun dengan sendirinya.
update-open-page = Buka Halaman Unduhan
update-later = Nanti
settings-check-updates = Periksa pembaruan sekali sehari
settings-check-updates-detail = Menanyakan rilis terbaru ke github.com. Tidak ada data lain yang dikirim.
file-local-history = Riwayat Lokal…
file-clear-history = Hapus Riwayat Lokal…
history-title = Riwayat lokal { $name }
history-open = Buka
history-restore = Pulihkan
history-empty = Belum ada versi tersimpan. Salinan disimpan setiap kali Anda menyimpan.
history-unreadable = Versi ini tidak dapat dibaca.
history-just-now = Baru saja
history-minutes-ago = { $count ->
   *[other] { $count } menit yang lalu
    }
history-hours-ago = { $count ->
   *[other] { $count } jam yang lalu
    }
history-days-ago = { $count ->
   *[other] { $count } hari yang lalu
    }
history-purge-title = Hapus semua riwayat lokal?
history-purge-body = Salinan tersimpan dari semua berkas dihapus dari komputer ini. Berkas Anda sendiri tidak diubah.
history-purge-confirm = Hapus Riwayat
history-purged = Riwayat lokal dihapus.
settings-local-history = Simpan riwayat lokal berkas yang disimpan
settings-local-history-detail = Menyimpan hingga 50 versi per berkas selama 30 hari, hanya di komputer ini.
file-open-log = Buka di Penampil Log…
log-lines = { $count ->
   *[other] { $count } baris
    }
log-indexing = Membaca… { $percent }%
log-matches = { $count ->
   *[other] { $count } baris yang cocok
    }
log-filter-hint = Saring baris
log-bad-pattern = Pola tidak valid
log-follow = Ikuti
log-close = Tutup Penampil
log-opened-large = { $name } berukuran besar, jadi dibuka hanya-baca di penampil log.
tab-pin = Sematkan Tab
tab-unpin = Lepas Sematan Tab
edit-add-cursor-above = Tambah Kursor di Atas
edit-add-cursor-below = Tambah Kursor di Bawah
view-word-wrap = Bungkus Kata
status-wrap-on = Baris panjang kini dibungkus sesuai lebar jendela.
status-wrap-off = Baris panjang tidak lagi dibungkus.
settings-word-wrap = Bungkus baris panjang
edit-go-to-line = Ke Baris…
edit-go-to-heading = Ke Judul…
goto-line-hint = Baris atau baris:kolom
goto-line-note = { $count ->
   *[other] Dokumen ini memiliki { $count } baris.
    }
goto-line-invalid = Masukkan nomor baris, opsional diikuti :kolom.
goto-heading-hint = Ketik untuk menyaring judul
goto-heading-none = Dokumen ini tidak memiliki judul.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } tidak valid pada baris { $line }, kolom { $column }: { $reason }
edit-select-next-occurrence = Pilih Kemunculan Berikutnya
tools-record-macro = Mulai atau Hentikan Perekaman Makro
tools-play-macro = Putar Makro
macro-recording = Merekam makro
macro-recorded = { $count ->
   *[other] Makro direkam: { $count } langkah.
    }
