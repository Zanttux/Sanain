# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
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
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
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
app-quit = Text³ Uygulamasından Çık
app-hide = Text³ Uygulamasını Gizle
app-hide-others = Diğerlerini Gizle
app-show-all = Tümünü Göster
window-minimize = Simge Durumuna Küçült
window-zoom = Yakınlaştır
status-recovered = Önceki oturumdaki kaydedilmemiş değişiklikler geri yüklendi
status-recovery-failed = Kurtarma kopyası kaydedilemedi: { $reason }
status-hot-exit-failed = Kaydedilmemiş değişiklikler bir sonraki başlatmaya kadar saklanamadı ({ $reason }); bunun yerine sorulacak.
status-hot-exit-too-large = { $name } bir sonraki başlatmaya kadar saklanamayacak kadar büyük; bunun yerine sorulacak.
status-restored-changed = Kaydedilmemiş değişiklikleriniz saklandıktan sonra { $name } diskte değişti. Metniniz sekmede; dosyaya dokunulmadı.
status-restored-missing = { $name } artık diskte yok. Kaydedilmemiş metni sekmede duruyor; kaydetmek dosyayı yeniden oluşturur.
edit-duplicate-line = Satırı Çoğalt
edit-delete-line = Satırı Sil
edit-move-line-up = Satırı Yukarı Taşı
edit-move-line-down = Satırı Aşağı Taşı
file-close-tab = Sekmeyi Kapat
file-reopen-tab = Kapatılan Sekmeyi Yeniden Aç
window-next-tab = Sonraki Sekme
window-prev-tab = Önceki Sekme
tab-close = Sekmeyi kapat
tab-missing-on-disk = Diskte bulunamadı
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
settings-hot-exit = Çıkışta kaydedilmemiş değişiklikleri hatırla
settings-hot-exit-detail = Kaydedilmemiş her sekme için sorulmadan çıkın. Kaydedilmemiş ve adsız sekmeler, önceki oturumdaki dosyaları yeniden açma kapalı olsa bile bir sonraki başlatmada metinleriyle birlikte geri gelir. Tek bir sekmeyi kapatmak yine sorar.
settings-language = Dil
settings-language-system = Sistemle aynı
settings-language-note = Dil değişiklikleri Text³ yeniden başlatıldıktan sonra uygulanır.
settings-save-failed = Ayarlar kaydedilemedi: { $reason }
view-command-palette = Komut Paleti…
palette-placeholder = Bir komut yazın
view-markdown-source = Markdown Kaynağı
view-markdown-split = Markdown Kaynağı ve Önizleme
view-markdown-preview = Markdown Önizleme
preview-too-large = Bu belge önizleme için çok büyük.
preview-mermaid-unsupported-type = Diyagram türü desteklenmiyor; kaynak gösteriliyor.
preview-mermaid-unsupported-syntax = Diyagram çizilemeyen bir sözdizimi kullanıyor; kaynak gösteriliyor.
preview-mermaid-too-large = Diyagram çizilemeyecek kadar büyük; kaynak gösteriliyor.
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
view-structured = Yapılandırılmış Görünüm (CSV, JSON, XML, YAML)
csv-delimiter = Ayırıcı
csv-rows = { $count ->
    [one] 1 satır
   *[other] { $count } satır
    }
