#[cfg(test)]
mod test;

macro_rules! declare_sort {
    ( $x:ident ) => {
        pub use $x::$x;
        mod $x;
    };
}

declare_sort!(bubble_sort);
declare_sort!(comb_sort);
declare_sort!(cycle_sort);
declare_sort!(dual_quick_sort);
declare_sort!(gnome_sort);
declare_sort!(heap_sort);
declare_sort!(insertion_sort);
declare_sort!(merge_sort);
declare_sort!(odd_even_sort);
declare_sort!(quick_sort);
declare_sort!(selection_sort);
declare_sort!(shaker_sort);
declare_sort!(shell_sort);
declare_sort!(stooge_sort);
