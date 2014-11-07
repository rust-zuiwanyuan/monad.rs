pub struct Reader<'r,E,A:'r>(proc(E):'r -> A);

impl <'r,E,A> Reader<'r,E,A>
    where
        E:Clone,
{

    #[inline]
    pub fn run(self, e:E) -> A {
        let Reader(reader) = self;
        reader(e)
    }

    #[inline]
    pub fn point(a:A) -> Reader<'r,E,A> {
        Reader(proc(_) { a })
    }

    #[inline]
    pub fn bind<B>(self, f:proc(A) -> Reader<'r,E,B>) -> Reader<'r,E,B>
        where
            B:Clone,
    {
        Reader(proc(e:E) { f(self.run(e.clone())).run(e) })
    }

    #[inline]
    pub fn local(self, f:proc(E) -> E) -> Reader<'r,E,A> {
        Reader(proc(e) { self.run(f(e)) })
    }

}

#[inline]
pub fn ask<'r,E>() -> Reader<'r,E,E> {
    Reader(proc(e) { e })
}