csv-unterminated = { $line }. satırda başlayan tırnak kapatılmamış
csv-truncated = İlk 200.000 satır gösteriliyor
csv-edit-cell = Hücreyi düzenle
csv-insert-row-below = Alta satır ekle
csv-delete-row = Satırı sil
csv-read-only = Salt okunur dosya: tablo düzenlenemez
json-more = … { $count } tane daha
json-edit-value = Değeri düzenle
json-rename-key = Anahtarı yeniden adlandır
json-add-after = Sonrasına ekle
json-add-inside = İçine ekle
json-delete-node = Sil
json-read-only = Salt okunur dosya: ağaç düzenlenemez
json-invalid-value = Geçerli bir JSON değeri değil
json-duplicate-key = Nesnede bu adda bir anahtar zaten var
xml-edit-value = Değeri düzenle
xml-edit-text = Metni düzenle
xml-rename = Yeniden adlandır
xml-add-attribute = Öznitelik ekle
xml-add-child = Alt öğe ekle
xml-add-after = Sonrasına ekle
xml-delete = Sil
xml-invalid-name = Geçerli bir XML adı değil
xml-duplicate-attribute = Öğede bu adda bir öznitelik zaten var
xml-not-well-formed = Bu değişiklik XML'i geçersiz kılar
yaml-not-preserved = Bu değişiklik YAML'ın başka bölümlerini de değiştirirdi
yaml-key-too-long = Bir anahtar en fazla 1024 karakter olabilir
yaml-read-only-flow = Akış stili (parantez içinde yazılan) henüz ağaçta düzenlenemez
yaml-read-only-anchor = Çapalı (&) değerler ağaçta düzenlenemez
yaml-read-only-tag = Etiketli (!) değerler ağaçta düzenlenemez
yaml-read-only-alias = Takma adlar (*) ağaçta düzenlenemez
yaml-read-only-block-scalar = Metin blokları (| veya >) henüz ağaçta düzenlenemez
yaml-read-only-complex-key = Karmaşık anahtarlar ağaçta düzenlenemez
yaml-read-only-document = Ağaçta yalnızca ilk belge düzenlenebilir
yaml-read-only-layout = Bu girdi satırlarını başka içerikle paylaşıyor, bu yüzden burada değiştirilemez
yaml-read-only-only-child = Tek girdi kaldırılamaz
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
settings-keymap-default = Text³
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
update-up-to-date = Text³ { $version } en yeni sürümdür.
update-no-releases = Henüz yayımlanmış bir Text³ sürümü yok.
update-failed = Güncellemeler denetlenemedi: { $reason }
update-invalid = beklenmeyen yanıt
update-available-title = Text³ { $version } kullanılabilir
update-available-body = Edinmek için indirme sayfasını açın. Text³ kendiliğinden hiçbir şey indirmez veya yüklemez.
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
edit-add-cursor-above = Yukarıya İmleç Ekle
edit-add-cursor-below = Aşağıya İmleç Ekle
view-word-wrap = Sözcük Kaydırma
status-wrap-on = Uzun satırlar artık pencere genişliğinde kaydırılıyor.
status-wrap-off = Uzun satırlar artık kaydırılmıyor.
settings-word-wrap = Uzun satırları kaydır
edit-go-to-line = Satıra Git…
edit-go-to-heading = Başlığa Git…
goto-line-hint = Satır veya satır:sütun
goto-line-note = { $count ->
    [one] Belgede 1 satır var.
   *[other] Belgede { $count } satır var.
    }
goto-line-invalid = Bir satır numarası girin; isteğe bağlı olarak ardından :sütun ekleyin.
goto-heading-hint = Başlıkları filtrelemek için yazın
goto-heading-none = Bu belgede başlık yok.
edit-goto-anything = Herhangi Bir Yere Git…
goto-anything-hint = Dosya adı veya :satır, @sembol, #metin
goto-anything-line = { $line }. satıra git
goto-anything-open = açık
goto-anything-scanning = { $folder } içindeki dosyalar listeleniyor…
goto-anything-scanned = { $folder } içindeki dosyalar: { $count }
goto-anything-limit = { $folder } içindeki yalnızca ilk { $max } dosya listelenir.
goto-anything-no-folder = Bir klasörün dosyalarını listelemek için dosyayı kaydedin veya Klasörde Bul’u kullanın.
goto-anything-symbols-none = Bu belgede sembol yok.
goto-anything-matches-none = Bu belgede eşleşme yok.
format-xml = XML
format-yaml = YAML
tools-tree-error = Satır { $line }, sütun { $column } konumunda geçersiz { $format }: { $reason }
edit-select-next-occurrence = Sonraki Eşleşmeyi Seç
tools-record-macro = Makro Kaydını Başlat veya Durdur
tools-play-macro = Makroyu Oynat
macro-recording = Makro kaydediliyor
macro-recorded = { $count ->
    [one] Makro kaydedildi: 1 adım.
   *[other] Makro kaydedildi: { $count } adım.
    }
