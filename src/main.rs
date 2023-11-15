mod SimpleLinkedList;
mod TestObject;
mod AdvancedLinkedList;
mod TestSuite1;
mod Timer;
mod TestSuite2;
mod TestSuite3;
mod TestSuite4;

use std::mem;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    TestSuite1::test1();
    TestSuite1::test2();
    TestSuite1::test3();
    TestSuite1::test4();
    TestSuite1::test5();
    TestSuite1::test6();
    TestSuite1::test7();

    TestSuite2::test1();
    TestSuite2::test2();
    TestSuite2::test3();
    TestSuite2::test4();
    TestSuite2::test5();
    TestSuite2::test6();
    TestSuite2::test7();

    TestSuite3::test1();
    TestSuite3::test2();
    TestSuite3::test3();
    TestSuite3::test4();
    TestSuite3::test5();
    TestSuite3::test6();
    TestSuite3::test7();
    TestSuite3::test8();
    TestSuite3::test9();

    TestSuite4::test1();
    TestSuite4::test2();
    TestSuite4::test3();
    TestSuite4::test4();
    TestSuite4::test5();
    TestSuite4::test6();
    TestSuite4::test7();
    TestSuite4::test8();
}
