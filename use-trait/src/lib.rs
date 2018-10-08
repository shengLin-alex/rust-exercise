mod summary;

#[cfg(test)]
mod tests {

    use super::summary::Summarizable;

    #[test]
    fn test_article_summary() {
        let article = super::summary::NewsArticle {
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("content")
        };

        assert_eq!("headline by author (location)", article.summary());
    }

    #[test]
    fn test_notify_article() {
        let article = super::summary::NewsArticle {
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("content")
        };

        assert_eq!("Breaking news! headline by author (location)", notify(article));
    }

    /// 定義一泛型方法，接受實做 Summarizalbe 的物件
    /// 
    /// 可以使用 + 號來增加單一型別參數的 bound eg. <T: Summarizable + clone>
    fn notify<T: Summarizable>(item: T) -> String {
        format!("Breaking news! {}", item.summary())
    }

    /// 可以改為使用 where 來增加約束
    fn notify_use_where<T>(item: T) -> String where T: Summarizable {
        format!("Breaking news! {}", item.summary())
    }

    /// 使用 trait 重寫尋找最大值之函數，由於需要使用運算符因此型別參數 T 必須實做 PartialOrd
    /// 
    /// 同時由於需要 move out from slice(不回傳 &T 的情況), 因此必須在加上 Copy trait.
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    /// 改為回傳 &T 的版本，不需要加上 Copy trait
    fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    /// 尋找最大值
    /// 
    /// pass a slice of i32, and fine the largest
    /// 
    /// 抽 method 為最常用的程式碼覆用之重構方式
    /// 
    /// 但如果今天要同時接受多種型別 slice 找最大值，泛型將會是最佳解
    fn larget(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}