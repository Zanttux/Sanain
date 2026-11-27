# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
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
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
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
app-quit = Keluar dari Text³
app-hide = Sembunyikan Text³
app-hide-others = Sembunyikan Lainnya
app-show-all = Tampilkan Semua
window-minimize = Minimalkan
window-zoom = Zoom
status-recovered = Perubahan yang belum disimpan dari sesi terakhir telah dipulihkan
status-recovery-failed = Tidak dapat menyimpan salinan pemulihan: { $reason }
status-hot-exit-failed = Tidak dapat menyimpan perubahan yang belum disimpan untuk mulai berikutnya ({ $reason }); Anda akan ditanya sebagai gantinya.
status-hot-exit-too-large = { $name } terlalu besar untuk disimpan hingga mulai berikutnya; Anda akan ditanya sebagai gantinya.
status-restored-changed = { $name } berubah di disk setelah perubahan Anda yang belum disimpan diamankan. Teks Anda ada di tab; berkas tidak disentuh.
status-restored-missing = { $name } sudah tidak ada di disk. Teksnya yang belum disimpan tetap ada di tab; menyimpan akan membuat berkas itu lagi.
edit-duplicate-line = Duplikat Baris
edit-delete-line = Hapus Baris
edit-move-line-up = Pindahkan Baris ke Atas
edit-move-line-down = Pindahkan Baris ke Bawah
file-close-tab = Tutup Tab
file-reopen-tab = Buka Ulang Tab yang Ditutup
window-next-tab = Tab Berikutnya
window-prev-tab = Tab Sebelumnya
tab-close = Tutup tab
tab-missing-on-disk = Tidak ditemukan di disk
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
settings-hot-exit = Ingat perubahan yang belum disimpan saat keluar
settings-hot-exit-detail = Keluar tanpa ditanya tentang setiap tab yang belum disimpan. Tab yang belum disimpan dan tab tanpa judul kembali beserta teksnya saat mulai berikutnya, meskipun membuka ulang berkas dari sesi terakhir dimatikan. Menutup satu tab tetap akan bertanya.
settings-language = Bahasa
settings-language-system = Ikuti sistem
settings-language-note = Perubahan bahasa berlaku setelah Text³ dimulai ulang.
settings-save-failed = Tidak dapat menyimpan pengaturan: { $reason }
view-command-palette = Palet Perintah…
palette-placeholder = Ketik perintah
view-markdown-source = Sumber Markdown
view-markdown-split = Sumber dan Pratinjau Markdown
view-markdown-preview = Pratinjau Markdown
preview-too-large = Dokumen ini terlalu besar untuk dipratinjau.
preview-mermaid-unsupported-type = Jenis diagram tidak didukung; menampilkan sumbernya.
preview-mermaid-unsupported-syntax = Diagram memakai sintaks yang tidak dapat digambar; menampilkan sumbernya.
preview-mermaid-too-large = Diagram terlalu besar untuk digambar; menampilkan sumbernya.
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
csv-edit-cell = Edit sel
csv-insert-row-below = Sisipkan baris di bawah
csv-delete-row = Hapus baris
csv-read-only = Berkas hanya-baca: kisi tidak dapat diedit
json-more = … { $count } lagi
json-edit-value = Edit nilai
json-rename-key = Ganti nama kunci
json-add-after = Tambah sesudahnya
json-add-inside = Tambah di dalam
json-delete-node = Hapus
json-read-only = Berkas hanya-baca: pohon tidak dapat diedit
json-invalid-value = Bukan nilai JSON yang valid
json-duplicate-key = Objek sudah memiliki kunci dengan nama ini
xml-edit-value = Edit nilai
xml-edit-text = Edit teks
xml-rename = Ganti nama
xml-add-attribute = Tambah atribut
xml-add-child = Tambah elemen anak
xml-add-after = Tambah sesudahnya
xml-delete = Hapus
xml-invalid-name = Bukan nama XML yang valid
xml-duplicate-attribute = Elemen sudah memiliki atribut dengan nama ini
xml-not-well-formed = Perubahan ini akan membuat XML tidak valid
yaml-not-preserved = Perubahan ini akan mengubah bagian lain dari YAML
yaml-key-too-long = Kunci paling banyak 1024 karakter
yaml-read-only-flow = Gaya flow (ditulis di dalam kurung) belum dapat diedit di pohon
yaml-read-only-anchor = Nilai dengan anchor (&) tidak dapat diedit di pohon
yaml-read-only-tag = Nilai dengan tag (!) tidak dapat diedit di pohon
yaml-read-only-alias = Alias (*) tidak dapat diedit di pohon
yaml-read-only-block-scalar = Blok teks (| atau >) belum dapat diedit di pohon
yaml-read-only-complex-key = Kunci kompleks tidak dapat diedit di pohon
yaml-read-only-document = Hanya dokumen pertama yang dapat diedit di pohon
yaml-read-only-layout = Entri ini berbagi baris dengan konten lain, jadi tidak dapat diubah di sini
yaml-read-only-only-child = Satu-satunya entri tidak dapat dihapus
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
settings-keymap-default = Text³
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
update-up-to-date = Text³ { $version } adalah versi terbaru.
update-no-releases = Belum ada rilis Text³ yang diterbitkan.
update-failed = Tidak dapat memeriksa pembaruan: { $reason }
update-invalid = respons tak terduga
update-available-title = Text³ { $version } tersedia
update-available-body = Buka halaman unduhan untuk mendapatkannya. Text³ tidak mengunduh atau memasang apa pun dengan sendirinya.
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
tools-compare-saved = Bandingkan dengan Versi Tersimpan
tools-compare-clipboard = Bandingkan dengan Papan Klip
compare-saved-label = { $name } (di disk)
compare-clipboard-label = Papan Klip
compare-saved-failed = Tidak dapat membaca berkas tersimpan: { $reason }
compare-clipboard-empty = Papan klip tidak berisi teks untuk dibandingkan.
error-save-disk-full = { $name } tidak disimpan: disk penuh. Berkas di disk tidak berubah dan teks Anda masih ada di sini. Kosongkan sebagian ruang, atau gunakan Simpan Sebagai untuk menyimpannya di tempat lain.
error-save-permission = { $name } tidak disimpan: Anda tidak memiliki izin untuk menulis di lokasi tersebut. Berkas di disk tidak berubah. Gunakan Simpan Sebagai untuk menyimpannya di tempat lain.
error-save-read-only-volume = { $name } tidak disimpan: disk hanya-baca. Berkas di disk tidak berubah. Gunakan Simpan Sebagai untuk menyimpannya di tempat lain.
replace-preview-title = { $count ->
   *[other] Ganti { $count } kecocokan?
    }
