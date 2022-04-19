#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub fn z(_: [u32; 1]) -> u32 {
    0
}

pub fn n(x: [u32; 1]) -> u32 {
    x[0] + 1
}

pub fn u<const N: usize, const K: usize>(x: [u32; N]) -> u32 {
    x[K - 1]
}

pub fn s<const N: usize, const K: usize, G>(
    g: G,
    f: [fn([u32; N]) -> u32; K],
) -> impl Fn([u32; N]) -> u32 + Copy
where
    G: Fn([u32; K]) -> u32 + Copy,
{
    move |x: [u32; N]| {
        let mut res = [0; K];
        for (func, arg) in f.iter().zip(res.iter_mut()) {
            *arg = func(x);
        }
        g(res)
    }
}

pub fn r<const N: usize, F, G>(f: F, g: G) -> impl Fn([u32; N + 1]) -> u32 + Copy
where
    F: Fn([u32; N]) -> u32 + Copy,
    G: Fn([u32; N + 2]) -> u32 + Copy,
    [(); N + 1]:,
{
    move |x: [u32; N + 1]| {
        let mut x_n = [0; N];
        let mut g_args = [0; N + 2];
        (0..N).for_each(|i| {
            x_n[i] = x[i];
            g_args[i] = x[i];
        });
        let mut res = f(x_n);
        for yi in 0..x[N] {
            g_args[N] = yi;
            g_args[N + 1] = res;
            res = g(g_args);
        }
        res
    }
}
