#![doc=include_str!("../readme.md")]

#[derive(Debug,Clone,PartialEq)]
pub enum KeyCode {
  ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
  CtrlArrowUp, CtrlArrowDown, CtrlArrowLeft, CtrlArrowRight,
  ShiftArrowUp, ShiftArrowDown, ShiftArrowLeft, ShiftArrowRight,
  CtrlShiftArrowUp, CtrlShiftArrowDown, CtrlShiftArrowLeft, CtrlShiftArrowRight,
  Numpad5, CtrlNumpad5,
  Delete, Insert, Home, End, PageUp, PageDown,
  CtrlDelete, CtrlInsert, CtrlHome, CtrlEnd, CtrlPageUp, CtrlPageDown,
  ShiftDelete, ShiftInsert, ShiftHome, ShiftEnd, ShiftPageUp, ShiftPageDown,
  CtrlShiftDelete, CtrlShiftInsert, CtrlShiftHome, CtrlShiftEnd, CtrlShiftPageUp, CtrlShiftPageDown,
  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
  CtrlF1, CtrlF2, CtrlF3, CtrlF4, CtrlF5, CtrlF6, CtrlF7, CtrlF8, CtrlF9, CtrlF10, CtrlF11, CtrlF12,
  ShiftF1, ShiftF2, ShiftF3, ShiftF4, ShiftF5, ShiftF6, ShiftF7, ShiftF8, ShiftF9, ShiftF10, ShiftF11, ShiftF12,
  CtrlShiftF1, CtrlShiftF2, CtrlShiftF3, CtrlShiftF4, CtrlShiftF5, CtrlShiftF6,
  CtrlShiftF7, CtrlShiftF8, CtrlShiftF9, CtrlShiftF10, CtrlShiftF11, CtrlShiftF12,
  CtrlA, CtrlB, CtrlC, CtrlD, CtrlE, CtrlF, CtrlG, CtrlH,
  Tab, ShiftTab, Linefeed, CtrlK, CtrlL, Enter,
  CtrlN, CtrlO, CtrlP, CtrlQ, CtrlR, CtrlS, CtrlT, CtrlU, CtrlV, CtrlW, CtrlX, CtrlY, CtrlZ,
  Space, Backspace, Escape, Menu, CtrlMenu, ShiftMenu, CtrlShiftMenu,
  Char(char),
  Byte(u8),
}