replace-preview-confirm = { $count ->
   *[other] Ganti { $count }
    }
replace-preview-more = Hanya 200 baris pertama yang berubah yang ditampilkan.
replace-preview-stale = Dokumen telah berubah; tidak ada yang diganti. Jalankan Ganti Semua lagi.
encoding-detected = Dibuka sebagai { $encoding } (terdeteksi dari isi berkas).
encoding-suggested = Disarankan dari isi berkas: { $encoding }. Periksa pratinjau sebelum membuka.
app-about = Tentang Text³
menu-help = Bantuan
about-version = Versi { $version }
about-license = Bebas dan sumber terbuka di bawah Mozilla Public License 2.0.
about-ai = Dikembangkan dengan bantuan AI (Claude dari Anthropic) di bawah arahan pengelolanya.
about-notices = Komponen pihak ketiga beserta lisensinya tercantum dalam THIRD_PARTY_NOTICES, yang disertakan di setiap rilis.
about-source = Kode sumber dan laporan masalah di GitHub
tools-xml-validate = Validasi XML
tools-yaml-validate = Validasi YAML
tools-markup-valid = { $format } valid
file-open-recent = Buka Terbaru…
file-save-all = Simpan Semua
file-revert = Kembalikan ke Versi Tersimpan
recent-hint = Ketik untuk memfilter berkas terbaru
recent-none = Belum ada berkas terbaru.
revert-title = Buang perubahan pada { $name }?
revert-body = Berkas dimuat ulang dari disk. Perubahan yang belum disimpan akan hilang dan tidak dapat diurungkan.
revert-confirm = Buang dan Muat Ulang
revert-done = { $name } dimuat ulang dari disk.
status-saved-all = { $count ->
    [0] Tidak ada yang perlu disimpan.
   *[other] { $count } berkas disimpan.
    }
edit-toggle-comment = Alihkan Komentar Baris
edit-indent = Indentasi Baris
edit-outdent = Kurangi Indentasi Baris
comment-unsupported = Jenis file ini tidak memiliki komentar baris.
tab-close-others = Tutup Tab Lain
tab-close-all = Tutup Semua Tab
file-copy-path = Salin Jalur File
file-reveal-macos = Tampilkan di Finder
file-reveal-windows = Tampilkan di File Explorer
file-reveal-other = Tampilkan di Pengelola File
status-path-copied = Jalur file disalin.
tabs-kept-unsaved = { $count ->
   *[other] { $count } tab dengan perubahan yang belum disimpan tetap dibuka.
    }