tools-compare-saved = Kaydedilmiş Sürümle Karşılaştır
tools-compare-clipboard = Panoyla Karşılaştır
compare-saved-label = { $name } (diskte)
compare-clipboard-label = Pano
compare-saved-failed = Kaydedilmiş dosya okunamadı: { $reason }
compare-clipboard-empty = Panoda karşılaştırılacak metin yok.
error-save-disk-full = { $name } kaydedilmedi: disk dolu. Diskteki dosya değişmedi ve metniniz hâlâ burada. Biraz yer açın veya başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
error-save-permission = { $name } kaydedilmedi: oraya yazma izniniz yok. Diskteki dosya değişmedi. Başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
error-save-read-only-volume = { $name } kaydedilmedi: disk salt okunur. Diskteki dosya değişmedi. Başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
replace-preview-title = { $count ->
    [one] 1 eşleşme değiştirilsin mi?
   *[other] { $count } eşleşme değiştirilsin mi?
    }
replace-preview-confirm = { $count ->
    [one] 1 tanesini değiştir
   *[other] { $count } tanesini değiştir
    }
replace-preview-more = Yalnızca değişen ilk 200 satır gösteriliyor.
replace-preview-stale = Belge değişti; hiçbir şey değiştirilmedi. Tümünü Değiştir'i yeniden çalıştırın.
encoding-detected = { $encoding } olarak açıldı (dosya içeriğinden algılandı).
encoding-suggested = Dosya içeriğine göre önerilen: { $encoding }. Açmadan önce önizlemeyi kontrol edin.
app-about = Text³ Hakkında
menu-help = Yardım
about-version = Sürüm { $version }
about-license = Mozilla Public License 2.0 kapsamında özgür ve açık kaynaklıdır.
about-ai = Bakımcısının yönetiminde yapay zekâ desteğiyle (Anthropic’in Claude’u) geliştirilmektedir.
about-notices = Üçüncü taraf bileşenler ve lisansları, her sürüme dahil olan THIRD_PARTY_NOTICES dosyasında listelenmiştir.
about-source = GitHub’da kaynak kodu ve sorunlar
tools-xml-validate = XML'i Doğrula
tools-yaml-validate = YAML'ı Doğrula
tools-markup-valid = { $format } geçerli
file-open-recent = Son Kullanılanı Aç…
file-save-all = Tümünü Kaydet
file-revert = Kaydedilmiş Sürüme Döndür
recent-hint = Son kullanılan dosyaları süzmek için yazın
recent-none = Henüz son kullanılan dosya yok.
revert-title = { $name } dosyasındaki değişiklikler atılsın mı?
revert-body = Dosya diskten yeniden yüklenir. Kaydedilmemiş değişiklikler kaybolur ve geri alınamaz.
revert-confirm = At ve Yeniden Yükle
revert-done = { $name } diskten yeniden yüklendi.
status-saved-all = { $count ->
    [0] Kaydedilecek bir şey yok.
    [one] 1 dosya kaydedildi.
   *[other] { $count } dosya kaydedildi.
    }
