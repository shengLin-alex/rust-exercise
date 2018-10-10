// 疊代器模式(類似 .net 中的 IEnumerable)

fn main() {
    let v1 = vec![1, 3, 4];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("{}", value);
    }
}

#[test]
fn iterator_demonstration() {
    let v = vec![1, 2, 3, 4, 5];

    // 呼叫 next 會產生 consume, 因此使用 mut
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), Some(&4));
    assert_eq!(v_iter.next(), Some(&5));
}

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3, 4, 5];

    let v_iter = v.iter();

    assert_eq!(15, v_iter.sum());

    // println!("{:?}", v_iter); // 錯誤, 因為 iter 已經被 sum() consume
}

#[test]
fn use_map() {
    let v = vec![3, 4, 5, 5, 32, 34];

    // iterator 為惰性!
    // v.iter().map(|x| x + 1); // 產生警告, 因為沒有 consume map 所產生之 iter

    let new_iter = v.iter().map(|x| x + 1);

    // 使用 collect 來 consume
    let v2: Vec<_> = new_iter.collect();

    // 或使用 next 來 consume
    // assert_eq!(4, new_iter.next().unwrap());
    // assert_eq!(5, new_iter.next().unwrap());
    // assert_eq!(6, new_iter.next().unwrap());

    assert_eq!(vec![4, 5, 6, 6, 33, 35], v2);
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // shoes.iter() // 錯誤 iter() 無法獲取 vec 元素所有權, 只能取得 ref, 不符合回傳值的型態
    //     .filter(|s| s.size == shoe_size)
    //     .collect()

    shoes
        .into_iter() // 因此要用 into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

/// 把上面改成回傳 ref
fn shoes_in_my_size_ref<'a>(shoes: &'a [Shoe], shoe_size: u32) -> Vec<&'a Shoe> {
    // 由於 shoes 改成了切片 ref, 因此直接 iter()
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_shoes() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size_ref(&shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            &Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            &Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 為 Counter 實現疊代器模式
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // c1 [1,2,3,4,5]
    // c2 [2,3,4,5,None]
    // zip...
    // [(1,2),(2,3),(3,4),(4,5),"(5,None)"], 由於 zip 在任一個iter 回傳 None 時也會回傳 None
    // 所以實際上只有配對四組
    // map...
    // [2,6,12,20]
    // filter...
    // [6,12]
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