status-indent-tabs = Tab
status-indent-spaces = Spasi: { $count }
find-searching = Mencari…
compare-ignore-whitespace = Abaikan spasi
tools-compare-file = Bandingkan dengan Berkas…
compare-file-failed = Tidak dapat membandingkan dengan { $name }: { $reason }
cli-help =
    Penggunaan:
      text3 [BERKAS…]             buka berkas
      text3 --diff KIRI KANAN     bandingkan dua berkas berdampingan
      text3 --version             tampilkan versi
      text3 --help                tampilkan bantuan ini
    Gunakan -- sebelum nama berkas yang diawali tanda hubung.
cli-unknown-option = Opsi tidak dikenal { $option }.
cli-diff-needs-two = --diff memerlukan dua berkas: text3 --diff KIRI KANAN
window-move-tab-left = Pindahkan Tab ke Kiri
window-move-tab-right = Pindahkan Tab ke Kanan
edit-skip-occurrence = Lewati ke Kemunculan Berikutnya
tools-play-macro-times = Putar Makro Beberapa Kali…
macro-times-hint = Jumlah kali
macro-times-note = Dari 1 hingga { $max }
macro-times-invalid = Masukkan angka dari 1 hingga { $max }.
view-outline = Kerangka
outline-empty = Tidak ada judul dalam dokumen ini.
outline-close = Tutup Kerangka
folder-replace-placeholder = Ganti dengan
folder-replace-run = Ganti di Berkas…
replace-files-title = Ganti { $count ->
   *[other] { $count } kecocokan
    } dalam { $files ->
   *[other] { $files } berkas
    }?
replace-files-body = Berkas yang terbuka di tab diubah di sana dan perubahannya dapat dibatalkan; simpan sendiri berkas tersebut. Berkas lain disimpan sekarang, dan isi sebelumnya disimpan di riwayat lokal jika fitur itu aktif. Berkas yang berubah di disk sementara itu akan dilewati.
replace-files-confirm = Ganti
replace-files-done = { $count ->
   *[other] { $count } kecocokan diganti
    } dalam { $files ->
   *[other] { $files } berkas
    } ({ $open } terbuka di tab, belum disimpan).
replace-files-failed = { $failed ->
   *[other] { $failed } berkas tidak diubah; yang pertama:
    } { $first }
image-outside = Gambar ini berada di luar folder dokumen.
image-allow = Tampilkan gambar dari folder ini
cli-help-integration =
    Integrasi dengan pengelola berkas (hanya pengguna saat ini):
      text3 --register            tambahkan Text³ ke "Buka Dengan" (Linux, Windows)
      text3 --unregister          hapus lagi dari sana
