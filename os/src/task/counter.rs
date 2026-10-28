const MAX_SYSCALL: usize = 512;
const MAX_TASKS: usize = 16;

static mut SYSCALL_COUNTS: [[isize; MAX_SYSCALL]; MAX_TASKS] =
    [[0; MAX_SYSCALL]; MAX_TASKS];

///增加当前任务的系统调用计数  
pub fn add_syscall_count(task_id: usize, syscall_id: usize) {
    unsafe {
        //let val = SYSCALL_COUNTS[task_id][syscall_id];
        //println!("[fetch]The task: {} and syscall_id: {} is {}",task_id,syscall_id,val+1);
        SYSCALL_COUNTS[task_id][syscall_id] +=1;
    }
}

///获取当前任务的指定系统调用使用次数
pub fn fetch_syscall_count(task_id: usize, syscall_id: usize) -> isize {
    unsafe {
        let val = SYSCALL_COUNTS[task_id][syscall_id];
        //println!("[fetch]The task: {} and syscall_id: {} is {}",task_id,syscall_id,val);
        val
    }
}
