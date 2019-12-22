extern crate fylm;

use fylm::image::Color;

#[test]
fn color_test() {
  let asadoai = Color::new(0x37, 0x5C, 0x77, 0x80);
  assert_eq!(asadoai.alpha, 0x80);
  assert_eq!(asadoai.red, 0x37);
  assert_eq!(asadoai.green, 0x5C);
  assert_eq!(asadoai.blue, 0x77);

  let asadoai = Color::new_opaque(0x37, 0x5C, 0x77);
  assert_eq!(asadoai.alpha, 0xFF);
  assert_eq!(asadoai.red, 0x37);
  assert_eq!(asadoai.green, 0x5C);
  assert_eq!(asadoai.blue, 0x77);

  let asadoai = Color::new_from_argb(0x80375C77);
  assert_eq!(asadoai.alpha, 0x80);
  assert_eq!(asadoai.red, 0x37, "{:?}", asadoai);
  assert_eq!(asadoai.green, 0x5C);
  assert_eq!(asadoai.blue, 0x77);

  let asadoai = Color::new_from_rgb(0x375C77);
  assert_eq!(asadoai.alpha, 0xFF);
  assert_eq!(asadoai.red, 0x37, "{:?}", asadoai);
  assert_eq!(asadoai.green, 0x5C);
  assert_eq!(asadoai.blue, 0x77);

  let asadoai = Color::new_from_argb(0x80375C77);
  assert_eq!(asadoai.to_argb(), 0x80375C77);

  assert_eq!(asadoai.to_string(), "#80375C77");
  assert_eq!(Color::new_from_rgb(0x375C77).to_string(), "#375C77");
}
