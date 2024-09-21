// このエクササイズでは`Cow`というスマートポインターを探求する。
// このポインターは借用したデータを囲い込み、変更が可能なアクセスを可能にする。
// さらに変更や所有が必要になった場合にはデータを遅延してクローンすることができる。
// この型は`Borrow`トレイト経由で一般的な借用データを扱うために設計された。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // もしも所有していなければ配列にクローンを作る。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // クローンは`input`の変更が必要のため実行される。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // クローンは`input`の変更が必要ないため、実行されない。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: `Cow::Owned(_)`か`Cow::Borrowed(_)`に置き換えてください。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // `&`なしに`vec`のコンパイルが通るので、`Cow`は`vec`を直接所有している。
        // この場合、変更は発生せず(全ての要素はすでに絶対値になっているため)、クローンも発生しない。
        // しかしその結果は所有され続ける。なぜなら`vec`は決して借用や変更が行われないからである。

        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: `Cow::Owned(_)`か`Cow::Borrowed(_)`に置き換えてください。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // もちろん、変更が発生するケースも存在する。
        // その場合には`abs_all`関数で`to_mut()`を呼び出して以前に同じデータを参照を変換する。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: `Cow::Owned(_)`か`Cow::Borrowed(_)`に置き換えてください。
        assert!(matches!(input, todo!()));
    }
}
