// 関数での所有権
// 関数実行時の引数渡しでも所有権の移動やコピーはおこる

fn myprint<T: std::fmt::Display>(msg: T) {
}   println!("{}", msg);
fn main() {
let s = "Hello".to_string();
myprint(s); // sの所有権が関数内の変数に移動
myprint(s); // sの所有権は移動し、初期化されていない変数になるのでエラーになる
}