

fn main() {
    trait AlexTraitOnStr {
        fn foo(&self);
    }
    impl AlexTraitOnStr for ~str {
        fn foo(&self) {
            println!("{}", *self);
            println!("added foo to ~str dynamically in this scope only");
        }
    }
    let s = ~"Ok these dynamic method dispatch is insane.\n";
    s.foo();
}
