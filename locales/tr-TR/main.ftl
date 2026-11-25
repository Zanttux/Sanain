# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Dosya
file-open = Aç…
file-save = Kaydet
status-tabs = { $count ->
    [one] Bir sekme
   *[other] { $count } sekme
    }
error-open = { $name } açılamadı.
menu-edit = Düzen
file-new = Yeni Sekme
file-save-as = Farklı Kaydet…
edit-undo = Geri Al
edit-redo = Yinele
edit-select-all = Tümünü Seç
untitled = Adsız
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Sat { $line }, Süt { $column }
status-selections = { $count ->
    [one] 1 seçim
   *[other] { $count } seçim
    }
status-saved = { $name } kaydedildi
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 (BOM ile)
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Karışık satır sonları
editor-accessible-name = Metin düzenleyici
button-save = Kaydet
button-dont-save = Kaydetme
button-cancel = İptal
button-ok = Tamam
button-overwrite = Üzerine Yaz
button-save-as = Farklı Kaydet…
dialog-unsaved-title = { $name } dosyasındaki değişiklikler kaydedilsin mi?
dialog-unsaved-body = Kaydetmezseniz değişiklikleriniz kaybolacak.
dialog-conflict-title = { $name } diskte değişti
dialog-conflict-body = Siz açtıktan sonra başka bir program bu dosyayı değiştirdi. Üzerine yazmak bu değişiklikleri siler.
dialog-readonly-title = { $name } salt okunur
dialog-readonly-body = Bunun yerine farklı bir adla bir kopya kaydedin.
error-open-encoding = { $name } UTF-8 veya UTF-16 metin değil. Açılmadı, bu nedenle içeriği değişmedi. Kodlamasını seçmek için Dosya > Kodlamayla Yeniden Aç komutunu kullanın.
error-save = { $name } kaydedilemedi: { $reason }
menu-window = Pencere
edit-cut = Kes
edit-copy = Kopyala
edit-paste = Yapıştır
app-quit = Sanain'den Çık
app-hide = Sanain'i Gizle
app-hide-others = Diğerlerini Gizle
app-show-all = Tümünü Göster
window-minimize = Simge Durumuna Küçült
window-zoom = Yakınlaştır
status-recovered = Önceki oturumdaki kaydedilmemiş değişiklikler geri yüklendi
status-recovery-failed = Kurtarma kopyası kaydedilemedi: { $reason }
edit-duplicate-line = Satırı Çoğalt
edit-delete-line = Satırı Sil
edit-move-line-up = Satırı Yukarı Taşı
edit-move-line-down = Satırı Aşağı Taşı
file-close-tab = Sekmeyi Kapat
file-reopen-tab = Kapatılan Sekmeyi Yeniden Aç
window-next-tab = Sonraki Sekme
window-prev-tab = Önceki Sekme
tab-close = Sekmeyi kapat
edit-find = Bul…
edit-find-next = Sonrakini Bul
edit-find-prev = Öncekini Bul
edit-replace = Değiştir…
find-placeholder = Bul
replace-placeholder = Şununla değiştir
find-match-case = Büyük/küçük harf eşleştir
find-whole-word = Tam sözcük
find-regex = Normal ifade
find-no-results = Sonuç yok
find-count = { $total ->
    [one] 1 eşleşme
   *[other] { $total } eşleşme
    }
find-count-current = { $current } / { $total }
find-prev = Önceki eşleşme (Shift+Enter)
find-next = Sonraki eşleşme (Enter)
find-close = Kapat (Esc)
replace-one = Değiştir
replace-all = Tümünü Değiştir
status-replaced = { $count ->
    [one] 1 eşleşme değiştirildi
   *[other] { $count } eşleşme değiştirildi
    }
