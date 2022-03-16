use std::ops::Deref;
use rayon::prelude::*;
use itertools::*;

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
        for (bit, input_number) in allocated_vectors.radix_sort_bits_for_i.iter_mut().zip(input_array.iter()) {
            *bit = ((*input_number >> i) & 1) == 1;
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

    for (index_to_update, suffix_i, prefix_i, radix_bit) in izip!(allocated_vectors.split_indexes.iter_mut(), allocated_vectors.prefix_result.iter(), allocated_vectors.suffix_result.iter(), allocated_vectors.radix_sort_bits_for_i.iter()) {
        if *radix_bit {
            *index_to_update = (*prefix_i - 1) as usize;
        } else {
            *index_to_update = (*suffix_i - 1) as usize;
        }
    }


    permute(allocated_vectors, input);
}

fn revert(allocated_vectors: &mut AllocatedVectors) {
    for (to_revert, flag) in allocated_vectors.revert_result.iter_mut().zip(allocated_vectors.radix_sort_bits_for_i.iter()) {
        *to_revert = !(*flag);
    }
}

fn prefix(allocated_vectors: &mut AllocatedVectors) {
    let result_ptr = allocated_vectors.prefix_result.as_mut_ptr();
    unsafe {
        *(result_ptr.offset(0)) = match allocated_vectors.revert_result[0] {
            true => 1,
            false => 0,
        };
        for(index, flag) in allocated_vectors.revert_result.iter().enumerate().skip(1) {
            *(result_ptr.offset(index as isize)) = *(result_ptr.offset((index - 1) as isize)) + match flag {
                true => 1,
                false => 0,
            };
        }
    }
}

fn suffix(allocated_vectors: &mut AllocatedVectors) {
    let result_ptr = allocated_vectors.suffix_result.as_mut_ptr();
    let size = allocated_vectors.radix_sort_bits_for_i.len();
    unsafe {
        *(result_ptr.offset((size - 1) as isize)) = match allocated_vectors.radix_sort_bits_for_i[size - 1] {
            true => 1,
            false => 0,
        };
        for (index, flag) in allocated_vectors.radix_sort_bits_for_i.iter().rev().enumerate().skip(1) {
            *(result_ptr.offset((size - index - 1) as isize)) = *(result_ptr.offset((size - index) as isize)) + match flag {
                true => 1,
                false => 0,
            };
        }
    }
}

fn permute(allocated_vectors: &mut AllocatedVectors, input: &[u32]) {
    let result_ptr = allocated_vectors.permute_result.as_mut_ptr();
    unsafe {
        for (index, input_element) in allocated_vectors.split_indexes.iter().zip(input.iter()) {
            *(result_ptr.offset(*index as isize)) = *input_element;
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_sorting_course() {
        let mut input = vec![5, 1021, 2, 9, 0, 23, 9, 512, 511, 8];
        super::radix_sort(&mut input, 1021);
        assert_eq!(input, vec![0, 2, 5, 8, 9, 9, 23, 511, 512, 1021]);
    }

    #[test]
    fn test_sorting_20() {
        let mut input = vec![855, 953, 384, 106, 35, 215, 269, 674, 546, 189, 824, 500, 639, 231, 156, 619, 778, 336, 797, 248];
        super::radix_sort(&mut input, 1000);
        assert_eq!(input, vec![35, 106, 156, 189, 215, 231, 248, 269, 336, 384, 500, 546, 619, 639, 674, 778, 797, 824, 855, 953]);
    }
}