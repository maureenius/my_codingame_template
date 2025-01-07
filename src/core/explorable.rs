use std::hash::Hash;

#[warn(dead_code)]
pub trait Explorable {
    /// 状態を表す型
    type State: Clone + Eq + Hash;
    /// 状態間を繋ぐ「移動/遷移」の型
    type Action: Clone;

    /// 指定した状態から移動（Action）できる先の (次の状態, 移動コスト, Action) の一覧
    fn successors(&self, state: &Self::State) -> Vec<(Self::State, i32, Self::Action)>;

    /// 終了判定 or 目的判定
    fn is_goal(&self, state: &Self::State) -> bool;
}
