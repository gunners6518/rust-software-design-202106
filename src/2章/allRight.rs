// sの2回目では所有権を失う為、コンパイルエラーになる。

fn main() {
  let s = "hello".to_string();
  let t = s;
  printIn!("{}", s);
  printIn!("{}", t);
}