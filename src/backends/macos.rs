pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    Some(
        crate::posix::pthread_get_stackaddr_np(crate::posix::pthread_self()) as usize
            - crate::posix::pthread_get_stacksize_np(crate::posix::pthread_self()) as usize,
    )
}
