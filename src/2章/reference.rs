// 参照による所有権の借用
// &を付けると変数の所有権を変えずに「参照できる」

fn myprint<T: std::fmt::Display>(msg: &T) {
  // 参照によってmsgを受け取る
  // 正確にはmsgという参照から値を取り出すので*msgとなるが、 // msgでもその参照が示す値と解釈される println!("{}", msg);
  }
  fn main() {
  let s = "Hello".to_string();
  myprint(&s); // 参照によって関数に渡している myprint(&s); // sが所有権を失わないので2回実行できる
  }