const fn num_bits<T>() -> usize { core::mem::size_of::<T>() * 8 }

pub const fn log_2(x: usize) -> usize {
    num_bits::<usize>() as usize - x.leading_zeros() as usize - 1
}