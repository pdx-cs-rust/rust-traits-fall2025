use std::fmt;
use std::ops::Div;

#[derive(Debug)]
struct Ratio {
    num: usize,
    den: usize,
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio { num: 1, den: 1 }
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

struct Integer(usize);

impl Div for Integer {
    type Output = Ratio;

    fn div(self, rhs: Integer) -> Self::Output {
        Ratio { num: self.0, den: rhs.0 }
    }
}

fn div_generic<T, U>(a: T, b: T) -> U
where T: Div<Output=U>
{
    a / b
}

fn main() {
    let r: Ratio = Integer(3) / Integer(6);
    println!("{}", r);

    let r = Ratio::default();
    println!("{}", r);
    let a: [Ratio; 17] = Default::default();
    println!("{:?}", a);

    println!("{}", div_generic(3usize, 6usize));
    println!("{}", div_generic(Integer(3), Integer(6)));
}
