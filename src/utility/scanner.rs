use std::{
    fmt::Debug,
    io::{stdin, Read},
    str::{FromStr, SplitWhitespace},
};

pub struct Scanner<'a> {
    ptr: *mut String,
    iter: SplitWhitespace<'a>,
}

impl Scanner<'_> {
    pub fn new() -> Scanner<'static> {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();
        let ptr = Box::leak(Box::new(buf));
        Scanner {
            ptr,
            iter: ptr.split_whitespace(),
        }
    }

    pub fn next_token<T>(&mut self) -> T
    where
        T: Ord + FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.iter.next().unwrap().parse().unwrap()
    }

    pub fn next_vec<T>(&mut self, size: usize) -> Vec<T>
    where
        T: Ord + FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut res = Vec::with_capacity(size);
        (0..size).for_each(|_| res.push(self.next_token()));
        res
    }
    pub fn next_chars(&mut self) -> Vec<char> {
        self.iter.next().unwrap().chars().collect()
    }
    pub fn next_bytes(&mut self) -> Vec<u8> {
        self.iter.next().unwrap().bytes().collect()
    }
}

impl Drop for Scanner<'_> {
    fn drop(&mut self) {
        let _x = unsafe { Box::from_raw(self.ptr) };
    }
}
