use std::io::Read;
use raw_tty::IntoRawMode;
use terminal_keycode::{Decoder,KeyCode};

fn main() {
  let mut stdin = std::io::stdin().into_raw_mode().unwrap();
  let mut buf = vec![0];
  let mut decoder = Decoder::new();
  loop {
    stdin.read_exact(&mut buf).unwrap();
    for keycode in decoder.write(buf[0]) {
      print!["code={:?} bytes={:?} printable={:?}\r\n",
        keycode, keycode.bytes(), keycode.printable()
      ];
      if keycode == KeyCode::CtrlC { return }
    }
  }
}
