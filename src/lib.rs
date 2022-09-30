//#![deny(missing_docs)]
//#![deny(rustdoc::missing_doc_code_examples)]
//#![deny(rustdoc::broken_intra_doc_links)]
//#![deny(rustdoc::missing_crate_level_docs)]
//#![deny(rustdoc::missing_doc_code_examples)]
//#![deny(rustdoc::invalid_codeblock_attributes)]
//#![deny(rustdoc::invalid_html_tags)]
//#![deny(rustdoc::invalid_rust_codeblocks)]
//#![deny(rustdoc::bare_urls)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(array_windows)]

//! Savitzky-Golay Filter README
//! This is for the README

/// Savitzky-Golay Filter SUPER
pub fn sgfilter1(v: Vec<f32>) -> Vec<f32> {
    //! This is for my docs
    v
}
/// Savitzky-Golay Filter SUPER
pub fn sgfilter2(v: &Vec<f32>) -> &Vec<f32> {
    //! This is for my docs
    v
}
/// Savitzky-Golay Filter SUPER
pub fn sgfilter3(v: &mut [f32]) -> &mut [f32] {
    //! This is for my docs
    v.iter_mut().for_each(|i| *i /= 3.0);
    v
}
/// Savitzky-Golay Filter SUPER
pub fn sgfilter4(v: &mut [f32]) -> &mut [f32] {
    //! This is for my docs
    v.iter_mut().for_each(|i| *i /= 2.0);
    v
}
/// Savitzky-Golay Filter SUPER
pub fn sgfilter5(v: &mut [f32]) -> &mut [f32] {
    //! This is for my docs
    v
}
macro_rules! vec_builder {
    ([$($e1:expr),*$(,)?]$(*$n1:tt)?$(+[$($e:expr),*$(,)?]$(*$n:tt)?)*) => {{
        let mut v = Vec::new();
        let mut n = 1;
        $(n = $n1;)?
        for _ in 0..n {
            v.extend([$($e1),*]);
        }
        $(
            let mut n = 1;
            $(n = $n;)?
            for _ in 0..n {
                v.extend([$($e),*]);
            }
        )*
        v
    }};
}

pub fn sgfilter6(v: Vec<f32>) -> Vec<f32> {
    // 5 filler elements
    // [1,1,1,1,1 ...]
    let v2 = v.clone();
    let mut iter = v2.iter();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    v
}

// 10 interesting elemnts to "average"
// [..., 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, ...]
//               ^ m, window_size = 3
//          ^_______^
// 10 + 0 + 10/3
// applied to every element in the interesting subslice

// 5 filler elements
// [..., 1, 1, 1, 1, 1]
pub fn sgfilter7(v: Vec<f32>) -> Vec<f32> {
    if let [_, _, _, _, _, ref mid @ .., _, _, _, _, _] = *v {
        // what if I just take the windows and skip 5 of them? ===> sgfilter8
        //    mid.windows(3).map(|w| w.iter().sum()/3.0)
    } else {
        panic!("Size of Vec should be larger than 10");
    }
    v
}
// Strategy: take windows of size, and advance them to start of `mid`, then do calculation
// [1,1,1,1,1 ...]
// [..., 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, ...]
//               ^ m, window_size = 3
//           [-------]
// [..., 1, 1, 1, 1, 1]
pub fn sgfilter8(v: &mut Vec<f32>) {
    // Windows are defined only forward!
    // You need to calculate the initial element as (TODO! MATH!!!)
    let s = 5;
    let mid: Vec<f32> = v
        .array_windows::<3>()
        .skip(s-1)
        // TODO! MATH!!!
        .take(10+1)
        .map(|w| w.iter().sum::<f32>() / 3.)
        .collect();
    v.iter_mut().skip(s).zip(&mid).for_each(|(a, b)| *a = *b);
}

