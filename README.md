# diom-gear-academy

## 测试 tips
这里通过 features 去实现

~~~toml
[features]
testr = []
~~~

~~~rust
#[cfg(not(feature = "testr"))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[cfg(feature = "testr")]
fn get_random_u32() -> u32 {
    0
}
~~~

测试命令
~~~bash
cargo t --features testr
~~~