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

## Character Notation

`CHAR` は Unicode で定義されているすべての文字を示す。ただし、定義されているかは策定によって変わるため、Unicode 範囲内のすべての値を示す。

> `CHAR` := U+0000 - U+FFFF

### White Space and Line Terminator

> `WHITE_SPACE` := `SP` | `TAB` | `FF` | `EOL`

| Notation | Description    | Unicode |
|:---------|:---------------|:--------|
| `SP`     | space          | \u0020  |
| `TAB`    | horizontal tab | \u0009  |
| `FF`     | form feed      | \u000C  |


改行または行末を示す `EOL` は単独の LF (U+000A), CR (U+000D) またはシーケンス CR LF によって示される。

> `EOL` := `CR` | `LF` | `CR LF`

### Digit

10進数の数字は 0 から 9 の文字で示される。

> `NON_ZERO_DIGIT` := `"1"` | `"2"` | `"3"` | `"4"` | `"5"` | `"6"` | `"7"` | `"8"` | `"9"`<br/>
> `DIGIT` := `"0"` | `NON_ZERO_DIGIT`

16 進数の数字は 0 から F の文字で示される。

> `HEX_DIGIT` := `DIGIT` | `"a"` | `"b"` | `"c"` | `"d"` | `"e"` | `"f"` | `"A"` | `"B"` | `"C"` | `"D"` | `"E"` | `"F"`

2 進数の数字は 0 か　1 のいずれかである。

> `BINARY_DIGIT` := `"0"` | `"1"`

いくつかの言語で使用されている 8 進数表記はこのプログラムでは扱わない。

### Letter

> `LETTER` := *Unicode character that is defined as Letter*

## Repeat Notation

アスタリスク記号 * は直前の要素の 0 個以上の繰り返しを示す。

> `X`* := "" | `X` | `X` `X` | ...

プラス記号 + は直前の要素の 1 個以上の繰り返しを示す。

> `X`+ := `X` | `X` `X` | ...

ブレースで囲われた数字 {N} は正確に N 個の繰り返しを示す。

> `X`{N} := `X`<sub>1</sub> `X`<sub>2</sub> ... `X`<sub>N</sub>

ブレースで囲われた 2 つの数字 {M,N} は M 個以上 N 個以下の繰り返し繰り返しを示す。

> `X`{M,N} := `X`<sub>1</sub> `X`<sub>2</sub> ... `X`<sub>n</sub>  (M≦n≦N)

### Comment

コメントは `// commet` 形式の行コメントと `/* comment */` 形式のブロックコメントの 2 種類が存在する。構文中の `"//"` から行末までは
行コメントとして構文上は意味がないものとして無視される。同様にブロックコメントは `"/*"` から `*/` までが無視される。

> `COMMENT` := `LINE_COMMENT` | `BLOCK_COMMENT`<br/>
> `LINE_COMMENT` := `"//"` (`CHAR` ^ `EOL`)* `EOL`<br/>
> `BLOCK_COMMENT` := `"/*"` (`CHAR` ^ `"*/"`)* `"*/"`

### Literal

> `LITERAL` := `BOOLEAN_LITERAL` | `STRING_LITERAL` | `NUMERIC_LITERAL` | `LIST_LITERAL` | `OBJECT_LITERAL`

#### Boolean Literal

> `BOOLEAN_LITERAL` := `"true"` | `"false"`

#### Numeric Literal

> `NUMERIC_LITERAL` := `DECIMAL_NUMERAL` | `HEX_NUMERAL` | `BINARY_NUMERAL` | `FLOATING_POINT_LITERAL`<br/>

> `DECIMAL_NUMERAL` := `NON_ZERO_DIGIT` `DIGIT`*<br/>
> `HEX_NUMERAL` := (`"0x"` | `"0X"`) `HEX_DIGIT`+<br/>
> `BINARY_NUMERAL` := (`"0b"` | `"0B"`) `BINARY_DIGIT`+

> `FLOATING_POINT_LITERAL` := (`NON_ZERO_DIGIT` `DIGIT`* `"."` `DIGIT`* `EXPONENT`?) |
> (`"."` `DIGIT`+ `EXPONENT`?) | (`NON_ZERO_DIGIT` `DIGIT`* `EXPONENT`)<br/>
> `EXPONENT` := (`"e"` | `"E"`) `SIGN`? `DIGIT`+<br/>
> `SIGN` := `"+"` | `"-"`

#### String Literal

> `STRING_LITERAL` := `'"'` `STRING_CHARACTER`* `'"'`<br/>
> `STRING_CHARACTER` := (`CHAR` ^ (`'"'` | `"\"`)) | `ESCAPE_SEQUENCE`<br/>
> `ESCAPE_SEQUENCE` := *Notion of the following table* | `UNICODE_ESCAPE`

| Notation | Character       | Unicode  |
|:---------|:----------------|:---------|
| `"\t"`   | horizontal tab  | `\u0009` |
| `"\n"`   | linefeed        | `\u000a` |
| `"\r"`   | carriage return | `\u000d` |
| `'\"'`   | double quote    | `\u0022` |
| `"\'"`   | single quote    | `\u0027` |
| `"\\"`   | backslash       | `\u005c` |

> `UNICODE_ESCAPE` := "\u" ( `HEXDIGIT`{4} | "{" `HEXDIGIT`{1,32} "}" )

#### List Literal

リストリテラルは要素を内包するリストを記述するための表記方法。最後の要素の後ろにコンマを配置することができる。

> `LIST_LITERAL` := `"["` `COMMA_SEPARATED_ELEMENTS` ","? `"]"`<br/>
> `COMMA_SEPARATED_ELEMENTS` := `ELEMENT` | `COMMA_SEPARATED_ELEMENTS` "," `ELEMENT`

#### Object Literal

オブジェクトリテラルはフィールドを持つ構造体を記述するための表記方法。最後のフィールドの後ろにコンマを配置することができる。

> `OBJECT_LITERAL` := "{" `COMMA_SEPARATED_FIELDS` ","? "}"<br/>
> `COMMA_SEPARATED_FIELDS` := `FIELD` | `COMMA_SEPARATED_FIELDS` "," `FIELD`<br/>
> `FIELD` := `IDENTIFIER` ":" `EXPRESSION`

### Identifier

> `IDENTIFIER` := `LETTER` (`LETTER` | `DIGIT`)*<br/>

## Data Structure

> `TUPLE` := "(" `COMMA_SEPARATED_LIST` ")"<br/>
> `SET` := "{" `COMMA_SEPARATED_LIST` "}"<br/>
> `MAP` := "{" `COMMA_SEPARATED_FIELD` "}"

## Expression and Statement Syntax

### `if` Expression

#### Basic `if` Expression

> `BASIC_IF_EXPRESSION` := "if" "(" `EXPRESSION` ")" `EXPRESSION`
> ("else" "if" "(" `EXPRESSION` ")" `EXPRESSION`)*
> ("else" "(" `EXPRESSION` ")" `EXPRESSION`)?

#### Assignment `if` Expression

> `ASSIGNMENT_IF_EXPRESSION` := `EXPRESSION` "if" `EXPRESSION` "else" `EXPRESSION`

### `while` Statement

### Comprehension

#### List Comprehension

リスト内包表記
[Wikipedia](https://en.wikipedia.org/wiki/List_comprehension)

> `"["` `EXPRESSION` `"in"` `IDENTIFIER` `"for"` `EXPRESSION` ("," `IDENTIFIER` `"for"` `EXPRESSION`)* (`"if"` `EXPRESSION`)? `"]"`

