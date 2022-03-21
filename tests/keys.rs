use terminal_keycode::{Decoder,KeyCode};

#[test]
fn keys() {
  let mut decoder = Decoder::new();
  assert_eq![decoder.write(0x1b), vec![]];
  assert_eq![decoder.write(0x5b), vec![]];
  assert_eq![decoder.write(0x35), vec![]];
  assert_eq![decoder.write(0x7e), vec![KeyCode::PageUp]];
  assert_eq![decoder.write(0x61), vec![KeyCode::Char('a')]];
  assert_eq![decoder.write(0x41), vec![KeyCode::Char('A')]];
  assert_eq![decoder.write(0xc3), vec![]];
  assert_eq![decoder.write(0xa9), vec![KeyCode::Char('é')]];
  assert_eq![decoder.write(0xe2), vec![]];
  assert_eq![decoder.write(0x98), vec![]];
  assert_eq![decoder.write(0x83), vec![KeyCode::Char('☃')]];
  assert_eq![decoder.write(0xd5), vec![]];
  assert_eq![decoder.write(0xf0), vec![KeyCode::Byte(0xd5),KeyCode::Byte(0xf0)]];
  assert_eq![decoder.write(0xeb), vec![]];
  assert_eq![decoder.write(0xa5), vec![]];
  assert_eq![decoder.write(0x1b), vec![KeyCode::Byte(0xeb),KeyCode::Byte(0xa5),KeyCode::Byte(0x1b)]];
}
