***Rust学习记录***

***基于Rust 官方教程Rust Book流程进行学习记录***

# Hello World!

首先，Rust 风格的缩进使用 4 个空格，而不是制表符。

其次，println! 调用 Rust 宏。如果改为调用函数，则应该将其输入为 println（不含 !）。

第三，你看到 "Hello, world!" 字符串。我们将这个字符串作为参数传递给 println!，接着 println! 将字符串打印到屏幕上。

第四，我们用分号（;，注意这是英文分号）结束该行，这表明该表达式已结束，下一个表达式已准备好开始。
Rust 代码的大多数行都以一个 ; 结尾。

`fn main() { }` 定义了Rust的函数
函数主体同样使用大括号 { } 括起来

## Hello Cargo!
*Cargo 是 Rust 的构建系统和包管理器*

在上一个"Hello World"程序中,是没有包含任何依赖的。
在编写复杂的Rust程序时,Cargo是必不可少的

检查是否安装Cargo:
`cargo --version`
看到版本号则说明安装成功！

>命令行创建(兼容任何操作系统):
> cargo new hello_cargo
> cd hello_cargo

进入目录会发现Cargo自动帮我们初始化了一个Git仓库,看上去非常不错,但有一点需要注意:
*如果在现有Git中运行cargo new是不生效的,但你也可以使用cargo new --vcs=git强制创建*

Rust非常提倡我们使用Cargo来构建和运行项目,下面是一个*实例*:

`cargo build
  Compiling hello_cargo v0.1.1(file://"你的路径"/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs`

我们来运行它:
`./target/debug/hello_cargo`
*在windows环境下则是target\debug\hello_cargo.exe*

到了这里,如果一切顺利,那么终端应该会打印Hello,world了
首次运行Cargo会创建一个新文件:*Cargo.lock*
这个文件记录着项目以来的实际版本,*但文件本身没有依赖*,Cargo会帮我们管理好他的

*Cargo Run*
`$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!`
以上是文档中的一段示例,Cargo Run同样可以帮我们一次性完成代码编译和运行的操作

但需要注意的是,这一次并没有表现出Cargo正在编译hello_cargo的输出
**因为Cargo发现文件并没有被改变**,就直接运行了二进制文件,这在大多数语言中同样适用