edit-toggle-comment = Satır Yorumunu Aç/Kapat
edit-indent = Satırları Girintile
edit-outdent = Satır Girintisini Azalt
comment-unsupported = Bu dosya türünde satır yorumu yok.
tab-close-others = Diğer Sekmeleri Kapat
tab-close-all = Tüm Sekmeleri Kapat
file-copy-path = Dosya Yolunu Kopyala
file-reveal-macos = Finder’da Göster
file-reveal-windows = Dosya Gezgini’nde Göster
file-reveal-other = Dosya Yöneticisinde Göster
status-path-copied = Dosya yolu kopyalandı.
tabs-kept-unsaved = { $count ->
    [one] Kaydedilmemiş değişiklikleri olan 1 sekme açık bırakıldı.
   *[other] Kaydedilmemiş değişiklikleri olan { $count } sekme açık bırakıldı.
    }
status-indent-tabs = Sekmeler
status-indent-spaces = Boşluklar: { $count }
find-searching = Aranıyor…
compare-ignore-whitespace = Boşlukları yok say
tools-compare-file = Dosyayla Karşılaştır…
compare-file-failed = { $name } ile karşılaştırılamadı: { $reason }
cli-help =
    Kullanım:
      text3 [DOSYA…]           dosyaları aç
      text3 --diff SOL SAĞ     iki dosyayı yan yana karşılaştır
      text3 --version          sürümü göster
      text3 --help             bu yardımı göster
    Tire ile başlayan dosya adlarından önce -- kullanın.
cli-unknown-option = Bilinmeyen seçenek { $option }.
cli-diff-needs-two = --diff iki dosya gerektirir: text3 --diff SOL SAĞ
window-move-tab-left = Sekmeyi Sola Taşı
window-move-tab-right = Sekmeyi Sağa Taşı
edit-skip-occurrence = Sonraki Eşleşmeye Atla
tools-play-macro-times = Makroyu Birden Çok Kez Oynat…
macro-times-hint = Tekrar sayısı
macro-times-note = 1 ile { $max } arası
macro-times-invalid = 1 ile { $max } arasında bir sayı girin.
view-outline = Ana Hat
outline-empty = Bu belgede başlık yok.
outline-close = Ana Hattı Kapat
folder-replace-placeholder = Şununla değiştir
folder-replace-run = Dosyalarda Değiştir…
replace-files-title = { $files ->
    [one] 1 dosyadaki
   *[other] { $files } dosyadaki
    } { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    } değiştirilsin mi?
replace-files-body = Bir sekmede açık olan dosyalar orada değiştirilir ve değişiklik geri alınabilir; bunları kendiniz kaydedin. Diğer dosyalar şimdi kaydedilir ve önceki içerikleri, etkinse yerel geçmişte saklanır. Bu sırada diskte değişen bir dosya atlanır.
replace-files-confirm = Değiştir
replace-files-done = { $files ->
    [one] 1 dosyada
   *[other] { $files } dosyada
    } { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    } değiştirildi (sekmelerde açık: { $open }, kaydedilmedi).
replace-files-failed = { $failed ->
    [one] 1 dosya değiştirilmedi:
   *[other] { $failed } dosya değiştirilmedi; ilki:
    } { $first }
image-outside = Bu görüntü belgenin klasörünün dışında.
image-allow = Bu klasördeki görüntüleri göster
cli-help-integration =
    Dosya yöneticisi tümleştirmesi (yalnızca geçerli kullanıcı):
      text3 --register            Text³ uygulamasını "Birlikte Aç" menüsüne ekle (Linux, Windows)
      text3 --unregister          yeniden kaldır
      text3 --default-apps[=plain,markdown,code,data,web]
                                  Text³ uygulamasını bu dosyalar için varsayılan yap
