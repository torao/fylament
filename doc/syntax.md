# Object Motion Markup Language

## Object Notation

```
ObjectName {
  PropertyName1: DefaultValue1,
  PropertyName2: DefaultValue2,
  // ...
}
```

## オブジェクト
### Image
```
Image {
  uri: 画像 URI,
}
```
```
Image.transform(1000msec) { t =>
  uri: transit("1.png", "2.png", "3.png")
}.loop()
```

### Basic Notation

#### Repeat Notation

アスタリスク記号 * は直前の要素の 0 個以上の繰り返しを示す。

> `X`* := "" | `X` | `X` `X` | ...

プラス記号 + は直前の要素の 1 個以上の繰り返しを示す。

> `X`+ := `X` | `X` `X` | ...

ブレースで囲われた数字 {N} は正確に N 個の繰り返しを示す。

> `X`{N} := `X`<sub>1</sub> `X`<sub>2</sub> ... `X`<sub>N</sub>

ブレースで囲われた数字 {M,N} は M 個以上 N 個以下の繰り返し繰り返しを示す。

> `X`{M,N} := `X`<sub>1</sub> `X`<sub>2</sub> ... `X`<sub>n</sub>  (M≦n≦N)

#### Character Notation

`CHAR` は Unicode で定義されているすべての文字を示す。ただし、定義されているかは策定によって変わるため、Unicode 範囲内のすべての値を示す。

> `CHAR` := U+0000 - U+FFFF

#### End of Line

改行または行末を示す `EOL` は単独の LF (U+000A), CR (U+000D) またはそれらのシーケンス CR LF によって示される。

> `EOL` := `CR` | `LF` | `CR LF`

### Comment

コメントは行コメントとブロックコメントの 2 種類が存在する。構文中の `"//"` から行末までは行コメントとして構文上は意味がない
ものとして無視される。同様にブロックコメントは `"/*"` から `*/` までが無視される。

> `COMMENT` := `LINE_COMMENT` | `BLOCK_COMMENT`<br/>
> `LINE_COMMENT` := `"//"` (`CHAR` ^ `EOL`)* `EOL`<br/>
> `BLOCK_COMMENT` := `"/*"` (`CHAR` ^ `"*/"`)* `"*/"`

### Literal

> `LITERAL` := `STRING_LITERAL` | `NUMERIC_LITERAL`

#### String Literal

> `STRING_LITERAL` := `'"'` (`STRING_LITERAL_CHAR` | ESCAPE_SEQUENCE`)* `'"'`
> `STRING_LITERAL_CHAR` := *Unicode excludes control code and "*

##### Escape Sequence

> `ESCAPE_SEQUENCE` := `"\r"` | `"\n"` | `"\t"` | 

