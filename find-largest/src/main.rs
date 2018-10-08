fn main() {
    let num_list = vec![3,90,32,1,54,342];
    let num_list2 = vec![4,324,4290,342,1,4];

    let larget1 = larget(&num_list);
    let largest2 = larget(&num_list2);
}

/// 尋找最大值
/// 
/// pass a slice of i32, and fine the largest
/// 
/// 抽 method 為最常用的程式碼覆用之重構方式
fn larget(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}