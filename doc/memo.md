# Idea Memo

「世界」には単一の時刻があり、時刻をフレームレートごとに進めながらアニメーションに見せる。
各オブジェクトは時間に対して位置や形などを決定する属性がどう変化するかを記述する。

オブジェクトはテンプレートのように扱う。`deploy` 命令でそのインスタンスが生成される。

```
Image {
  x as left: 0px,
  y as top: 0px,
  uri: "image.png",
  
  paint: (g) -> Unit {
    // 属性に基づいた描画処理
  },
}
```

```
deploy Image.transform(100ms) {
  x: 0..100,
  y: 0..100,
  uri: ["1.png", "2.png", "3.png"s],
} as image;
```
