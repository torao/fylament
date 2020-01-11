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