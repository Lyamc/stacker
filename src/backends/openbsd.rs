pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    let mut stackinfo = std::mem::MaybeUninit::<crate::posix::StackT>::uninit();
    let res = crate::posix::pthread_stackseg_np(crate::posix::pthread_self(), stackinfo.as_mut_ptr());
    if res != 0 {
        return None;
    }
    let stackinfo = stackinfo.assume_init();
    Some(stackinfo.ss_sp as usize - stackinfo.ss_size)
}
