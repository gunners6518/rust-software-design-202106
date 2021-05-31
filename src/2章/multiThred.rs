// クロージャーを利用したマルチスレッド化の例

fn main() {
  // スレッド数
  const N_THREADS : usize = 3;
  // 処理する整数のRange
  let series_range = 0..30; let add = 1;
  // 1
  let chunks = (0..N_THREADS)
          .map(¦ii¦ series_range.clone().skip(ii).step_by(N_THREADS));
  // 2
  let handles : Vec<_> = chunks
          .map(¦vv¦ std::thread::spawn(move ¦¦ {
          })  vv.for_each(¦nn¦ print!("{},", nn + add));
          ).collect();
  // 3各スレッドの終了を待つ
  handles.into_iter().for_each(¦hh¦ hh.join().unwrap());
  }

  // ❶の説明)
  // series_rangeをN_THREADS個に分割する。
  // it.skip(ii)はitの先頭からii個をスキップする。
  // it.step_by(N_THREADS)はitからN_ THREADS個ごとに取り出す。
  // 結果、chunks = [[0,3,6,...], [1,4,7,...], [2,5,8...]]となる。
  // ただし、mapは遅延処理なので、この段階では評価されていない。series_rangeはcloneする。 
  // cloneをしないと、map内のクロージャにある series_rangeに所有権が移動し、あとでseries_ rangeが使えなくなってしまう

  // ❷の説明)
  // chunksの[0,3,6,...], [1,4,7,...], [2,5,8...]のそれぞれの処理を行うスレッドを起動して、
  // それぞれの配列の要素にaddを加えた 値を出力する。
  // mapは遅延処理で実際 に必要になるときまでmap内のクロージャが実行されないので、
  // collectで それぞれの要素を要求して実行させている。
  // スレッドを起動するspawnのクロージャにはmoveを付けている。
  // これは、スコープ内にある変数(この場合 はadd)の所有権を強制的にクロージャ内に移動させる。
  // moveのあとの¦¦はクロージャに引数がないことを示す