impl KeyCode {
  /// KeyCodes are printable if they have a visible representation as a character, including
  /// whitespace.
  pub fn printable(&self) -> Option<char> {
    match self {
      Self::Char(c) => Some(*c),
      Self::Space => Some(' '),
      Self::Enter => Some('\r'),
      Self::Linefeed => Some('\n'),
      Self::Tab => Some('\t'),
      _ => None,
    }
  }
  /// Return the terminal byte sequences for a given KeyCode.
  pub fn bytes(&self) -> Vec<u8> {
    match self {
      Self::ArrowUp => vec![0x1b,0x5b,0x41],
      Self::ArrowDown => vec![0x1b,0x5b,0x42],
      Self::ArrowRight => vec![0x1b,0x5b,0x43],
      Self::ArrowLeft => vec![0x1b,0x5b,0x44],
      Self::CtrlArrowUp => vec![0x1b,0x5b,0x31,0x3b,0x35,0x41],
      Self::CtrlArrowDown => vec![0x1b,0x5b,0x31,0x3b,0x35,0x42],
      Self::CtrlArrowRight => vec![0x1b,0x5b,0x31,0x3b,0x35,0x43],
      Self::CtrlArrowLeft => vec![0x1b,0x5b,0x31,0x3b,0x35,0x44],
      Self::ShiftArrowUp => vec![0x1b,0x5b,0x31,0x3b,0x32,0x41],
      Self::ShiftArrowDown => vec![0x1b,0x5b,0x31,0x3b,0x32,0x42],
      Self::ShiftArrowRight => vec![0x1b,0x5b,0x31,0x3b,0x32,0x43],
      Self::ShiftArrowLeft => vec![0x1b,0x5b,0x31,0x3b,0x32,0x44],
      Self::CtrlShiftArrowUp => vec![0x1b,0x5b,0x31,0x3b,0x36,0x41],
      Self::CtrlShiftArrowDown => vec![0x1b,0x5b,0x31,0x3b,0x36,0x42],
      Self::CtrlShiftArrowRight => vec![0x1b,0x5b,0x31,0x3b,0x36,0x43],
      Self::CtrlShiftArrowLeft => vec![0x1b,0x5b,0x31,0x3b,0x36,0x44],
      Self::Numpad5 => vec![0x1b,0x5b,0x45],
      Self::CtrlNumpad5 => vec![0x1b,0x5b,0x31,0x3b,0x35,0x45],
      Self::Delete => vec![0x1b,0x5b,0x33,0x7e],
      Self::Insert => vec![0x1b,0x5b,0x32,0x7e],
      Self::Home => vec![0x1b,0x5b,0x48],
      Self::End => vec![0x1b,0x5b,0x46],
      Self::PageUp => vec![0x1b,0x5b,0x35,0x7e],
      Self::PageDown => vec![0x1b,0x5b,0x36,0x7e],
      Self::CtrlDelete => vec![0x1b,0x5b,0x33,0x3b,0x35,0x7e],
      Self::CtrlInsert => vec![0x1b,0x5b,0x32,0x3b,0x35,0x7e],
      Self::CtrlHome => vec![0x1b,0x5b,0x31,0x3b,0x35,0x48],
      Self::CtrlEnd => vec![0x1b,0x5b,0x31,0x3b,0x35,0x46],
      Self::CtrlPageUp => vec![0x1b,0x5b,0x35,0x3b,0x35,0x7e],
      Self::CtrlPageDown => vec![0x1b,0x5b,0x36,0x3b,0x35,0x7e],
      Self::ShiftDelete => vec![0x1b,0x5b,0x33,0x3b,0x32,0x7e],
      Self::ShiftInsert => vec![0x1b,0x5b,0x32,0x3b,0x32,0x7e], // unconfirmed
      Self::ShiftHome => vec![0x1b,0x5b,0x31,0x3b,0x32,0x48],
      Self::ShiftEnd => vec![0x1b,0x5b,0x31,0x3b,0x32,0x46],
      Self::ShiftPageUp => vec![0x1b,0x5b,0x35,0x3b,0x32,0x7e], // unconfirmed
      Self::ShiftPageDown => vec![0x1b,0x5b,0x36,0x3b,0x32,0x7e], // unconfirmed
      Self::CtrlShiftDelete => vec![0x1b,0x5b,0x33,0x3b,0x36,0x7e],
      Self::CtrlShiftInsert => vec![0x1b,0x5b,0x32,0x3b,0x36,0x7e], // unconfirmed
      Self::CtrlShiftHome => vec![0x1b,0x5b,0x31,0x3b,0x36,0x48],
      Self::CtrlShiftEnd => vec![0x1b,0x5b,0x31,0x3b,0x36,0x46],
      Self::CtrlShiftPageUp => vec![0x1b,0x5b,0x35,0x3b,0x36,0x7e], // unconfirmed
      Self::CtrlShiftPageDown => vec![0x1b,0x5b,0x36,0x3b,0x36,0x7e], // unconfirmed
      Self::Escape => vec![0x1b],
      Self::F1 => vec![0x1b,0x4f,0x50],
      Self::F2 => vec![0x1b,0x4f,0x51],
      Self::F3 => vec![0x1b,0x4f,0x52],
      Self::F4 => vec![0x1b,0x4f,0x53],
      Self::F5 => vec![0x1b,0x5b,0x31,0x35,0x7e],
      Self::F6 => vec![0x1b,0x5b,0x31,0x37,0x7e],
      Self::F7 => vec![0x1b,0x5b,0x31,0x38,0x7e],
      Self::F8 => vec![0x1b,0x5b,0x31,0x39,0x7e],
      Self::F9 => vec![0x1b,0x5b,0x32,0x30,0x7e],
      Self::F10 => vec![0x1b,0x5b,0x32,0x31,0x7e],
      Self::F11 => vec![0x1b,0x5b,0x32,0x33,0x7e],
      Self::F12 => vec![0x1b,0x5b,0x32,0x34,0x7e],
      Self::CtrlF1 => vec![0x1b,0x5b,0x31,0x3b,0x35,0x50],
      Self::CtrlF2 => vec![0x1b,0x5b,0x31,0x3b,0x35,0x51],
      Self::CtrlF3 => vec![0x1b,0x5b,0x31,0x3b,0x35,0x52],
      Self::CtrlF4 => vec![0x1b,0x5b,0x31,0x3b,0x35,0x53],
      Self::CtrlF5 => vec![0x1b,0x5b,0x31,0x35,0x3b,0x35,0x7e],
      Self::CtrlF6 => vec![0x1b,0x5b,0x31,0x37,0x3b,0x35,0x7e],
      Self::CtrlF7 => vec![0x1b,0x5b,0x31,0x38,0x3b,0x35,0x7e],
      Self::CtrlF8 => vec![0x1b,0x5b,0x31,0x39,0x3b,0x35,0x7e],
      Self::CtrlF9 => vec![0x1b,0x5b,0x32,0x30,0x3b,0x35,0x7e],
      Self::CtrlF10 => vec![0x1b,0x5b,0x32,0x31,0x3b,0x35,0x7e],
      Self::CtrlF11 => vec![0x1b,0x5b,0x32,0x33,0x3b,0x35,0x7e],
      Self::CtrlF12 => vec![0x1b,0x5b,0x32,0x34,0x3b,0x35,0x7e],
      Self::ShiftF1 => vec![0x1b,0x5b,0x31,0x3b,0x32,0x50],
      Self::ShiftF2 => vec![0x1b,0x5b,0x31,0x3b,0x32,0x51],
      Self::ShiftF3 => vec![0x1b,0x5b,0x31,0x3b,0x32,0x52],
      Self::ShiftF4 => vec![0x1b,0x5b,0x31,0x3b,0x32,0x53],
      Self::ShiftF5 => vec![0x1b,0x5b,0x31,0x35,0x3b,0x32,0x7e],
      Self::ShiftF6 => vec![0x1b,0x5b,0x31,0x37,0x3b,0x32,0x7e],
      Self::ShiftF7 => vec![0x1b,0x5b,0x31,0x38,0x3b,0x32,0x7e],
      Self::ShiftF8 => vec![0x1b,0x5b,0x31,0x39,0x3b,0x32,0x7e],
      Self::ShiftF9 => vec![0x1b,0x5b,0x32,0x30,0x3b,0x32,0x7e],
      Self::ShiftF10 => vec![0x1b,0x5b,0x32,0x31,0x3b,0x32,0x7e],
      Self::ShiftF11 => vec![0x1b,0x5b,0x32,0x33,0x3b,0x32,0x7e],
      Self::ShiftF12 => vec![0x1b,0x5b,0x32,0x34,0x3b,0x32,0x7e],
      Self::CtrlShiftF1 => vec![0x1b,0x5b,0x31,0x3b,0x36,0x50],
      Self::CtrlShiftF2 => vec![0x1b,0x5b,0x31,0x3b,0x36,0x51],
      Self::CtrlShiftF3 => vec![0x1b,0x5b,0x31,0x3b,0x36,0x52],
      Self::CtrlShiftF4 => vec![0x1b,0x5b,0x31,0x3b,0x36,0x53],
      Self::CtrlShiftF5 => vec![0x1b,0x5b,0x31,0x35,0x3b,0x36,0x7e],
      Self::CtrlShiftF6 => vec![0x1b,0x5b,0x31,0x37,0x3b,0x36,0x7e],
      Self::CtrlShiftF7 => vec![0x1b,0x5b,0x31,0x38,0x3b,0x36,0x7e],
      Self::CtrlShiftF8 => vec![0x1b,0x5b,0x31,0x39,0x3b,0x36,0x7e],
      Self::CtrlShiftF9 => vec![0x1b,0x5b,0x32,0x30,0x3b,0x36,0x7e],
      Self::CtrlShiftF10 => vec![0x1b,0x5b,0x32,0x31,0x3b,0x36,0x7e],
      Self::CtrlShiftF11 => vec![0x1b,0x5b,0x32,0x33,0x3b,0x36,0x7e],
      Self::CtrlShiftF12 => vec![0x1b,0x5b,0x32,0x34,0x3b,0x36,0x7e],
      Self::CtrlA => vec![0x01],
      Self::CtrlB => vec![0x02],
      Self::CtrlC => vec![0x03],
      Self::CtrlD => vec![0x04],
      Self::CtrlE => vec![0x05],
      Self::CtrlF => vec![0x06],
      Self::CtrlG => vec![0x07],
      Self::CtrlH => vec![0x08],
      Self::Tab => vec![0x09],
      Self::ShiftTab => vec![0x1b,0x5b,0x5a], // or sometimes 0x1b 0x09
      Self::Linefeed => vec![0x0a],
      Self::CtrlK => vec![0x0b],
      Self::CtrlL => vec![0x0c],
      Self::Enter => vec![0x0d],
      Self::CtrlN => vec![0x0e],
      Self::CtrlO => vec![0x0f],
      Self::CtrlP => vec![0x10],
      Self::CtrlQ => vec![0x11],
      Self::CtrlR => vec![0x12],
      Self::CtrlS => vec![0x13],
      Self::CtrlT => vec![0x14],
      Self::CtrlU => vec![0x15],
      Self::CtrlV => vec![0x16],
      Self::CtrlW => vec![0x17],
      Self::CtrlX => vec![0x18],
      Self::CtrlY => vec![0x19],
      Self::CtrlZ => vec![0x1a],
      Self::Space => vec![0x20],
      Self::Backspace => vec![0x7f],
      Self::Menu => vec![0x1b,0x5b,0x32,0x39,0x7e],
      Self::CtrlMenu => vec![0x1b,0x5b,0x32,0x39,0x3b,0x35,0x7e],
      Self::ShiftMenu => vec![0x1b,0x5b,0x32,0x39,0x3b,0x32,0x7e],
      Self::CtrlShiftMenu => vec![0x1b,0x5b,0x32,0x39,0x3b,0x36,0x7e],
      Self::Char(c) => char_bytes(*c),
      Self::Byte(b) => vec![*b],
    }
  }
}

