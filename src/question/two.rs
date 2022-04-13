// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None

fn claute(arr: &[u32]) -> Option<u32> {
    let mut sum = 0;

    for item in arr.iter() {
        sum += item;
    }

    Some(sum)
}

pub fn sum_num(arr: &[u32]) {
    let sum = claute(arr).unwrap();
    println!("[1,2,3] 集合的和: {}", sum);
}
