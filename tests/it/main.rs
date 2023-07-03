#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod combined;
mod foreign_types;
mod from;
mod into;
mod macros;
mod try_from;
mod try_into;

#[test]
fn test_combined_attributes() {
    combined::combined_attributes();
}

#[test]
fn test_foreign_types() {
    foreign_types::foreign_types();
}

#[test]
fn test_from() {
    from::from();
}

#[test]
fn test_into() {
    into::into();
}

#[test]
fn test_try_from() {
    try_from::try_from();
}

#[test]
fn test_try_from_custom_err() {
    try_from::try_from_custom_err();
}

#[test]
fn test_try_into() {
    try_into::try_into();
}

#[test]
fn test_try_into_custom_err() {
    try_into::try_into_custom_err()
}
