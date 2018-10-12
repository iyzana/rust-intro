use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Cacher<T, I, O>
where
    T: Fn(I) -> O,
    O: Clone,
    I: Hash + Eq + Clone,
{
    compute: T,
    values: HashMap<I, O>,
    _marker: PhantomData<I>,
}

impl<T, I, O> Cacher<T, I, O>
where
    T: Fn(I) -> O,
    O: Clone,
    I: Hash + Eq + Clone,
{
    fn new(compute: T) -> Cacher<T, I, O> {
        Cacher {
            compute,
            values: HashMap::new(),
            _marker: PhantomData,
        }
    }

    fn get(&mut self, arg: I) -> O {
        if let Some(v) = self.values.get(&arg) {
            return v.clone();
        };

        let v = (self.compute)(arg.clone());
        self.values.insert(arg, v.clone());
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use std::thread;

    #[test]
    fn it_works() {
        let mut cacher = Cacher::new(|_i| SystemTime::now());

        let time1 = cacher.get(1);
        thread::sleep_ms(2);
        let time2 = cacher.get(1);
        assert_eq!(time1, time2);
    }
}
