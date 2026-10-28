# lab1实验报告
## 编程作业功能实现
    trace的前两个实现很简单，对id进行强制类型转换，获取或写入数据data即可。
    最主要的改动在于新增了/os/src/task/counter.rs模块，它维护一个静态的全局任务的系统调用计数表。我为其实现了两个函数接口：`add_syscall_count`和`fetch_syscall_count`，传入当前任务id和系统调用号，就可以**增加/获取当前任务的一个系统调用计数**。他们分别在os/src/syscall/process.rs和os/src/syscall/mod.rs被使用，完成了trace的功能。
## 简答题
一.
    RustSBI version 0.4.1
    ch2b_bad_address.rs的报错是程序访问了一个错误地址0x0，触发缺页异常，内核kill了它。
    ch2b_bad_instructions.rs的错误是用户程序想执行特权指令`sret`，触发 Illegal Instruction 异常，内核kill了它。
    ch2b_bad_register.rs的错误是用户程序想访问CSR寄存器`sstatus`，触发 Illegal Instruction 异常，内核kill了它。
二.
    1.sp指向内核栈栈顶；__restore会在trap返回时，在第一次进入用户态被使用。
    2. CSR: 
            sstatus 记录Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
            sepc 记录 Trap 发生之前执行的最后一条指令的地址
            sscratch 暂存寄存器，U 态进入 S 态时，__alltraps 的 csrrw sp, sscratch, sp 会用它换栈
    3.  直接 `sd x2, 2*8(sp)` 存的是内核栈指针，是错的；所以要 `t2 ← sscratch` 绕一步
        U 态用不到x4
    4.  换栈：sp←用户栈顶，sscratch←内核栈顶
    5.  sret，会切换回用户态
    6.  换栈：sp→内核栈顶，sscratch→用户栈顶
    7.  ecall

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    deepseek

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。