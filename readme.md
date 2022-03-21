# terminal-keycode

convert bytes [from a terminal][vt102] into a keycode enum

[vt102]: https://vt100.net/docs/vt102-ug/appendixc.html

# example

``` rs
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
      print!["code={:?} bytes={:?}\r\n", keycode, keycode.bytes()];
      if keycode == KeyCode::CtrlC { return }
    }
  }
}
```

