use std::result::Result;
use std::option::Option;

pub trait Functor<A, B, F> {
  type Output;
  fn fmap(&self, f: &F) -> <Self as Functor<A, B, F>>::Output;
}

impl<A, B, F> Functor<A, B, F> for Vec<A>
where F:Fn(&A) -> B {
  type Output = Vec<B>;
  fn fmap(&self, f: &F) -> Self::Output {
    let mut result = Vec::with_capacity(self.len());
    for data in self {
      result.push(f(data));
    }
    result
  }
}

#[test]
fn vec_functor_test0() {
    let source = vec![0, 1, 2, 3, 4];
    let f = |x:&i32| x*2;
    let data = source.fmap(&f);
    assert_eq!(data, vec![0, 2, 4, 6, 8]);
}
