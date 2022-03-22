use terminal_keycode::KeyCode;

fn main() {
  println!["{:?}", KeyCode::ArrowLeft.bytes()]; // [27, 91, 68]
  println!["{:?}", KeyCode::CtrlShiftF1.bytes()]; // [27, 91, 49, 59, 54, 80]
  println!["{:?}", KeyCode::Char('F').bytes()]; // [70]
  println!["{:?}", KeyCode::Home.bytes()]; // [27, 91, 72]
}
