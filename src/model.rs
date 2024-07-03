#![allow(unused)]

#[derive(Debug)]
pub struct Body<'b> {
    lang: &'b str,
    body: &'b str,
}
