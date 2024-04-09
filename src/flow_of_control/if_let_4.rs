// This enum purposely neither implements nor derives PartialEq
// That is why comparing Foo::Bar == a a fails below

enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        // ^ this cause a compile-time error. Use `if let` instead.
        println!("a  is foobar");
    }
}