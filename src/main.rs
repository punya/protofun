use protobuf::prelude::*;

include!(concat!(env!("OUT_DIR"), "/protobuf_generated/generated.rs"));

fn main() {
    let foo_base = proto!(Foo {
        bar: Bar { baz: 42 },
        maybe_bar: Bar { baz: 33 },
    });

    let mut foo_builder = foo_base;
    foo_builder.bar_mut().set_baz(16);

    let foo = foo_builder;

    println!("{foo:?}");
}
