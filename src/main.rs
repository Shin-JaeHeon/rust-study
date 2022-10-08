#[path = "basic_study/ownership_and_slice_study.rs"]
mod ownership_and_slice_study;

fn main() {
    ownership_and_slice_study::safe_clear_string();
    ownership_and_slice_study::second_mutable();
}
