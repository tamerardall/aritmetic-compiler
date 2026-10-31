Rust ile derleyici yazmak

# Derleyici Atölyesi

Bir derleyici, metni adım adım daha yapılı bir şeye çevirir. Aşağıya bir ifade yazın: her aşamanın çıktısını canlı görürsünüz. Her aşamanın altında da o işi Rust'ta yapan araçlar var.

Kaynak kod

x =

y =

## Hepsi bir arada: Rust araç haritası

Hepsini kullanmak zorunda değilsiniz. Küçük bir dilde lexer ve parser'ı elle yazmak, öğrenmek için en iyi yoldur.

| Aşama | Rust'ta kullanılan | Ne zaman? |
| --- | --- | --- |
| Lexer | elle `chars()` döngüsü, `logos` | Elle yazmak 50 satır; `logos` hız ve düzen için. |
| Parser | elle recursive descent, `chumsky`, `lalrpop`, `nom`, `pest` | Önce elle yazın; dil büyürse üretici kütüphaneye geçin. |
| AST | `enum` + `Box`, `struct` | Her zaman. Rust'ın en güçlü olduğu yer. |
| Anlam analizi | `HashMap`, kendi tip denetleyiciniz | Hazır kütüphane yok, dilinize özgüdür. |
| Hata mesajı | `ariadne`, `miette`, `codespan-reporting` | Kullanıcı deneyimi için ilk günden. |
| Optimizasyon | AST/IR üzerinde `match` ile yeniden yazma | Önce sabit katlama gibi basit kurallar. |
| Kod üretimi | `cranelift`, `inkwell` (LLVM), C/WASM çıktısı (`wasm-encoder`) | Hız gerekiyorsa. Başlangıçta bytecode VM yeter. |
