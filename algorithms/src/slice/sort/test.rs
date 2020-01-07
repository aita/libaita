use super::*;

macro_rules! test_sort {
    ($name:ident) => {
        mod $name {
            use super::*;
            use rand::seq::SliceRandom;
            use rand::thread_rng;

            #[test]
            fn sort_random_slice_100() {
                let v: Vec<_> = (0..100).collect();
                let mut rng = thread_rng();
                let mut w = v.to_vec();
                w[..].shuffle(&mut rng);
                $name(&mut w, |a, b| a.cmp(b));
                assert_eq!(v, w)
            }
        }
    };
}

test_sort!(bubble_sort);
test_sort!(comb_sort);
test_sort!(cycle_sort);
test_sort!(dual_quick_sort);
test_sort!(gnome_sort);
test_sort!(heap_sort);
test_sort!(insertion_sort);
test_sort!(merge_sort);
test_sort!(odd_even_sort);
test_sort!(quick_sort);
test_sort!(selection_sort);
test_sort!(shell_sort);
test_sort!(shaker_sort);
test_sort!(stooge_sort);
