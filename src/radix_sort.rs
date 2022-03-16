use rayon::prelude::*;

struct AllocatedVectors {
    pub radix_sort_bits_for_i : Vec<bool>,
    pub split_indexes: Vec<usize>,
    pub revert_result: Vec<bool>,
    pub prefix_result: Vec<u32>,
    pub suffix_result: Vec<u32>,
    pub permute_result: Vec<u32>,
}

impl AllocatedVectors {
    pub fn new(size: usize) -> Self {
        AllocatedVectors {
            radix_sort_bits_for_i: vec![false; size],
            split_indexes: vec![0; size],
            revert_result: vec![false; size],
            prefix_result: vec![0; size],
            suffix_result: vec![0; size],
            permute_result: vec![0; size],
        }
    }
}

pub fn radix_sort(input_array: &mut Vec<u32>, max_value: u32) {
    let mut allocated_vectors = AllocatedVectors::new(input_array.len());
    let nb_bits = log2(max_value).unwrap() + 1;
    for i in 0..nb_bits {
        for j in 0..input_array.len() {
            allocated_vectors.radix_sort_bits_for_i[j] = ((input_array[j] >> i) & 1) == 1;
        }
        split(&mut allocated_vectors, input_array.as_slice());
        *input_array = allocated_vectors.permute_result.clone();
    }
}

fn log2(mut x: u32) -> Result<u32, ()> {
    if x <= 0 {
        return Err(());
    }
    let mut result = 0u32;
    while x > 1 {
        x >>= 1;
        result += 1;
    }
    Ok(result)
}

fn split(allocated_vectors: &mut AllocatedVectors, input: &[u32]) {
    revert(allocated_vectors);
    prefix(allocated_vectors);
    suffix(allocated_vectors);
    for e in allocated_vectors.suffix_result.iter_mut() {
        *e = (input.len() as u32 + 1) - *e;
    }
    /*unsafe {
        (0..input.len()).into_par_iter().for_each(|i| {
            if allocated_vectors.radix_sort_bits_for_i[i] {
                allocated_vectors.split_indexes.get_unchecked_mut(i) = (allocated_vectors.suffix_result[i] - 1) as usize;
            } else {
                allocated_vectors.split_indexes[i] = (allocated_vectors.prefix_result[i] - 1) as usize;
            }
        });
    }*/

    unsafe {
        for i in 0..input.len() {
            if *allocated_vectors.radix_sort_bits_for_i.get_unchecked(i) {
                *allocated_vectors.split_indexes.get_unchecked_mut(i) = (allocated_vectors.suffix_result.get_unchecked(i) - 1) as usize;
            } else {
                *allocated_vectors.split_indexes.get_unchecked_mut(i) = (allocated_vectors.prefix_result.get_unchecked(i) - 1) as usize;
            }
        }
    }
    permute(allocated_vectors, input);
}

fn revert(allocated_vectors: &mut AllocatedVectors) {
    unsafe {
        for (index, flag) in allocated_vectors.radix_sort_bits_for_i.iter().enumerate() {
            //allocated_vectors.revert_result[index] = !(*flag);
            *(allocated_vectors.revert_result.get_unchecked_mut(index)) = !(*flag);
        }
    }
}

fn prefix(allocated_vectors: &mut AllocatedVectors) {
    allocated_vectors.prefix_result[0] = match allocated_vectors.revert_result[0] {
        true => 1,
        false => 0,
    };
    unsafe {
        for i in 1..allocated_vectors.revert_result.len() {
            /*allocated_vectors.prefix_result[i] = allocated_vectors.prefix_result[i - 1] + match allocated_vectors.revert_result[i] {
                true => 1,
                false => 0,
            };*/

            *(allocated_vectors.prefix_result.get_unchecked_mut(i)) = *(allocated_vectors.prefix_result.get_unchecked(i - 1)) + match allocated_vectors.revert_result.get_unchecked(i) {
                true => 1,
                false => 0,
            };
        }
    }
}

fn suffix(allocated_vectors: &mut AllocatedVectors) {
    let size = allocated_vectors.radix_sort_bits_for_i.len();
    allocated_vectors.suffix_result[size - 1] = match allocated_vectors.radix_sort_bits_for_i[size - 1] {
        true => 1,
        false => 0,
    };
    unsafe {
        for i in (0..(size - 1)).rev() {
            *allocated_vectors.suffix_result.get_unchecked_mut(i) = allocated_vectors.suffix_result.get_unchecked(i + 1) + match allocated_vectors.radix_sort_bits_for_i.get_unchecked(i) {
                true => 1,
                false => 0,
            };
        }
    }
}

fn permute(allocated_vectors: &mut AllocatedVectors, input: &[u32]) {
    unsafe {
        for i in 0..allocated_vectors.permute_result.len() {
            *(allocated_vectors.permute_result.get_unchecked_mut(*allocated_vectors.split_indexes.get_unchecked(i))) = *input.get_unchecked(i);
            //allocated_vectors.permute_result[allocated_vectors.split_indexes[i]] = input[i];
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_sorting() {
        let mut input = vec![5, 1021, 2, 9, 0, 23, 9, 512, 511, 8];
        super::radix_sort(&mut input, 1021);
        assert_eq!(input, vec![0, 2, 5, 8, 9, 9, 23, 511, 512, 1021]);
    }
}