extern crate closure;

use std::thread;
use std::time::Duration;

use closure::cacher::Cacher;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));

    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

/// 第一種重構, 將重複呼叫的方法移至變數
fn generate_workout_refactory_one(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

/// 第二種重構, 將重複呼叫的方法定義成一個閉包
fn generate_workout_refactory_two(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

/// 第三種重構, 使用怪取機制
/// 
/// 達到與第二種重構相同效果
fn generate_workout_refactory_three(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        println!("Finally, do {} fucks", expensive_result.value(100));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

/// 一般函數式
fn add_one_v1(x: u32) -> u32 { x + 1 }

/// 閉包
fn add_one_closure() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1 };
    let add_one_v4 = |x: u32| x + 1;
    // 以上三種表達接相同
}

fn call_closure_twice() {
    let example_closure = |x| x;

    let s = example_closure(String::new());
    // let n = example_closure(12); // 錯誤因為根據型別推斷, 編譯器已經認知該閉包參數為字串
}

fn main() {
    let simulated_user_specified_value: u32 = 10;
    let simulated_random_number: u32 = 7;

    generate_workout_refactory_three(simulated_user_specified_value, simulated_random_number);
}
