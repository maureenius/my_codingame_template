use crate::core::explorable::Explorable;

pub fn bfs_search<E: Explorable>(explorable: &E, start: E::State) -> Option<Vec<E::Action>> {
    use std::collections::{VecDeque, HashMap};

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start.clone(), None); // 訪れた経路の (盤面, 遷移する行動)

    while let Some(current) = queue.pop_front() {
        if explorable.is_goal(&current) {
            // 経路を復元する
            let mut path = Vec::new();
            let mut point = current;
            while let Some((prev_point, action)) = visited[&point].clone() {
                path.push(action);
                point = prev_point;
            }
            path.reverse();
            return Some(path);
        }

        for (next, _cost, action) in explorable.successors(&current) {
            if !visited.contains_key(&next) {
                queue.push_back(next.clone());
                visited.insert(next, Some((current.clone(), action)));
            }
        }
    }

    None
}