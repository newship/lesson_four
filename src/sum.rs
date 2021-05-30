//作业二：实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None

pub fn sum(arr: &[u32]) -> Option<u32> {
    let mut total: Option<u32> = Some(0);  
    for &i in arr {
        total = total.unwrap().checked_add(i);
    }  
    total
  }