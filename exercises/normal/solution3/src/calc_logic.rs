pub fn new_birthday_probability(n: u32) -> f64 {
    // 实现一个算法，输入是人数(>=2)，
    // 计算任意一天同时存在两个及以上的人过生日的概率，
    // 保留四位小数。(20)
    if n < 2 {
        return 0.0;
    }

    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }

    // 计算至少有两个人生日相同的概率
    let result = 1.0 - probability;
    (result * 10000.0).round() / 10000.0
}
 