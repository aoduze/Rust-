#猜数字游戏

>命令行创建
> Cargo new guessing_game
> cd guessing_game

*突然一下子看到新的结构可能会有点慌乱,让我们一句句来分析*

# `use std::io;`
在这一行代码中,io库来自于我们Rust的标准库,Rust标准库同样也被称为std

## println!(Guess the number),println!("Please input your guess"")
这两行的功能想必也能猜到,类比于CPP中的std::cout<<,将我们输入的字符串打印到屏幕上

### let mut guess = String::new();

***现在事情开始变得麻烦起来了***

1.let语句
let语句是我们用来创建变量的重要方式,我们来看看另一个例子: `let apples = 5;`
这一行代码新建了一个叫做apples的变量并把它绑定到值5上,请注意:**把变量绑定到值上**

在Rust中,*变量默认是不可变的*,我们会在第三章看到相关概念，在这里,我们只要知道如果
你想让变量可变,可以在变量名前添加***mut***(mutability,可变性)

这简直跟CPP以及绝大多数语言完全相反,学习过CPP的大家都知道CPP中有一个名叫*const*的东西,
当我们在变量名前添加*const*之后,这个值就是固定的,正常情况下我们就无法改变它的值.
*示例*:

`CPP`
    `const int const_value = 5;`
    `int change_value = 5`

    const_value += change_value,错误操作
    change_value += const_value，可以这样操作

`Rust`
    `let apples = 5` //不可变
    `let mut bananas = 5` //可变
>在Rust中,注释同样是使用//

好,理解之后我们将注意力放到等号右边:`String::new()`
String显然是一个字符串类型,我们格外关注一下*new()*:
::语法表明new是String类型中的一个关联函数,什么是关联函数？
*简单概括,我们将它理解成开辟了一个新的空间用来存放我们的String对象即可*

这个new函数创建了一个新的空字符串,所以这一行的意思是创建了一个可变变量,把他绑定到一个
String空实例上,单说可能有点难以理解,我们换个方式讲解

`CPP`
`std::string guess;` *完事了,就是这么简单的意思*

#### io::stdin().read_line(&mut guess).expect("Failed to read line");

1. `io::stdin()`负责调用我们io库中的方法,其中stdin返回一个`std::io::Stdin`的实例
这是一个类型,代表终端标准输入的*句柄*

2. 随后,`.read_lin(&mut guess)`立即就调用了read_line方法,这个方法有点类似于CPP中的
`std::cin.getline()`,大致意思就是从输入句柄中获取用户的输入,并存入我们的可变参数guess
中。

这里有一点要注意一下,那就是引用符号`&`，引用符号代表着我们即将忽略多次拷贝操作,而是直接访问
这一处数据,这样避免了多次拷贝的性能和内存消耗。 同样的,引用默认是不可变的,这同样保证了内存安全

*练习:&mut guess 与 &guess 的区别？*
>答案：
> 如果不加入mut,Rust会认为该值不可更改,既然不可更改,我们又要怎样去输入它的值？

3. `.expect("Failed to read line");`
之前我们提及到read_line将用户输入的数据存储到guess这个字符串中,这个操作实际上也会产生一个
值,我们暂且把这个值称为Result:
*Result值一般是枚举类型,他只有两个成员:`OK` 或者 `Err`*

Result的返回值一般代表着操作的成功与否,操作成功则是*OK*,反之*Err*;
实际上就算你不调用expect,程序照常运行,只不过会警告

***Rust中的占位符打印规则***
`println!("the number is {}",guess);`
Rust使用闭合花括号来代表特定位置的占位符




