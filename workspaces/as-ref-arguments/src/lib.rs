//! This crate has two purposes:
//! 1. Test how to implement trait for reference;
//! 2. test how `AsRef<T>` functions as generic argument.

#[derive(Clone)]
pub struct Demo {}

pub trait DemoTrait {
    fn whats_this(&self);
}

impl DemoTrait for Demo {
    fn whats_this(&self) {
        println!("This is Demo.");
    }
}

impl DemoTrait for &Demo {
    fn whats_this(&self) {
        println!("This is &demo");
    }
}

impl AsRef<Demo> for Demo {
    fn as_ref(&self) -> &Demo {
        return self;
    }
}

pub fn work(v: impl DemoTrait) {
    v.whats_this();
}

pub fn work_2(v: impl AsRef<Demo>) {
    v.as_ref().whats_this();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let demo = Demo {};
        demo.whats_this();
        // Auto de-reference; invoke `whats_this` on `Demo`.
        (&demo).whats_this();
        // Invoke `whats_this` on `&Demo`.
        (&&demo).whats_this();
        (&&&demo).whats_this();
        let m = &demo;
        m.whats_this();
        let n = m as &(dyn DemoTrait);
        n.whats_this();
        work(demo.clone());
        work(m);
        // Invoke `whats_this` on `Demo`.
        work_2(demo.clone());
        // Auto de-reference & invoke the `whats_this` function on `Demo`.
        work_2(m);
    }
}
