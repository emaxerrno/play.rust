#[crate_id = "world#0.9.8"];
pub fn explore() -> &'static str { "world" }


pub fn exported_generator () -> |int| -> int {
    |prev: int| -> int {
        prev + 1
    }
}
