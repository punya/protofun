use protobuf::proto;

include!(concat!(env!("OUT_DIR"), "/protobuf_generated/generated.rs"));

fn main() {
    let foo = proto!(Foo {
        bar: Bar {
            baz: 42,
        },
        maybe_bar: Bar {
            baz: 33,
        },
    });

    println!("{foo:?}");
}