cli-registered = Text³ "Birlikte Aç" menüsüne eklendi. { $place }
cli-unregistered = Text³ "Birlikte Aç" menüsünden kaldırıldı.
cli-register-failed = Dosya yöneticisi tümleştirmesi değiştirilemedi: { $reason }
cli-register-macos = macOS'ta hiçbir şeyin kaydedilmesi gerekmez: Text³.app Uygulamalar klasörüne konduğunda Finder onu "Birlikte Aç" menüsünde sunar.
cli-default-apps-set = Text³ artık bu dosya türlerini açıyor: { $types }
cli-default-apps-unknown = Bilinmeyen dosya türü grubu: { $group }. Bilinen gruplar: plain, markdown, code, data, web.
default-apps-title = Dosyaları Text³ ile aç
default-apps-body = Text³, bu dosyalara çift tıkladığınızda onları açan uygulama olabilir. Bunu daha sonra Ayarlar › Genel bölümünden değiştirebilirsiniz.
default-apps-body-windows = Windows'ta varsayılan uygulamaları kendiniz seçersiniz: Text³, Ayarlar › Varsayılan uygulamalar sayfasını açar; orada her dosya türü için Text³ uygulamasını seçersiniz.
default-apps-unavailable = Text³ uygulamasının bu kopyası varsayılan uygulamaları değiştiremez: taşınabilir bir kopyadır veya macOS'ta Text³.app üzerinden başlatılmamıştır.
default-apps-plain = Metin ve günlükler
default-apps-markdown = Markdown
default-apps-code = Kaynak kod ve betikler
default-apps-data = Veri ve yapılandırma
default-apps-web = Web sayfaları ve stiller
default-apps-is-default = zaten Text³
default-apps-apply = Text³ uygulamasını varsayılan yap
default-apps-open-settings = Windows Ayarlarını aç
default-apps-not-now = Şimdi değil
default-apps-close = Kapat
default-apps-done = Text³ artık seçilen dosya türlerini açıyor.
default-apps-settings-opened = Windows Ayarları açık: Text³ uygulamasının açması gereken dosya türleri için onu seçin.
default-apps-failed = Varsayılan uygulamalar değiştirilemedi: { $reason }
settings-default-apps = Varsayılan uygulamalar
settings-default-apps-button = Dosya türlerini seç…
settings-tab-general = Genel
settings-tab-editor = Düzenleyici
settings-tab-appearance = Görünüm
settings-shortcut-search = Komut veya tuş ara
settings-font = Yazı tipi
settings-font-default = Yerleşik eş aralıklı
settings-font-loading = Yüklü yazı tipleri aranıyor…
settings-font-missing = { $font } yazı tipi bulunamadı; yerleşik yazı tipi kullanılıyor.
settings-theme-more = Diğer temalar
settings-theme-user-note = Kendi temalarınız: { $folder } klasöründeki .theme dosyaları (bkz. docs/THEMES.md).
language-auto = Otomatik Algıla
language-choose = Bu sekmenin dilini seçin
tools-xml-format = XML'i Biçimlendir
tools-save-macro = Makroyu Kaydet…
tools-play-saved-macro = Kayıtlı Makroyu Oynat…
tools-delete-saved-macro = Kayıtlı Makroyu Sil…
macro-save-hint = Makro adı
macro-save-note = Aynı adlı kayıtlı makro değiştirilir.
macro-save-empty = Kaydedilecek bir şey yok: önce bir makro kaydedin.
macro-saved = Makro kaydedildi: { $name }
macro-deleted = Makro silindi: { $name }
macro-name-invalid = Tek satırda 1 ile { $max } karakter arasında bir ad girin.
macro-save-too-large = Bu makro kaydedilemeyecek kadar uzun.
macro-save-full = En fazla { $max } makro kaydedilebilir. Önce birini silin.
macro-save-failed = Makrolar kaydedilemedi: { $error }
macro-saved-hint = Kayıtlı makroları süzmek için yazın
macro-saved-none = Henüz kayıtlı makro yok.
settings-shortcut-import-npp = Notepad++'tan içe aktar…
settings-shortcut-import-npp-hint = Notepad++ shortcuts.xml dosyasını seçin. Windows'ta %AppData%\Notepad++\shortcuts.xml konumundadır.
settings-shortcut-import-npp-done = Notepad++'tan içe aktarılan: { $imported }. Atlanan: { $skipped }. Kalan çakışma: { $conflicts }.
settings-shortcut-import-npp-failed = { $name } içe aktarılamadı: { $reason }
settings-shortcut-import-npp-too-large = Dosya 1 MiB'den büyük.
settings-shortcut-import-npp-invalid = Dosya geçerli bir Notepad++ shortcuts.xml dosyası değil.
view-split-right = Sağa Böl
view-split-down = Aşağı Böl
view-close-pane = Bölmeyi Kapat
view-focus-next-pane = Sonraki Bölmeye Odaklan
view-focus-prev-pane = Önceki Bölmeye Odaklan
view-focus-next-region = Sonraki Bölgeye Odaklan
view-focus-prev-region = Önceki Bölgeye Odaklan
status-pane-limit = Bölme sınırına ulaşıldı (en fazla { $max }).
a11y-csv-grid = CSV tablosu
a11y-grid-cell = Satır { $row }, sütun { $column }: { $name }
a11y-grid-cell-plain = Satır { $row }, sütun { $column }
a11y-grid-column-header = Sütun { $column } başlığı
a11y-grid-row-header = Satır { $row }
a11y-edit-field = Düzenleniyor: { $item }
a11y-json-tree = JSON ağacı
a11y-xml-tree = XML ağacı
a11y-yaml-tree = YAML ağacı
a11y-tree-item = Düzey { $level }
a11y-tree-item-expanded = Düzey { $level }, genişletilmiş
a11y-tree-item-collapsed = Düzey { $level }, daraltılmış
a11y-tree-object = Nesne, girdi sayısı: { $count }
a11y-tree-list = Liste, öğe sayısı: { $count }
a11y-tree-element = Öğe { $name }
a11y-tree-attribute = Öznitelik { $name }
a11y-tree-text = Metin
a11y-pane = Bölme { $pane } / { $count }: { $name }
a11y-pane-active = Bölme { $pane } / { $count }: { $name }, etkin
a11y-pane-editor = Metin düzenleyici, bölme { $pane } / { $count }: { $name }
a11y-mermaid-flowchart = Akış şeması. Düğümler: { $nodes }, bağlantılar: { $links }.
a11y-mermaid-sequence = Sıralama diyagramı. Katılımcılar: { $participants }, iletiler: { $messages }.
a11y-mermaid-state = Durum diyagramı. Durumlar: { $states }, geçişler: { $transitions }.
a11y-mermaid-class = Sınıf diyagramı. Sınıflar: { $classes }, ilişkiler: { $relations }.
a11y-shortcut-change = { $command } kısayolunu değiştir
a11y-shortcut-change-current = { $command } kısayolunu değiştir, şu an { $keys }
a11y-shortcut-reset = { $command } kısayolunu sıfırla
status-line-ending-choose = Bu dosyanın satır sonlarını değiştir
status-encoding-choose = Başka bir kodlamayla yeniden aç veya kaydet
status-indent-choose = Girinti ayarları
a11y-settings-sections = Ayar bölümleri
a11y-tab-strip = Açık dosyalar
a11y-tab-unsaved = { $name }, kaydedilmemiş değişiklikler
migration-copied = Sanain ayarları ve verileri Text³ uygulamasına kopyalandı. Sanain klasörleri değiştirilmedi.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "." }000
count-status = Karakter: { $chars } · Sözcük: { $words }
count-status-selection = Karakter: { $chars } / { $total-chars } · Sözcük: { $words } / { $total-words }
count-choose = Karakter, sözcük, satır ve bayt sayıları
count-a11y-pending = Karakterler ve sözcükler: henüz sayılmadı. Saymak için açın.
count-heading-selection = Seçim
count-heading-document = Belge
count-characters = Karakter
count-characters-no-spaces = Karakter (boşluksuz)
count-words = Sözcük
count-lines = Satır
count-bytes = Bayt ({ $encoding })
count-unencodable = Bazı karakterler { $encoding } olarak kaydedilemez.
count-counting = Sayılıyor…
settings-show-counts = Durum çubuğunda karakter ve sözcük sayısını göster
