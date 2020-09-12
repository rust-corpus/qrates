#![allow(dead_code)]

static mut X: u32 = 4;

unsafe fn test1() {
    X = 5;
}

fn test2() {
    unsafe {
        test1();
    }
}

struct S;

impl S {
    unsafe fn test7() {
        test1();
    }
    fn test8() {
        unsafe {
            test1();
        }
    }
}

trait T {
    unsafe fn test3() {
        test1();
    }
    unsafe fn test4();
    fn test5() {
        unsafe {
            test1();
        }
    }
    fn test6();
}

impl T for S {
    unsafe fn test3() {
        test1();
    }
    unsafe fn test4() {
        test1();
    }
    fn test5() {
        unsafe {
            test1();
        }
    }
    fn test6() {
        unsafe {
            test1();
        }
    }
}

mod module1 {

    mod module2 {

        fn test2() {
            unsafe {
                crate::test1();
            }
        }

        struct S;

        impl S {
            unsafe fn test7() {
                crate::test1();
            }
            fn test8() {
                unsafe {
                    crate::test1();
                }
            }
        }

        trait T {
            unsafe fn test3() {
                crate::test1();
            }
            unsafe fn test4();
            fn test5() {
                unsafe {
                    crate::test1();
                }
            }
            fn test6();
        }


        impl T for S {
            unsafe fn test3() {
                crate::test1();
            }
            unsafe fn test4() {
                crate::test1();
            }
            fn test5() {
                unsafe {
                    crate::test1();
                }
            }
            fn test6() {
                unsafe {
                    crate::test1();
                }
            }
        }

    }

}
