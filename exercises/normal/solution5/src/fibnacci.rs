pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let (mut p, mut q) = (0u32, 1u32);
    let mut sum = 1u32;
    loop {
        let t = p + q;
        if (t > threshold) {
            break;
        }
        if t % 2 == 1 {
            sum += t;
        }
        p = q;
        q = t;
        // print!("{} ", q);
    }
    // println!("{}", sum);
    sum
}
