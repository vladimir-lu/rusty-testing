// using the hamcrest library
extern crate hamcrest;

use std::fmt;
use self::hamcrest::{assert_that, is, not, equal_to, contains, of_len, close_to, Matcher, MatchResult, expect};
use self::hamcrest::matchers::equal_to::EqualTo;
use ::{Cup, CupSize, Kettle, LiquidContainer};

#[test]
fn simple_assert() {
	assert_that(2, is(equal_to(4 / 2)));
}

#[test]
#[should_panic]
fn simply_wrong() {
	assert_that(42, is(equal_to(0)));
}

#[test]
fn simply_wrong_alt() {
	assert_that(42, is(not(equal_to(22))));
}

#[test]
fn vector_tests() {
	let v = vec![1, 2, 3, 5, 8];
	assert_that(&v, contains(vec!(1, 8)));
	assert_that(&v, is(of_len(5)));
}

#[test]
fn numeric_equality() {
	assert_that(1.0f64, is(close_to(1.0, 0.00001)));
}


// FIXME
// src/tests/with_hamcrest.rs:49:2: 49:13 error: the trait `hamcrest::core::Matcher<&Kettle>` is not implemented for the type `hamcrest::matchers::equal_to::EqualTo<&Cup>` [E0277]

// #[test]
// fn partial_equality() {
// 	let kettle = Kettle { capacity: 500, colour: (0, 0, 0) };
// 	let mug = Cup { size: CupSize::Large };
// 
// 	impl PartialEq for Cup {
// 		fn eq(&self, other: &Cup) -> bool {
// 			self.capacity() == other.capacity()
// 		}
// 	}
//
// 	assert_that(&kettle, equal_to(&mug));	
// }