pub struct Decoder {
  index: usize,
  seq: [Option<u8>;6],
  lookahead: usize,
}

impl Default for Decoder {
  fn default() -> Self {
    Self {
      index: 0,
      seq: [None,None,None,None,None,None],
      lookahead: 0,
    }
  }
}

impl Decoder {
  pub fn new() -> Self {
    Self::default()
  }
  /// Write a single byte, generating zero or more key codes based on internal buffering and
  /// internal decoding states.
  pub fn write(&mut self, b: u8) -> Vec<KeyCode> {
    self.seq[self.index] = Some(b);
    self.index += 1;
    if self.lookahead > 0 {
      if (b >> 6) != 0b10 {
        let res = match self.seq {
          [Some(x0),Some(x1),None,None,None,None] => vec![
            KeyCode::Byte(x0),KeyCode::Byte(x1)
          ],
          [Some(x0),Some(x1),Some(x2),None,None,None] => vec![
            KeyCode::Byte(x0),KeyCode::Byte(x1),KeyCode::Byte(x2)
          ],
          [Some(x0),Some(x1),Some(x2),Some(x3),None,None] => vec![
            KeyCode::Byte(x0),KeyCode::Byte(x1),KeyCode::Byte(x2),KeyCode::Byte(x3)
          ],
          _ => panic!["unexpected keycode state"],
        };
        self.lookahead = 0;
        self.seq = [None,None,None,None,None,None];
        self.index = 0;
        return res;
      }
      let r = match (self.lookahead,self.seq) {
        (1,[Some(x0),Some(x1),None,None,None,None]) => {
          Some(char::from_u32(
            ((x0 as u32 & 0b00111111) << 6) | (x1 as u32 & 0b00111111)
          ).unwrap())
        },
        (2,[Some(x0),Some(x1),Some(x2),None,None,None]) => {
          Some(char::from_u32(
            ((x0 as u32 & 0b00011111) << 12) | ((x1 as u32 & 0b00111111) << 6)
            | (x2 as u32 & 0b00111111)
          ).unwrap())
        },
        (3,[Some(x0),Some(x1),Some(x2),Some(x3),None,None]) => {
          Some(char::from_u32(
            ((x0 as u32 & 0b00001111) << 18) | ((x1 as u32 & 0b00111111) << 12)
            | ((x2 as u32 & 0b00111111) << 6) | (x3 as u32 & 0b00111111)
          ).unwrap())
        },
        _ => None,
      };
      if let Some(c) = r {
        self.lookahead = 0;
        self.seq = [None,None,None,None,None,None];
        self.index = 0;
        return vec![KeyCode::Char(c)];
      } else {
        return vec![];
      }
    }
    let res = match self.seq {
      [Some(0x1b),None,None,None,None,None]
      | [Some(0x1b),Some(0x5b),None,None,None,None]
      | [Some(0x1b),Some(0x4f),None,None,None,None]
      | [Some(0x1b),Some(0x5b),Some(0x33|0x35|0x36),None,None,None]
      | [Some(0x1b),Some(0x5b),Some(0x31),None|Some(0x35|0x37|0x38|0x39),None,None]
      | [Some(0x1b),Some(0x5b),Some(0x32),None|Some(0x30|0x31|0x33|0x34),None,None] => {
        vec![]
      },
      [Some(0x1b),Some(0x5b),Some(0x41),None,None,None] => vec![KeyCode::ArrowUp],
      [Some(0x1b),Some(0x5b),Some(0x42),None,None,None] => vec![KeyCode::ArrowDown],
      [Some(0x1b),Some(0x5b),Some(0x43),None,None,None] => vec![KeyCode::ArrowRight],
      [Some(0x1b),Some(0x5b),Some(0x44),None,None,None] => vec![KeyCode::ArrowLeft],
      [Some(0x1b),Some(0x5b),Some(0x45),None,None,None] => vec![KeyCode::Numpad5],
      [Some(0x1b),Some(0x5b),Some(0x33),Some(0x7e),None,None] => vec![KeyCode::Delete],
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x7e),None,None] => vec![KeyCode::Insert],
      [Some(0x1b),Some(0x5b),Some(0x48),None,None,None] => vec![KeyCode::Home],
      [Some(0x1b),Some(0x5b),Some(0x46),None,None,None] => vec![KeyCode::End],
      [Some(0x1b),Some(0x5b),Some(0x35),Some(0x7e),None,None] => vec![KeyCode::PageUp],
      [Some(0x1b),Some(0x5b),Some(0x36),Some(0x7e),None,None] => vec![KeyCode::PageDown],
      [Some(0x1b),Some(0x4f),Some(0x50),None,None,None] => vec![KeyCode::F1],
      [Some(0x1b),Some(0x4f),Some(0x51),None,None,None] => vec![KeyCode::F2],
      [Some(0x1b),Some(0x4f),Some(0x52),None,None,None] => vec![KeyCode::F3],
      [Some(0x1b),Some(0x4f),Some(0x53),None,None,None] => vec![KeyCode::F4],
      [Some(0x1b),Some(0x5b),Some(0x31),Some(0x35),Some(0x7e),None] => vec![KeyCode::F5],
      [Some(0x1b),Some(0x5b),Some(0x31),Some(0x37),Some(0x7e),None] => vec![KeyCode::F6],
      [Some(0x1b),Some(0x5b),Some(0x31),Some(0x38),Some(0x7e),None] => vec![KeyCode::F7],
      [Some(0x1b),Some(0x5b),Some(0x31),Some(0x39),Some(0x7e),None] => vec![KeyCode::F8],
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x30),Some(0x7e),None] => vec![KeyCode::F9],
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x31),Some(0x7e),None] => vec![KeyCode::F10],
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x33),Some(0x7e),None] => vec![KeyCode::F11],
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x34),Some(0x7e),None] => vec![KeyCode::F12],
      [Some(0x1b),Some(x),None,None,None,None] => {
        self.lookahead = lookahead(x);
        if self.lookahead == 0 {
          vec![KeyCode::Escape,lookup1(x)]
        } else {
          self.seq = [Some(x),None,None,None,None,None];
          vec![KeyCode::Escape]
        }
      },
      [Some(0x1b),Some(0x5b|0x4f),Some(x),None,None,None] => {
        self.lookahead = lookahead(x);
        let c = self.seq[1].unwrap();
        if self.lookahead == 0 {
          vec![KeyCode::Escape,KeyCode::Char(char::from(c)),lookup1(x)]
        } else {
          self.seq = [Some(x),None,None,None,None,None];
          vec![KeyCode::Escape,KeyCode::Char(char::from(c))]
        }
      },
      [Some(0x1b),Some(0x5b),Some(0x31|0x32|0x33|0x35|0x36),Some(x),None,None] => {
        self.lookahead = lookahead(x);
        let c = self.seq[2].unwrap();
        if self.lookahead == 0 {
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(c)),
            lookup1(x)
          ]
        } else {
          self.seq = [Some(x),None,None,None,None,None];
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(c))
          ]
        }
      },
      [Some(0x1b),Some(0x5b),Some(0x31),Some(0x35|0x37|0x38|0x39),Some(x),None] => {
        let c = self.seq[3].unwrap();
        if self.lookahead == 0 {
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(0x31)),
            KeyCode::Char(char::from(c)),
            lookup1(x)
          ]
        } else {
          self.seq = [Some(x),None,None,None,None,None];
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(0x31)),
            KeyCode::Char(char::from(c))
          ]
        }
      },
      [Some(0x1b),Some(0x5b),Some(0x32),Some(0x30|0x31|0x33|0x34),Some(x),None] => {
        let c = self.seq[3].unwrap();
        if self.lookahead == 0 {
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(0x32)),
            KeyCode::Char(char::from(c)),
            lookup1(x)
          ]
        } else {
          self.seq = [Some(x),None,None,None,None,None];
          vec![
            KeyCode::Escape,
            KeyCode::Char(char::from(0x5b)),
            KeyCode::Char(char::from(0x32)),
            KeyCode::Char(char::from(c))
          ]
        }
      },
      [Some(x),None,None,None,None,None] => {
        self.lookahead = lookahead(x);
        if self.lookahead == 0 {
          vec![lookup1(x)]
        } else {
          vec![]
        }
      },
      _ => panic!["unhandled decode state: {:?}", self.seq],
    };
    if !res.is_empty() && self.lookahead == 0 {
      self.seq = [None,None,None,None,None,None];
      self.index = 0;
    }
    res
  }
}

