
extern mod world;

fn main() {
    println!("hello {}", world::explore());
    fn recurse(limit: int) -> () {
        if((limit - 1) <= 0) {return;}
        recurse(limit - 1);
        println!("version: {}", world::exported_generator()(limit));
    }
    recurse(100);

}
