# aritmetic-compiler

Rust ile yazılmış, aritmetik ifadeleri bytecode'a derleyen ve çalıştıran küçük bir derleyici. Derleyicilerin nasıl çalıştığını öğrenmek için yapıldı. Dış bağımlılığı yok.

```
> 1 + 2 * 3
7
> (1 + 2) * 3
9
> 2^3^2
512
> -2^2
-4
> 1 / 0
Err: Division by zero
```

## Nasıl çalışır?

İfade dört aşamadan geçer:

```
"1 + 2 * 3"
   │  lexer.rs     metni token'lara böler
   ▼
[Num(1) Plus Num(2) Star Num(3) Eof]
   │  parser.rs    Pratt parser ile ağaç (AST) kurar
   ▼
Bin(1, Add, Bin(2, Mul, 3))
   │  codegen.rs   ağacı alttan yukarı gezip komut listesi üretir
   ▼
[Push(1), Push(2), Push(3), Mul, Add]
   │  vm.rs        yığın tabanlı sanal makinede çalıştırır
   ▼
7
```

| Dosya | Görev |
|---|---|
| `src/lexer.rs` | Karakterleri `Token`'a çevirir |
| `src/parser.rs` | `Token` listesinden `Expr` ağacı kurar, işlem önceliğini çözer |
| `src/codegen.rs` | `Expr` ağacından `Instr` listesi üretir |
| `src/vm.rs` | `Instr` listesini yığın üzerinde çalıştırır |
| `src/enums.rs` | `Token`, `Expr`, `Operation`, `Instr` tanımları |
| `src/main.rs` | Konsoldan satır okuyan REPL |

## Desteklenenler

| İşlem | Anlamı | Öncelik | Birleşme |
|---|---|---|---|
| `^` | üs alma | en yüksek | sağdan sola |
| `-x` | tekli eksi | `^`'den düşük | |
| `*` `/` | çarpma, bölme | | soldan sağa |
| `+` `-` | toplama, çıkarma | en düşük | soldan sağa |

Parantez ve ondalık sayılar (`2.5`) da çalışır. Ondalık ayracı noktadır.

Hatalar program çökmeden mesaj olarak döner: tanınmayan karakter, kapanmayan parantez, fazladan token, boş yığın, sıfıra bölme.

## Çalıştırma

Rust 1.85 veya üstü gerekir (edition 2024).

```
cargo run      # REPL'i açar, çıkmak için Ctrl+D
cargo test     # 20 birim testi
```

## Sınırlar

- Sayılar `f64`. Tam sayılar yaklaşık 9 × 10^15'e kadar kesin kalır, üstü yuvarlanır.
- Değişken ve fonksiyon yok.
- Hata mesajlarında konum bilgisi yok.
