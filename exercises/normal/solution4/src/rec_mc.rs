const cls: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
pub fn dp_rec_mc(mut amount: u32) -> u32 {
    let mut res = 0;
    for i in 0..cls.len() {
        let n;
        (amount, n) = (
            amount % cls[cls.len() - i - 1],
            amount / cls[cls.len() - i - 1],
        );
        res+=n;
        
    }
    res
}
