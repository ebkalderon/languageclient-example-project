pub trait Foo {
    type Bar;
}

struct MyVeryLongNamedType;

type Alias = MyVeryLongNamedType;

struct Inner;

impl Foo for Inner {
    type Bar = MyVeryLongNamedType;
}

struct Wrapper<T>(T);

impl<T> Wrapper<T>
where
    T: Foo<Bar = Alias>,
{
    pub fn new(foo: T) -> Self {
        Wrapper(foo)
    }

    pub fn say_foo(&self) {
        println!("Foo bar baz quux");
        println!("hello world");
    }
}

fn main() {
    let thing = Wrapper::new(Inner);
    thing.say_foo();
}
