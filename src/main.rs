#[macro_use]
extern crate ndarray;
extern crate image;
extern crate rayon;

use ndarray::{Array, Array2, ArrayView2, ArrayViewMut2};
use rayon::prelude::*;

fn topple(mut pile: ArrayViewMut2<i64>) {
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
}

fn topple_threaded(mut pile: &mut Array2<i64>, n: usize) {
    // Break in to n parts which are threaded
    let x = pile.shape()[0];
    let y = pile.shape()[1];
    let mut sizes = vec!{};
    for _ in 0..n {sizes.push(x/n)}
    for i in 0..(x%n) {sizes[i]+=1}
    let mut pos = vec!{};
    pos.push(0);
    for i in 0..n {
        let a = pos[i];
        pos.push(a+sizes[i]);
    }
    let positions: Vec<(usize, usize)> = pos.iter().zip(pos.iter().skip(1)).map(|(&x, &y)| (x, y)).collect();
    let mut inside_bounds = vec!{};
    for i in 0..n {
        if i!=0 { inside_bounds.push(positions[i].0-1)}
        if i!=(n-1) {inside_bounds.push(positions[i].1)};
    }
    let mut boundary_pairs: Vec<(usize, usize)> = vec!{};
    for i in 0..inside_bounds.len()/2 {
        boundary_pairs.push((inside_bounds.pop().unwrap(), inside_bounds.pop().unwrap()));
    }
    while let Some(_) = get_first_untoppled(&pile.view()) {
        // for bounds in positions.par_iter() {
        //     topple(pile.slice_mut(s!((bounds.0)..(bounds.1), 0..y)));
        // }
        positions.iter().for_each(|bounds| {
            topple(pile.slice_mut(s!((bounds.0)..(bounds.1), 0..y)))
        });
        for pair in boundary_pairs.iter() {
            topple(pile.slice_mut(s!((pair.0-1)..(pair.1+1), 0..y)));
        }
    }
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

fn single_point() {
    // let colours = vec![
    //     [255,255,255],
	  //     [229,208,255],
    //     [204,163,255],
    //     [191,139,255]];
    let colours = vec![
        [26, 83, 92],
        [78, 205, 196],
        [247, 255, 247],
        [255, 107, 107],
        [255, 230, 109]
        ];
    let mut n = 0;
    loop {
        println!("On {} now", n);
        let res = 205;
        let mut a: Array2<i64> = Array::zeros((res, res));
        a[[102, 102]] = n*1050;
        topple(a.view_mut());
        //topple_threaded(&mut a, 1);
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, _>= image::ImageBuffer::new(res as u32, res as u32);
        for (i, &n) in a.iter().enumerate() {
            let x = i as u32 % res as u32;
            let y = i as u32 / res as u32;
            let pix = imgbuf.get_pixel_mut(x, y);
            if n > 0 && n < 4 {
                *pix = image::Rgb(colours[n as usize])
            } else if n >= 4 { *pix = image::Rgb(colours[4])}
            else { *pix = image::Rgb(colours[0])}
        }
        imgbuf.save(format!("sand_gif/sandpile_gif_{:04}.bmp", n)).unwrap();
        n += 1;
        if n>284 {break}
    }
}
fn four_points() {
    // let colours = vec![
    //     [255,255,255],
	  //     [229,208,255],
    //     [204,163,255],
    //     [191,139,255]];
    let colours = vec![
        [26, 83, 92],
        [78, 205, 196],
        [247, 255, 247],
        [255, 107, 107],
        [255, 230, 109]
        ];
    let mut n = 0;
    loop {
        println!("On {} now", n);
        let res = 182;
        let mut a: Array2<i64> = Array::zeros((res, res));
        a[[60, 60]] = n*250;
        a[[60, 121]] = n*250;
        a[[121, 60]] = n*250;
        a[[121, 121]] = n*250;
        topple(a.view_mut());
        //topple_threaded(&mut a, 1);
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, _>= image::ImageBuffer::new(res as u32, res as u32);
        for (i, &n) in a.iter().enumerate() {
            let x = i as u32 % res as u32;
            let y = i as u32 / res as u32;
            let pix = imgbuf.get_pixel_mut(x, y);
            if n > 0 && n < 4 {
                *pix = image::Rgb(colours[n as usize])
            } else if n >= 4 { *pix = image::Rgb(colours[4])}
            else { *pix = image::Rgb(colours[0])}
        }
        imgbuf.save(format!("four_points/sandpile_gif_{:04}.bmp", n)).unwrap();
        n += 1;
        if n>200 {break}
    }
}
fn main() {
    four_points();
}
