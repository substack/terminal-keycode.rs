# terminal-keycode

convert bytes [from a terminal][vt102] into keycodes

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
      print!["code={:?} bytes={:?} printable={:?}\r\n",
        keycode, keycode.bytes(), keycode.printable()
      ];
      if keycode == KeyCode::CtrlC { return }
    }
  }
}
```

output with various key presses:

``` sh
$ cargo run -q --example keys
code=Char('a') bytes=[97] printable=Some('a')
code=Char('b') bytes=[98] printable=Some('b')
code=Char('c') bytes=[99] printable=Some('c')
code=PageUp bytes=[27, 91, 53, 126] printable=None
code=PageDown bytes=[27, 91, 54, 126] printable=None
code=Char('é') bytes=[195, 169] printable=Some('é')
code=Char('☃') bytes=[226, 152, 131] printable=Some('☃')
code=Tab bytes=[9] printable=Some('\t')
code=Space bytes=[32] printable=Some(' ')
code=Backspace bytes=[127] printable=None
code=ArrowUp bytes=[27, 91, 65] printable=None
code=F9 bytes=[27, 91, 50, 48, 126] printable=None
code=F3 bytes=[27, 79, 82] printable=None
code=CtrlA bytes=[1] printable=None
code=CtrlB bytes=[2] printable=None
code=CtrlC bytes=[3] printable=None
```

