// the spirit
// 如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs 的文件中。
// 如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件中。

// define in client.rs
pub mod client;

// define in network.rs
pub mod network;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn foo() {

            }
        }
    }
}

#[cfg(test)]
mod tests {

    // use super in child mod.
    use super::client;
    use super::a::series::of;

    #[test]
    fn it_works() {

        client::connect();
        of::foo();
    }
}