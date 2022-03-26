use itertools::*;
use rayon::prelude::*;

struct AllocatedVectors {
    pub radix_sort_bits_for_i: Vec<bool>,
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

    fn split(&mut self, input: &[u32]) {
        self.revert();
        self.prefix();
        self.suffix();
        self.suffix_result.par_iter_mut().for_each(|e| {
            *e = (input.len() as u32 + 1) - *e;
        });

        for (index_to_update, suffix_i, prefix_i, radix_bit) in izip!(
            self.split_indexes.iter_mut(),
            self.prefix_result.iter(),
            self.suffix_result.iter(),
            self.radix_sort_bits_for_i.iter()
        ) {
            if *radix_bit {
                *index_to_update = (*prefix_i - 1) as usize;
            } else {
                *index_to_update = (*suffix_i - 1) as usize;
            }
        }

        self.permute(input);
    }

    fn revert(&mut self) {
        self.revert_result.par_iter_mut().zip(self.radix_sort_bits_for_i.par_iter()).for_each(
            |(to_revert, flag)| {
                *to_revert = !(*flag);
            },
        );
    }

    fn prefix(&mut self) {
        let result_ptr = self.prefix_result.as_mut_ptr();
        unsafe {
            *(result_ptr.offset(0)) = match self.revert_result[0] {
                true => 1,
                false => 0,
            };
            for (index, flag) in self.revert_result.iter().enumerate().skip(1) {
                *result_ptr.add(index) = *result_ptr.add(index - 1)
                    + match flag {
                        true => 1,
                        false => 0,
                    };
            }
        }
    }

    fn suffix(&mut self) {
        let result_ptr = self.suffix_result.as_mut_ptr();
        let size = self.radix_sort_bits_for_i.len();
        unsafe {
            *result_ptr.add(size - 1) = match self.radix_sort_bits_for_i[size - 1] {
                true => 1,
                false => 0,
            };
            for (index, flag) in self.radix_sort_bits_for_i.iter().rev().enumerate().skip(1) {
                *result_ptr.add(size - index - 1) = *result_ptr.add(size - index)
                    + match flag {
                        true => 1,
                        false => 0,
                    };
            }
        }
    }

    fn permute(&mut self, input: &[u32]) {
        let result_ptr = self.permute_result.as_mut_ptr();
        unsafe {
            for (index, input_element) in self.split_indexes.iter().zip(input.iter()) {
                *result_ptr.add(*index) = *input_element;
            }
        }
    }
}

pub fn radix_sort(input_array: &mut Vec<u32>, max_value: u32) {
    let mut allocated_vectors = AllocatedVectors::new(input_array.len());
    let nb_bits = log2(max_value) + 1;
    for i in 0..nb_bits {
        allocated_vectors.radix_sort_bits_for_i.par_iter_mut().zip(input_array.par_iter()).for_each(
            |(bit, input_number)| {
                *bit = ((*input_number >> i) & 1) == 1;
            },
        );
        allocated_vectors.split(input_array.as_slice());
        *input_array = allocated_vectors.permute_result.clone();
    }
}

fn log2(x: u32) -> u32 {
    32 - x.leading_zeros()
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
        let mut input = vec![
            855, 953, 384, 106, 35, 215, 269, 674, 546, 189, 824, 500, 639, 231, 156, 619, 778,
            336, 797, 248,
        ];
        super::radix_sort(&mut input, 1000);
        assert_eq!(
            input,
            vec![
                35, 106, 156, 189, 215, 231, 248, 269, 336, 384, 500, 546, 619, 639, 674, 778, 797,
                824, 855, 953
            ]
        );
    }
}
