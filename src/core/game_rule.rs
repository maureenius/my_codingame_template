use crate::core::score::Score;

#[warn(dead_code)]
pub trait GameRule {
    type State;  // ゲーム状態を表す型 (構造体)
    type Action;  // 1手(または1ターンに出せるコマンド)を表す型

    /// 指定した状態で取りうる行動の列挙
    fn possible_actions(&self, state: &Self::State) -> Vec<Self::Action>;

    /// 状態に行動を適用して次の状態を生成
    fn apply_action(&self, state: &Self::State, action: &Self::Action) -> Self::State;

    /// 状態を評価する（スコアリング）
    fn evaluate(&self, state: &Self::State) -> Score;
}