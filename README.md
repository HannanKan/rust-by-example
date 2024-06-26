# 记录相关笔记

## 12. Cargo
### 项目结构
Ref: https://rustwiki.org/zh-CN/rust-by-example/cargo/deps.html

```shell
# 创建二进制 可执行程序 项目
cargo new foo
# 创建 库 项目
cargo new --lib foo

```

可执行程序的项目结构
```shell
foo
├── Cargo.toml
└── src
    └── main.rs

```
* main.rs 是新项目的入口源文件——这里没什么新东西。 
* Cargo.toml 是本项目（foo）的 cargo 的配置文件。 

假设我们要在同一个项目中有两个二进制可执行文件。 那要怎样做呢？

很显然，cargo 支持这一点。正如我们之前看到的，默认二进制名称是 main，但可以通过将文件放在 bin/ 目录中来添加其他二进制可执行文件：

```shell
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```
为了使得 cargo 编译或运行这个二进制可执行文件而不是默认或其他二进制可执行文件，我们只需给 cargo 增加一个参数 --bin my_other_bin，其中 my_other_bin 是我们想要使用的二进制可执行文件的名称。

除了可添加其他二进制可执行文件外，cargo 还支持更多功能，如基准测试，测试和示例。

### 测试
参考：https://rustwiki.org/zh-CN/book/ch11-00-testing.html

目录结构
```shell
foo
├── Cargo.toml
├── src
│   └── main.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs

```

运行单测命令
```shell
# 运行所有单测
cargo test
# 运行指定单测, 匹配 test_foo 前缀的 test
cargo test test_foo
```

### 构建脚本
构建脚本是在 编译之前运行的，比如代码生成等。 需要在 Cargo.toml 中指定
```shell
[package]
...
build = "build.rs"

```
