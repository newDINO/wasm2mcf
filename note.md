# 第一步，调查：

## 问题
- 如何实现循环？用递归？确实可以用递归。经过测试mcfunction可以运行递归
- 如何concat string? 用function macro
- 能否将riscv转化为mcfunction？不能，因为riscv支持跳转到寄存器中储存的地址
- NBT str能否使用index在某位置获取或储存一个字符？不能
- 之前deepseek幻想的$(a0 + a1)有用吗？没用


## scoreboard支持的操作
`+=, -=, *=, /=, %=, <, >, =, ><`

其中`<`是让target <= source，即
```rs
if target > source { target = source }
```

`>`是让target >= source，即
```rs
if target < source { target = source }
```

`><`是交换target和source，即
```rs
std::mem::swap(&mut target, &mut source)
```

scoreboard只接受i32

## 模拟其他
### u32
#### mul add sub
与i32相同
#### div
这个有待测试
```rs
fn simulate_u32_div_using_i32_div(v: u32, d: u32) -> u32 {
    let vi = v.cast_signed();
    let di = d.cast_signed();
    let ri = if vi >= 0 {
        if di < 0 { 0 } else { vi / di }
    } else {
        if di < 0 {
            1
        } else {
            let a0 = i32::MAX;
            let q0 = a0 / di;
            let r0 = a0 % di;

            let a1 = 2 + r0 * 2 + vi;
            let q1 = a1 / di;
            let r1 = a1 % di;

            let q1 = if r1 < 0 { q1 - 1 } else { q1 };

            q0 * 2 + q1
        }
    };

    ri.cast_unsigned()
}
```
#### shr
方法1:
用下面的方法模拟除以2的倍数
```rs
fn simulate_u32_div_even_using_i32_div_even(v: u32, d: u32) -> u32 {
    let vi = v.cast_signed();
    let di = d.cast_signed();
    if vi >= 0 {
        return (vi / di).cast_unsigned();
    }
    let i2 = vi + 2;
    let a = i32::MAX / (di / 2) + i2 / di;
    let ri = if i2 % di < 0 { a - 1 } else { a };
    ri.cast_unsigned()
}
```
#### shl
用i32乘法实现
`<< 1` 变为 `* 2`  
`<< 2` 变为 `* 4`  
`<< n` 变为 `* 2^n`  
对于常数可以直接生成相应的，对于不是常数可能需要循环


## data指令支持的操作
- data modify的append在list最后添加一个
- data modify的insert也可以在最后添加一个，但在这还后面就不能了
- data中的list可以存储任意类型，类似js Array
- data remove可以用于移除list中的某个
- nbt中可以有更底层的data type如byte, short, byte array，详见[wiki](https://minecraft.wiki/w/NBT_format#Data_types)
- 可以byte array可以append和remove

## 算符替换
- i32.shr_u 替换为 u32 / 2，但是如何实现 u32 / 2呢？mc只支持i32 / 2

## 单点
- scoreboard可以直接添加不存在的玩家或实体，即任意变量名
- Invulnerable可以让实体不受大部分伤害，但创造模式玩家仍然可以将其杀死
- Marker应该是是最适合的作为双精度浮点加法器的
- exec.mcfunction
- 未经压缩的command_storage.dat也能被直接打开，并且随意设置DataVersion没关系

## 其他类似
- clang-mc
- Sandstone (based on typescript)

# 第二步，方案：
## 1
### 直接用list作为栈

`i32.const 42` 变为：`function wasmcore:stack_push {a0: 42}`

`i32.add` 变为：`function wasmcore:add`

目前是使用data get获取栈的大小，事实上栈的大小是在编译器可知的，吗？并不可知，如果可以随意调用其他函数的话

### low level 与 high level mcfunction
存在两种mcfunction：

一种是low level，它们可以使用args向其传递参数，并直接使用args中的特定名称的field作为参数，
因此当low level mcf内部可能调用自身时，必须要非常小心确保不会出bug

另一种是high level，它们使用locals进行参数传递，如wasm直接编译而来的mcfunction

除了low level mcf 除了object和string尽量使用mc原有返回机制
简单的和模拟循环的递归尽量不要使用locals避免栈变得过大

但是新发现的可以用-1从后往前索引或将改变一切，现在从locals获取参数变得非常容易，可能不会出现用fixed_args传递参数的情况了

## 2
### 直接将scoreboard作为栈
locals也在scoreboard中直接储存

原理是：每个函数的栈大小在编译器已知

问题1：由于可以无限调用函数，实际的栈的大小并不知道
方案1：每层调用都创建一个新的scoreboard
方案2：调用函数是把上次调用栈存储在storage里面

问题：scoreboard无法储存除了i32以外的数据，不过可以用两个位置来储存


# 第三步，实现：
## 目标
### Hello, world!
### 需要
- ascii array to nbt string

### 编写一个绘制球的function可以在控制台中调用
#### 分步
- 先可以在固定位置实现
- 然后可以传递参数
#### 需要
- for循环
- 调用其他指令
- 整数乘法、加法、比较
- 条件分支