# terminal-keycode

convert bytes [from a terminal][vt102] into keycodes

[vt102]: https://vt100.net/docs/vt102-ug/appendixc.html

Information is hard to come by, but these codes were measured experimentally in xterm with a 104 key
US windows keyboard. Other terminals may give somewhat different results. For example, the tty login
terminal on my linux laptop will only show the base keycode even if you hold down shift or ctrl.

Different terminals and window managers will capture different key combos, so your program won't be
able to catch everything.

This library expects unicode input and will lookahead a corresponding number of bytes based on how
many high bits are set, populating a `KeyCode::Char(char)` with a unicode scalar value. If a
non-interpreted sequence is not a valid unicode scalar value, you will get a sequence of
`KeyCode::Byte(u8)`.

# example

``` rust,no_run
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

output with various key presses in xterm:

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
code=CtrlF1 bytes=[27, 91, 49, 59, 53, 80] printable=None
code=CtrlShiftF2 bytes=[27, 91, 49, 59, 54, 81] printable=None
code=ShiftF3 bytes=[27, 91, 49, 59, 50, 82] printable=None
code=CtrlArrowUp bytes=[27, 91, 49, 59, 53, 65] printable=None
code=CtrlShiftArrowLeft bytes=[27, 91, 49, 59, 54, 68] printable=None
code=CtrlA bytes=[1] printable=None
code=CtrlB bytes=[2] printable=None
code=CtrlC bytes=[3] printable=None
```

If all you need are the byte sequences for different keycodes, you can do:

``` rust,no_run
use terminal_keycode::KeyCode;

fn main() {
  println!["{:?}", KeyCode::ArrowLeft.bytes()]; // [27, 91, 68]
  println!["{:?}", KeyCode::CtrlShiftF1.bytes()]; // [27, 91, 49, 59, 54, 80]
  println!["{:?}", KeyCode::Char('F').bytes()]; // [70]
  println!["{:?}", KeyCode::Home.bytes()]; // [27, 91, 72]
}
```
