# 操作系统
> Linux kernel synchronization

Why Linux synchronization

- A modern OS kernel is one of the most complicated parallel programs you can study

- Includes most common synchronization patterns

Slightly more recently

- Optimize kernel performance by blocking inside the kernel
    - Big Kernel Lock:锁定整个Linux
    - 整个核心仅有一个BKL
    - BKL 已经移出 2.6.39

- A slippery slope
    - We can enable interrupts during system calls 
        - More complexity, lower latency
    - We can block in more places that make sense
        - Better CPU usage, more complexity

- Performance Scalability
    - 核数变多 需要同步机制 否则性能不能有效地提高
    - 操作系统提供保护不够 以下为操作系统提供的lock
        - coarse-grained 粗颗粒
        - fine-grained 细颗粒

- Atomic instructions
> 不可切割性 执行时不会交错 lock-free struct

Linux lock types
    - blocking: mutex, semaphore
    - Non-blocking: spinlocks, seqlocks, completions