language-plain = Düz Metin
menu-view = Görünüm
app-settings = Ayarlar…
view-zoom-in = Yakınlaştır
view-zoom-out = Uzaklaştır
view-zoom-reset = Gerçek Boyut
settings-title = Ayarlar
settings-font-size = Yazı tipi boyutu
settings-tab-width = Tab genişliği
settings-insert-spaces = Tab tuşuna basıldığında boşluk ekle
settings-theme = Görünüm
settings-theme-system = Sistemle aynı
settings-theme-light = Açık
settings-theme-dark = Koyu
settings-restore-session = Önceki oturumdaki dosyaları yeniden aç
settings-language = Dil
settings-language-system = Sistemle aynı
settings-language-note = Dil değişiklikleri Sanain yeniden başlatıldıktan sonra uygulanır.
settings-save-failed = Ayarlar kaydedilemedi: { $reason }
view-command-palette = Komut Paleti…
palette-placeholder = Bir komut yazın
view-markdown-source = Markdown Kaynağı
view-markdown-split = Markdown Kaynağı ve Önizleme
view-markdown-preview = Markdown Önizleme
preview-too-large = Bu belge önizleme için çok büyük.
preview-mermaid-unsupported = Mermaid diyagramları henüz işlenmiyor; kaynak gösteriliyor.
menu-tools = Araçlar
tools-json-validate = JSON'u Doğrula
tools-json-format = JSON'u Biçimlendir
tools-json-minify = JSON'u Küçült
tools-sort-lines = Satırları Sırala
tools-sort-lines-desc = Satırları Azalan Sırada Sırala
tools-unique-lines = Yinelenen Satırları Kaldır
tools-trim-trailing = Sondaki Boşlukları Kırp
tools-upper-case = Büyük Harf
tools-lower-case = Küçük Harf
tools-to-lf = Satır Sonlarını LF'ye Dönüştür
tools-to-crlf = Satır Sonlarını CRLF'ye Dönüştür
tools-json-valid = JSON geçerli
tools-done = Tamamlandı. Geri Al ile geri alınabilir.
tools-no-change = Değiştirilecek bir şey yok
tools-json-error = Satır { $line }, sütun { $column } konumunda geçersiz JSON: { $reason }
json-reason-unterminated-string = sonlandırılmamış dize
json-reason-invalid-u-escape = geçersiz \u kaçış dizisi
json-reason-invalid-escape = geçersiz kaçış dizisi
json-reason-control-character-in-string = dizede denetim karakteri
json-reason-invalid-number = geçersiz sayı
json-reason-unexpected-character = beklenmeyen karakter
json-reason-unexpected-content-after-the-value = değerden sonra beklenmeyen içerik
json-reason-unexpected-comma = beklenmeyen virgül
json-reason-expected-a-string-key = dize anahtarı bekleniyordu
json-reason-expected = ':' bekleniyordu
json-reason-expected-or-a-closing-bracket = ',' veya kapanış ayracı bekleniyordu
json-reason-expected-a-value = değer bekleniyordu
json-reason-empty-document = boş belge
json-reason-unexpected-end-of-input = beklenmeyen giriş sonu
json-reason-too-deeply-nested-to-format = biçimlendirilemeyecek kadar derin iç içe
tools-compare = Şununla Karşılaştır…
compare-need-two = Karşılaştırmak için başka bir sekme açın.
compare-pick-title = { $name } şununla karşılaştır
compare-count = { $count ->
    [0] Fark yok
    [one] 1 değişiklik
   *[other] { $count } değişiklik
    }
compare-close = Karşılaştırmayı Kapat
compare-to-left = Bu değişikliği sola kopyala
compare-to-right = Bu değişikliği sağa kopyala
compare-stale = Karşılaştırma sırasında bir belge değişti. Karşılaştırma yenilendi; yeniden deneyin.
view-structured = Yapılandırılmış Görünüm (CSV, JSON)
csv-delimiter = Ayırıcı
csv-rows = { $count ->
    [one] 1 satır
   *[other] { $count } satır
    }