fn lookahead(b: u8) -> usize {
  if b >= 0b11110000 { 3 }
  else if b >= 0b11100000 { 2 }
  else if b >= 0b11000000 { 1 }
  else { 0 }
}

fn lookup1(b: u8) -> KeyCode {
  match b {
    0x01 => KeyCode::CtrlA,
    0x02 => KeyCode::CtrlB,
    0x03 => KeyCode::CtrlC,
    0x04 => KeyCode::CtrlD,
    0x05 => KeyCode::CtrlE,
    0x06 => KeyCode::CtrlF,
    0x07 => KeyCode::CtrlG,
    0x08 => KeyCode::CtrlH,
    0x09 => KeyCode::Tab,
    0x0a => KeyCode::Linefeed, // CtrlJ
    0x0b => KeyCode::CtrlK,
    0x0c => KeyCode::CtrlL,
    0x0d => KeyCode::Enter, // CtrlM
    0x0e => KeyCode::CtrlN,
    0x0f => KeyCode::CtrlO,
    0x10 => KeyCode::CtrlP,
    0x11 => KeyCode::CtrlQ,
    0x12 => KeyCode::CtrlR,
    0x13 => KeyCode::CtrlS,
    0x14 => KeyCode::CtrlT,
    0x15 => KeyCode::CtrlU,
    0x16 => KeyCode::CtrlV,
    0x17 => KeyCode::CtrlW,
    0x18 => KeyCode::CtrlX,
    0x19 => KeyCode::CtrlY,
    0x1a => KeyCode::CtrlZ,
    0x20 => KeyCode::Space,
    0x7f => KeyCode::Backspace,
    c => KeyCode::Char(char::from(c)),
  }
}

fn char_bytes(c: char) -> Vec<u8> {
  let mut buf = [0u8;4];
  let n = c.encode_utf8(&mut buf).len();
  buf[0..n].to_vec()
}
