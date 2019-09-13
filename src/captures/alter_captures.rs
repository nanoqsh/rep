
#[derive(Copy, Clone, Debug)]
pub enum EitherCaptures<A, B> {
    Left(A),
    Right(B),
}

impl<'a, A, B> Iterator for EitherCaptures<A, B> where
    A: Iterator<Item=&'a str>,
    B: Iterator<Item=&'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherCaptures::Left(a) => a.next(),
            EitherCaptures::Right(b) => b.next(),
        }
    }
}
