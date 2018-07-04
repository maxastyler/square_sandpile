#[macro_use]
extern crate ndarray;

use ndarray::{Array, Array2, ArrayView2, ArrayViewMut2};

fn topple(mut pile: ArrayViewMut2<i64>) -> Array2<i64> {
    // Topples a sandpile, using the edges as sinks.
    let dim = (pile.shape()[0], pile.shape()[1]);
    let mut collapse_queue: Vec<(usize, usize)> = vec!();
    while let Some(next_pos) = get_first_untoppled(&pile.view()) {
        pile[next_pos]-=4;
        for i in get_surrounding(next_pos).iter() {
            pile[*i]+=1;
            if !on_rect_edge(*i, dim) && pile[*i]>=4 {collapse_queue.push(*i)}
        }
        while let Some(i) = collapse_queue.pop() {
            if pile[i] >= 4 {
                pile[i] -= 4;
                for j in get_surrounding(i).iter() {
                    pile[*j]+=1;
                    if !on_rect_edge(*j, dim) && pile[*j]>=4 {collapse_queue.push(*j)}
                }
            }
        }
    }
    Array::<i64, _>::zeros([3, 4])
}

fn on_rect_edge(pos: (usize, usize), rect_dim: (usize, usize)) -> bool {
    if pos.0 > 0 && pos.0 < rect_dim.0-1 && pos.1 > 0 && pos.1 < rect_dim.0-1 {
        false } else { true }
}

fn get_first_untoppled(pile: &ArrayView2<i64>) -> Option<(usize, usize)> {
    let s = pile.shape();
    for x in 0..s[0] {
        for y in 0..s[1] {
            if pile[(x, y)]>=4 && !on_rect_edge((x, y), (s[0], s[1])) { return Some((x, y)) }
        }
    }
    None
}

fn get_surrounding(pos: (usize, usize)) -> [(usize, usize); 4]{
    [(pos.0-1, pos.1), (pos.0+1, pos.1), (pos.0, pos.1-1), (pos.0, pos.1+1)]
}

fn main() {
    let mut a: Array2<i64> = Array::zeros((10, 10));
    a[[5, 5]] = 100000;
    topple(a.view_mut());
    println!("{:?}", a);
    let b: i64 = a.iter().sum();
    println!("{}", b);
}
