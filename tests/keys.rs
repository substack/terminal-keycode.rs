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
  assert_eq![decoder.write(0x1b), vec![]];
  assert_eq![decoder.write(0x5b), vec![]];
  assert_eq![decoder.write(0x31), vec![]];
  assert_eq![decoder.write(0x35), vec![]];
  assert_eq![decoder.write(0x70), vec![
    KeyCode::Escape, KeyCode::Char('['), KeyCode::Char('1'), KeyCode::Char('5'), KeyCode::Char('p')
  ]];
}

#[test]
fn bytes() {
  let mut decoder = Decoder::new();
  let keycodes = vec![
    KeyCode::ArrowUp, KeyCode::ArrowDown, KeyCode::ArrowLeft, KeyCode::ArrowRight,
    KeyCode::CtrlArrowUp, KeyCode::CtrlArrowDown, KeyCode::CtrlArrowLeft, KeyCode::CtrlArrowRight,
    KeyCode::Numpad5, KeyCode::CtrlNumpad5,
    KeyCode::Delete, KeyCode::Insert, KeyCode::Home, KeyCode::End, KeyCode::PageUp, KeyCode::PageDown,
    KeyCode::CtrlDelete, KeyCode::CtrlInsert, KeyCode::CtrlHome, KeyCode::CtrlEnd,
    KeyCode::ShiftDelete, KeyCode::ShiftInsert, KeyCode::ShiftHome, KeyCode::ShiftEnd,
    KeyCode::CtrlPageUp, KeyCode::CtrlPageDown, KeyCode::ShiftPageUp, KeyCode::ShiftPageDown,
    KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4, KeyCode::F5, KeyCode::F6,
    KeyCode::F7, KeyCode::F8, KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12,
    KeyCode::CtrlF1, KeyCode::CtrlF2, KeyCode::CtrlF3, KeyCode::CtrlF4, KeyCode::CtrlF5, KeyCode::CtrlF6,
    KeyCode::CtrlF7, KeyCode::CtrlF8, KeyCode::CtrlF9, KeyCode::CtrlF10, KeyCode::CtrlF11, KeyCode::CtrlF12,
    KeyCode::ShiftF1, KeyCode::ShiftF2, KeyCode::ShiftF3, KeyCode::ShiftF4, KeyCode::ShiftF5, KeyCode::ShiftF6,
    KeyCode::ShiftF7, KeyCode::ShiftF8, KeyCode::ShiftF9, KeyCode::ShiftF10, KeyCode::ShiftF11, KeyCode::ShiftF12,
    KeyCode::CtrlShiftF1, KeyCode::CtrlShiftF2, KeyCode::CtrlShiftF3, KeyCode::CtrlShiftF4, KeyCode::CtrlShiftF5,
    KeyCode::CtrlShiftF6, KeyCode::CtrlShiftF7, KeyCode::CtrlShiftF8, KeyCode::CtrlShiftF9, KeyCode::CtrlShiftF10,
    KeyCode::CtrlShiftF11, KeyCode::CtrlShiftF12,
    KeyCode::CtrlA, KeyCode::CtrlB, KeyCode::CtrlC, KeyCode::CtrlD, KeyCode::CtrlE,
    KeyCode::CtrlF, KeyCode::CtrlG, KeyCode::CtrlH,
    KeyCode::Tab, KeyCode::ShiftTab, KeyCode::Linefeed, KeyCode::Enter,
    KeyCode::CtrlK, KeyCode::CtrlL, KeyCode::CtrlN, KeyCode::CtrlO, KeyCode::CtrlP,
    KeyCode::CtrlQ, KeyCode::CtrlR, KeyCode::CtrlS, KeyCode::CtrlT, KeyCode::CtrlU,
    KeyCode::CtrlV, KeyCode::CtrlW, KeyCode::CtrlX, KeyCode::CtrlY, KeyCode::CtrlZ,
    KeyCode::Space, KeyCode::Backspace,
    KeyCode::Menu, KeyCode::CtrlMenu, KeyCode::ShiftMenu, KeyCode::CtrlShiftMenu,
    KeyCode::Char('a'),
    KeyCode::Char('B'),
    KeyCode::Char('t'),
    KeyCode::Char('U'),
    KeyCode::Char('€'),
    KeyCode::Char('ø'),
    KeyCode::Char('☃'),
  ];
  for keycode in keycodes {
    let bs = keycode.bytes();
    let len = bs.len();
    for (i,b) in bs.iter().enumerate() {
      if i == len-1 {
        assert_eq![decoder.write(*b), vec![keycode.clone()]];
      } else {
        assert_eq![decoder.write(*b), vec![], "keycode={:?}", &keycode];
      }
    }
  }
}
