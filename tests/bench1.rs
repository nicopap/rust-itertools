extern crate test;
extern crate itertools;

use itertools::Stride;


#[bench]
fn slice_iter(b: &mut test::Bencher)
{
    let xs = Vec::from_elem(20u, 20u);
    b.iter(|| for elt in xs.as_slice().iter() {
        test::black_box(elt);
    })
}

#[bench]
fn slice_iter_rev(b: &mut test::Bencher)
{
    let xs = Vec::from_elem(20u, 20u);
    b.iter(|| for elt in xs.as_slice().iter().rev() {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter(b: &mut test::Bencher)
{
    let xs = Vec::from_elem(20u, 20u);
    b.iter(|| for elt in Stride::from_slice(xs.as_slice(), 1) {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter_rev(b: &mut test::Bencher)
{
    let xs = Vec::from_elem(20u, 20u);
    b.iter(|| for elt in Stride::from_slice(xs.as_slice(), 1).rev() {
        test::black_box(elt);
    })
}