csv-unterminated = { $line }. satırda başlayan tırnak kapatılmamış
csv-truncated = İlk 200.000 satır gösteriliyor
json-more = … { $count } tane daha
dialog-open-link-title = Bu bağlantı web tarayıcınızda açılsın mı?
button-open = Aç
link-blocked = Güvenlik nedeniyle bu tür bağlantılar açılmaz.
link-not-found = Bağlantılı dosya bulunamadı: { $target }
enc-windows-1252 = Batı Avrupa
enc-iso-8859-15 = Batı Avrupa
enc-windows-1250 = Orta Avrupa
enc-iso-8859-2 = Orta Avrupa
enc-windows-1251 = Kiril
enc-koi8-r = Kiril
enc-windows-1253 = Yunanca
enc-windows-1254 = Türkçe
enc-windows-1255 = İbranice
enc-windows-1256 = Arapça
enc-windows-1257 = Baltık
enc-windows-874 = Tayca
enc-shift_jis = Japonca
enc-euc-jp = Japonca
enc-gb18030 = Çince, Basitleştirilmiş
enc-big5 = Çince, Geleneksel
enc-euc-kr = Korece
file-reopen-encoding = Kodlamayla Yeniden Aç…
file-save-encoding = Kodlamayla Kaydet…
encoding-reopen-title = Kodlamayla Yeniden Aç
encoding-save-title = Kodlamayla Kaydet
encoding-not-exact = Bu kodlama dosyayı değiştirmeden açamaz: { $reason }
encoding-unencodable = “{ $character }” karakteri { $encoding } olarak kaydedilemez. Başka bir kodlama seçin veya metni değiştirin; hiçbir şey yazılmadı.
encoding-save-first = Başka bir kodlamayla yeniden açmadan önce değişikliklerinizi kaydedin veya geri alın.
image-remote = Uzak görüntüler yüklenmez.
image-blocked = Yalnızca bu belgenin klasöründeki görüntüler gösterilir.
image-unsupported = Desteklenmeyen görüntü biçimi.
image-too-large = Görüntü gösterilemeyecek kadar büyük.
image-unreadable = Görüntü okunamadı.
settings-theme-high-contrast = Yüksek karşıtlık
view-fold = Daralt
view-unfold = Genişlet
view-unfold-all = Tümünü Genişlet
settings-shortcuts = Klavye kısayolları
settings-keymap-preset = Hazır ayar
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Değiştir
settings-shortcut-reset = Sıfırla
settings-shortcut-press = Bir kısayola basın… (Esc iptal eder)
settings-shortcut-moved = Kısayol şuradan taşındı:
edit-find-in-folder = Klasörde Bul…
folder-search-placeholder = Klasörde bul
folder-search-run = Ara
folder-search-stop = Durdur
folder-search-no-folder = Klasör seç
folder-search-summary = { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    }, { $files ->
    [one] 1 dosyada
   *[other] { $files } dosyada
    }
app-check-updates = Güncellemeleri Denetle…
update-checking = Güncellemeler denetleniyor…
update-up-to-date = Sanain { $version } en yeni sürümdür.
update-no-releases = Henüz yayımlanmış bir Sanain sürümü yok.
update-failed = Güncellemeler denetlenemedi: { $reason }
update-invalid = beklenmeyen yanıt
update-available-title = Sanain { $version } kullanılabilir
update-available-body = Edinmek için indirme sayfasını açın. Sanain kendiliğinden hiçbir şey indirmez veya yüklemez.
update-open-page = İndirme Sayfasını Aç
update-later = Daha Sonra
settings-check-updates = Güncellemeleri günde bir kez denetle
settings-check-updates-detail = En son sürümü github.com'a sorar. Başka hiçbir şey gönderilmez.
file-local-history = Yerel Geçmiş…
file-clear-history = Yerel Geçmişi Temizle…
history-title = { $name } dosyasının yerel geçmişi
history-open = Aç
history-restore = Geri Yükle
history-empty = Henüz kaydedilmiş sürüm yok. Her kaydedişinizde bir kopya saklanır.
history-unreadable = Bu sürüm okunamadı.
history-just-now = Az önce
history-minutes-ago = { $count ->
    [one] 1 dakika önce
   *[other] { $count } dakika önce
    }
history-hours-ago = { $count ->
    [one] 1 saat önce
   *[other] { $count } saat önce
    }
history-days-ago = { $count ->
    [one] 1 gün önce
   *[other] { $count } gün önce
    }
history-purge-title = Tüm yerel geçmiş silinsin mi?
history-purge-body = Tüm dosyaların kaydedilmiş kopyaları bu bilgisayardan kaldırılır. Dosyalarınızın kendisi değiştirilmez.
history-purge-confirm = Geçmişi Sil
history-purged = Yerel geçmiş silindi.
settings-local-history = Kaydedilen dosyaların yerel geçmişini sakla
settings-local-history-detail = Dosya başına en fazla 50 sürümü 30 gün boyunca yalnızca bu bilgisayarda saklar.
file-open-log = Günlük Görüntüleyicide Aç…
log-lines = { $count ->
    [one] 1 satır
   *[other] { $count } satır
    }
log-indexing = Okunuyor… %{ $percent }
log-matches = { $count ->
    [one] 1 eşleşen satır
   *[other] { $count } eşleşen satır
    }
log-filter-hint = Satırları filtrele
log-bad-pattern = Geçersiz desen
log-follow = Takip Et
log-close = Görüntüleyiciyi Kapat
log-opened-large = { $name } büyük olduğu için günlük görüntüleyicide salt okunur olarak açıldı.
tab-pin = Sekmeyi Sabitle
tab-unpin = Sekmenin Sabitlemesini Kaldır
