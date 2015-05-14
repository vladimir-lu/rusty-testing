// graciously copied from the quickcheck docs
extern crate quickcheck;

use self::quickcheck::quickcheck;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

#[test]
fn reverse_of_the_reverse() {
	fn prop(xs: Vec<isize>) -> bool {
        xs == reverse(&reverse(&xs))
    }
    quickcheck(prop as fn(Vec<isize>) -> bool);	
}
