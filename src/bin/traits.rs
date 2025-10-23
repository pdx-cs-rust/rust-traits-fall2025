use ordered_float::{FloatIsNan, NotNan};

const fn make_copies<const N: usize, T>(v: T) -> [T; N]
where T: Copy
{
    [v; N]
}

use std::marker::PhantomData;

#[derive(Debug, Eq)]
struct Hash<T> {
    h: u128,
    _phantom: PhantomData<T>,
}

impl<T> Hash<T> {
    fn new(_x: T) -> Self {
        return Self{ h: 7, _phantom: PhantomData };
    }
}

/*
    |
    x
   / \
  y  z
   \/
   w

  all y z: y > z || y < z
*/

impl<T> std::cmp::PartialEq for Hash<T> {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h
    }
}

fn compare_hashes() {
    let _h1: Hash<[u8; 3]> = Hash::new([0u8, 1, 2]);
    let _h2: Hash<&str> = Hash::new("hello");
    /*
    if h1 == h2 {
        println!("matched???");
    }
    */
}


static FIVE_SEVENS: [u32; 5] = make_copies(7);

fn nananananan() -> Result<(), FloatIsNan> {
    let x = 0.0f32 / 0.0;
    let y = x;
    eprintln!("{:?} {:?}", x, y);
    eprintln!("{:?} {:?} {:?}", x == y, x > y, x < y);

    let mut a = [NotNan::new(0.7)?, NotNan::new(0.5)?];
    a.sort();
    eprintln!("{:?}", a);

    let mut a = [NotNan::new(x)?, NotNan::new(y)?];
    a.sort();
    eprintln!("{:?}", a);

    Ok(())
}

trait Ugly {
    type HashVal;

    fn uglify(&self) -> String {
        "I'm ugly".to_string()
    }

    fn hash(&self) -> Self::HashVal;
}

impl Ugly for () {
    type HashVal = ();
    
    fn hash(&self) -> Self::HashVal {
        ()
    }
}

impl Ugly for u32 {
    type HashVal = u8;

    fn uglify(&self) -> String {
        format!("((({})))", self)
    }

    fn hash(&self) -> Self::HashVal {
        (self + (self >> 8) + (self >> 16) + (self >> 24)) as Self::HashVal
    }
}

impl Ugly for u8 {
    type HashVal = u8;
    
    fn uglify(&self) -> String {
        format!("b{}", self)
    }

    fn hash(&self) -> Self::HashVal {
        *self
    }
}

impl Ugly for str {
    type HashVal = u8;

    fn uglify(&self) -> String {
        format!(".xX{}Xx.", self)
    }

    fn hash(&self) -> Self::HashVal {
        self.bytes().fold(0, |a, b| a.wrapping_add(b))
    }
}

fn super_uglify<T: Ugly>(v: T) -> String {
    v.uglify().uglify()
}

fn main() {
    println!("{:?}", FIVE_SEVENS);

    compare_hashes();

    if nananananan().is_err() {
        println!("Hah. NaNs");
    }
    // nananananan().unwrap();

    println!("{}", 17u8.uglify());
    println!("{}", "hello world".uglify());
    println!("{}", super_uglify(17u32));
    println!("{}", ().uglify());
    println!("{:?}", 17u8.hash() == "hello world".hash());
}

