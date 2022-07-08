use rand::prelude::*;
use std::collections::HashSet;
use std::iter;
use std::time::Instant;

fn main() {
    // TODO: 引数を受け取ってパースしたい
    let (n_door, n_leftclose, n_trial) = (3, 2, 1000);
    println!(
        "doors: {}, to choice: {}, trial: {}",
        n_door, n_leftclose, n_trial
    );

    let t0 = Instant::now();
    simulate_monty_hall(n_door, n_leftclose, n_trial);
    println!("simulation time: {:?}", t0.elapsed());
}

fn simulate_monty_hall(n_door: usize, n_leftclose: usize, n_trial: usize) {
    let mut rng = rand::thread_rng();

    let results: Vec<(bool, bool)> = iter::repeat(0 as usize)
        .take(n_trial)
        .map(|_| simulate_monty_hall_once(n_door, n_leftclose, &mut rng))
        .collect();

    let n_hit_without_change: u32 = results.iter().map(|&(s, _)| s as u32).sum();
    println!(
        "staying case: {} hits / {} trials, prob={:.3}",
        n_hit_without_change,
        n_trial,
        n_hit_without_change as f64 / n_trial as f64
    );
    let n_hit_with_change: u32 = results.iter().map(|&(_, c)| c as u32).sum();
    println!(
        "changed case: {} hits / {} trials, prob={:.3}",
        n_hit_with_change,
        n_trial,
        n_hit_with_change as f64 / n_trial as f64
    );
}

/// n_door 枚のドアの中に 1枚だけ当たりがある。
/// プレイヤーが1枚選択した後、選択されたドアと当たりのドアを含んで n_leftclose枚になるよう、ハズレのドアを除去する
/// プレイヤーはドアを変えないか、変える（はじめに選んだドア以外から1枚選択）することができる
///
/// return (ドアを変えなかった場合に当たったかどうか, 変えた場合に当たったかどうか)
fn simulate_monty_hall_once(
    n_door: usize,
    n_leftclose: usize,
    rng: &mut ThreadRng,
) -> (bool, bool) {
    if !(n_leftclose >= 2) {
        panic!("n_leftclose must be >= 2.")
    }

    // 当たりのドア
    let door_car = (0..n_door).choose(rng).unwrap();

    // プレイヤーはドアを1つ選ぶ
    let door_chosen = (0..n_door).choose(rng).unwrap();
    // ドアを変えなかった場合に当たっているか？
    let is_hit_without_change = door_car == door_chosen;

    // 変える場合
    let doors_untouch = HashSet::from([door_car, door_chosen]); // モンティが触れないドア
    let mut doors_left: HashSet<usize> = HashSet::from_iter(0..n_door);
    doors_left.remove(&door_car);
    doors_left.remove(&door_chosen);
    let doors_left = doors_left
        .into_iter()
        .choose_multiple(rng, n_leftclose - doors_untouch.len());

    let mut doors_candidate: HashSet<usize> = HashSet::from_iter(doors_left); // プレイヤーが再選択する候補
    doors_candidate.insert(door_car);
    doors_candidate.remove(&door_chosen);

    // プレイヤーが再選択したドア
    let door_changed = doors_candidate.into_iter().choose(rng).unwrap();
    let is_hit_with_change = door_car == door_changed;

    (is_hit_without_change, is_hit_with_change)
}