cli-registered = Text³ telah ditambahkan ke "Buka Dengan". { $place }
cli-unregistered = Text³ telah dihapus dari "Buka Dengan".
cli-register-failed = Integrasi pengelola berkas tidak dapat diubah: { $reason }
cli-register-macos = Di macOS tidak ada yang perlu didaftarkan: setelah Text³.app berada di folder Aplikasi, Finder akan menawarkannya di "Buka Dengan".
settings-tab-general = Umum
settings-tab-editor = Editor
settings-tab-appearance = Tampilan
settings-shortcut-search = Cari perintah atau tombol
settings-font = Font
settings-font-default = Monospace bawaan
settings-font-loading = Mencari font yang terpasang…
settings-font-missing = Font { $font } tidak ditemukan; menggunakan font bawaan.
settings-theme-more = Tema lainnya
settings-theme-user-note = Tema Anda sendiri: file .theme di folder { $folder } (lihat docs/THEMES.md).
language-auto = Deteksi Otomatis
language-choose = Pilih bahasa untuk tab ini
tools-xml-format = Format XML
tools-save-macro = Simpan Makro…
tools-play-saved-macro = Putar Makro Tersimpan…
tools-delete-saved-macro = Hapus Makro Tersimpan…
macro-save-hint = Nama makro
macro-save-note = Makro tersimpan dengan nama yang sama akan diganti.
macro-save-empty = Tidak ada yang disimpan: rekam makro terlebih dahulu.
macro-saved = Makro disimpan: { $name }
macro-deleted = Makro dihapus: { $name }
macro-name-invalid = Masukkan nama 1 hingga { $max } karakter dalam satu baris.
macro-save-too-large = Makro ini terlalu panjang untuk disimpan.
macro-save-full = Paling banyak { $max } makro dapat disimpan. Hapus salah satu terlebih dahulu.
macro-save-failed = Tidak dapat menyimpan makro: { $error }
macro-saved-hint = Ketik untuk memfilter makro tersimpan
macro-saved-none = Belum ada makro tersimpan.
settings-shortcut-import-npp = Impor dari Notepad++…
settings-shortcut-import-npp-hint = Pilih shortcuts.xml dari Notepad++. Di Windows, berkas ini ada di %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Diimpor dari Notepad++: { $imported }. Dilewati: { $skipped }. Konflik tersisa: { $conflicts }.
settings-shortcut-import-npp-failed = Tidak dapat mengimpor { $name }: { $reason }
settings-shortcut-import-npp-too-large = Berkas lebih besar dari 1 MiB.
settings-shortcut-import-npp-invalid = Berkas bukan shortcuts.xml Notepad++ yang valid.
view-split-right = Pisahkan ke Kanan
view-split-down = Pisahkan ke Bawah
view-close-pane = Tutup Panel
view-focus-next-pane = Fokus ke Panel Berikutnya
view-focus-prev-pane = Fokus ke Panel Sebelumnya
view-focus-next-region = Fokus ke Wilayah Berikutnya
view-focus-prev-region = Fokus ke Wilayah Sebelumnya
status-pane-limit = Batas panel tercapai (maksimal { $max }).
a11y-csv-grid = Tabel CSV
a11y-grid-cell = Baris { $row }, kolom { $column }: { $name }
a11y-grid-cell-plain = Baris { $row }, kolom { $column }
a11y-grid-column-header = Judul kolom { $column }
a11y-grid-row-header = Baris { $row }
a11y-edit-field = Mengedit: { $item }
a11y-json-tree = Pohon JSON
a11y-xml-tree = Pohon XML
a11y-yaml-tree = Pohon YAML
a11y-tree-item = Tingkat { $level }
a11y-tree-item-expanded = Tingkat { $level }, diperluas
a11y-tree-item-collapsed = Tingkat { $level }, diciutkan
a11y-tree-object = Objek, entri: { $count }
a11y-tree-list = Daftar, item: { $count }
a11y-tree-element = Elemen { $name }
a11y-tree-attribute = Atribut { $name }
a11y-tree-text = Teks
a11y-pane = Panel { $pane } dari { $count }: { $name }
a11y-pane-active = Panel { $pane } dari { $count }: { $name }, aktif
a11y-pane-editor = Editor teks, panel { $pane } dari { $count }: { $name }
a11y-mermaid-flowchart = Diagram alir. Simpul: { $nodes }, tautan: { $links }.
a11y-mermaid-sequence = Diagram urutan. Peserta: { $participants }, pesan: { $messages }.
a11y-mermaid-state = Diagram status. Status: { $states }, transisi: { $transitions }.
a11y-mermaid-class = Diagram kelas. Kelas: { $classes }, relasi: { $relations }.
a11y-shortcut-change = Ubah pintasan untuk { $command }
a11y-shortcut-change-current = Ubah pintasan untuk { $command }, saat ini { $keys }
a11y-shortcut-reset = Atur ulang pintasan untuk { $command }
status-line-ending-choose = Ubah akhir baris berkas ini
status-encoding-choose = Buka ulang atau simpan dengan pengodean lain
status-indent-choose = Pengaturan indentasi
a11y-settings-sections = Bagian pengaturan
a11y-tab-strip = Berkas terbuka
a11y-tab-unsaved = { $name }, ada perubahan yang belum disimpan
migration-copied = Pengaturan dan data dari Sanain telah disalin ke Text³. Folder Sanain tidak diubah.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "." }000
count-status = Karakter: { $chars } · Kata: { $words }
count-status-selection = Karakter: { $chars } dari { $total-chars } · Kata: { $words } dari { $total-words }
count-choose = Jumlah karakter, kata, baris, dan byte
count-a11y-pending = Karakter dan kata: belum dihitung. Buka untuk menghitung.
count-heading-selection = Pilihan
count-heading-document = Dokumen
count-characters = Karakter
count-characters-no-spaces = Karakter (tanpa spasi)
count-words = Kata
count-lines = Baris
count-bytes = Byte ({ $encoding })
count-unencodable = Beberapa karakter tidak dapat disimpan sebagai { $encoding }.
count-counting = Menghitung…
settings-show-counts = Tampilkan jumlah karakter dan kata di bilah status
