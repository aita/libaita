use rand::Rng;

// Fisher–Yates shuffle algorithm
pub fn shuffle<T, R>(v: &mut [T], rng: &mut R)
where
    R: Rng,
{
    for i in (1..v.len()).rev() {
        let j = rng.gen::<usize>() % (i + 1);
        v.swap(i, j);
    }
}
