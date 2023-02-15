mod atomics_and_memory_ordering;
mod channels;
mod declarative_macros;
mod dispatch_and_fat_pointers;
mod drop_check;
mod functions_closures_and_their_traits;
mod iterators;
mod lifetime_annotations;
mod smart_pointers_and_interior_mutability;
mod subtyping_and_variance;

use functions_closures_and_their_traits::call_me;

fn main() {
    call_me::main();
}
