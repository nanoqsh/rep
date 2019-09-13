
#[derive(Copy, Clone, Debug)]
pub struct DoubleCaptures<A, B> {
    a: Option<A>,
    b: B,
}

impl<A, B> DoubleCaptures<A, B> {
    pub fn new(a: A, b: B) -> DoubleCaptures<A, B> {
        DoubleCaptures {
            a: Some(a),
            b,
        }
    }
}

impl<'a, A, B> Iterator for DoubleCaptures<A, B> where
    A: Iterator<Item=&'a str>,
    B: Iterator<Item=&'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.a {
            Some(it) => {
                match it.next() {
                    a @ Some(_) => a,
                    None => {
                        self.a.take();
                        self.b.next()
                    }
                }
            },
            None => self.b.next(),
        }
    }
}
