pub fn radix_sort(input_array: &mut Vec<u32>, max_value: u32) {
    let nb_bits = log2(max_value).unwrap() + 1;
    for i in 0..nb_bits {
        let mut bits_for_i : Vec<bool> = Vec::new();
        for j in 0..input_array.len() {
            bits_for_i.push(((input_array[j] >> i) & 1) == 1);
        }
        *input_array = split(input_array.as_slice(), bits_for_i.as_slice());
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

fn split(input: &[u32], flags: &[bool]) -> Vec<u32> {
    let mut indexes : Vec<usize> = Vec::new();
    indexes.resize(input.len(), 0);
    let ldown = prefix(revert(flags).iter().map(|x| -> u32 {
        match x {
            true => 1,
            false => 0,
        }
    }).collect::<Vec<_>>().as_slice());
    let mut lup = suffix(flags.iter().map(|x| -> u32 {
        match x {
            true => 1,
            false => 0,
        }
    }).collect::<Vec<_>>().as_slice());
    for e in lup.iter_mut() {
        *e = (input.len() as u32 + 1) - *e;
    }
    for i in 0..input.len() {
        if flags[i] {
            indexes[i] = (lup[i] - 1) as usize;
        } else {
            indexes[i] = (ldown[i] - 1) as usize;
        }
    }
    permute(input, indexes.as_slice())
}

fn revert(flags: &[bool]) -> Vec<bool> {
    let mut result = Vec::new();
    for flag in flags {
        result.push(!(*flag));
    }
    result
}

fn prefix(input: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    result.push(input[0]);
    for i in 1..input.len() {
        result.push(input[i] + result[i - 1]);
    }
    result
}

fn suffix(input: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    result.resize(input.len(), 0);
    result[input.len() - 1] = input[input.len() - 1];
    for i in (0..(input.len() - 1)).rev() {
        result[i] = input[i] + result[i + 1];
    }
    result
}

fn permute(input: &[u32], indexes: &[usize]) -> Vec<u32> {
    let mut result = Vec::new();
    result.resize(input.len(), 0);
    for i in 0..result.len() {
        result[indexes[i]] = input[i];
    }
    result
}