//vec![1,1,1 ..., (100 x 5), 1, 1, 1]
// Size == n + 10
pub fn build_data(n: i32) -> Vec<f32> {
    let mut start = vec![1.0, 1.0, 1.0, 1.0, 1.0];
    let mid: _ = std::iter::repeat([10.0, 0.0])
        .flatten()
        .take(n as usize)
        .collect::<Vec<f32>>();
    let end = vec![1.0, 1.0, 1.0, 1.0, 1.0];
    start.extend(mid);
    start.extend(end);
    start

    // let a: [f32; 106] = std::array::from_fn(|i| match i {
    //     3..=102 => 10.0*(i % 2) as f32,
    //     _ => 1.0,
    // });
    // Vec::from(a)
    // (0..n).map(|i| match i {
    //     3..=
    // })
    // Thank you all for cursed/blursed code <3
    // [0].into_iter()
    //     .chain([5; 100])
    //     .chain([1; 3])
    //     .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works1() {
        use crate::*;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = sgfilter1(v1.clone());
        assert_eq!(v1, v2);
    }
    #[test]
    fn it_works2() {
        use crate::*;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = sgfilter2(&v1);
        assert_eq!(v1, *v2);
    }
    #[test]
    fn it_works3() {
        use crate::*;
        let mut v1 = vec![3.0; 1_000_000];
        let v2 = sgfilter3(&mut v1);
        let v3 = vec![1.0; 1_000_000];
        assert_eq!(v3, v2);
    }
    #[test]
    fn it_works4() {
        use crate::*;
        let mut v1 = vec![10.0; 10];
        let v2 = sgfilter4(&mut v1);
        let v3 = vec![5.0; 10];
        assert_eq!(v3, v2);
    }
    #[test]
    fn slices_iters() {
        {
            let slice = &mut [10.0; 2];
            let mut iter = slice.iter_mut();
            *iter.next().unwrap() += 1.0;
            *iter.next().unwrap() += 1.0;
            assert_eq!(slice, &[11.0, 11.0]);
        }
        {
            let slice = &mut [10.0; 2];
            let iter = slice.iter_mut();
            iter.for_each(|i| *i += 1.0);
            assert_eq!(slice, &[11.0, 11.0]);
        }
        {
            let slice = &mut [10.0; 2];
            let iter = slice.iter_mut();
            iter.for_each(|i| *i /= 2.0);
            assert_eq!(slice, &[5.0, 5.0]);
        }
        {
            let slice = &mut [10.0; 2];
            let iter = slice.iter_mut();
            iter.for_each(|i| *i /= 2.0);
            assert_eq!(slice, &[5.0, 5.0]);
        }
    }
    #[test]
    fn create_slice() {
        let mut v = Vec::with_capacity(106); // []
        v.resize(3, 1); // [1,1,1]
        v.resize(103, 5); // [1,1,1, 5x100]
        v.resize(106, 1); // [1,1,1, 5x100, 1,1,1]
                          //let mut x = vec![1, 1, 1];
                          //let mut y = vec![5; 100];
                          //let mut z = vec![1, 1, 1];
                          // [1,1,1, (0, 1)x50, 1,1,1]
        let t = vec![1, 1, 1, 10, 0, 10, 0, 10, 0, 1, 1, 1];
        let a = std::iter::repeat([10, 0]).flatten().take(6);
        let mut m = vec![1, 1, 1];
        m.extend(a);
        m.extend([1, 1, 1]);
        assert_eq!(t, m);
        // Vec::from_iter()
    }
    #[test]
    fn data() {
        use crate::*;
        let v1 = build_data(10);
        let v2 = vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 10.0, 0.0, 10.0, 0.0, 10.0, 0.0, 10.0, 0.0, 10.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
        ];
        assert_eq!(v1.len(), 20);
        assert_eq!(v1, v2);
        // Praise be to Mara, Conjurer of Macro Sorcery :wizard:
        let v3 = vec_builder!([1.] * 5 + [10., 0.,] * 5 + [1.] * 5);
        assert_eq!(v2, v3);
    }
    #[test]
    fn dry_run() {
        use crate::*;
        // You can coerce a Vec<f64> into a Vec<f32>
        // But only works if you are using `literals`, otherwise this won't work.
        let mut v: Vec<f32> = vec_builder!([0.] * 5 + [9., 0.,] * 5 + [0.] * 5);
        let res: Vec<f32> =   vec_builder!([0.] * 5 + [3., 6.] * 4 + [3., 3.] + [0.] * 5);
        sgfilter8(&mut v);
        assert_eq!(v, res)
    }
}
