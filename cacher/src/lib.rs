use std::marker::PhantomData;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct Cacher<T, I, O>
where
    T: Fn(I) -> O,
    O: Clone,
    I: Hash,
    I: Eq
{
    compute: T,
    values: HashMap<I, O>,
    _marker: PhantomData<I>,
}

impl<T, I, O> Cacher<T, I, O>
where
    T: Fn(I) -> O,
    O: Clone,
    I: Hash,
    I: Eq,
    I: Clone
{
    fn new(compute: T) -> Cacher<T, I, O> {
        Cacher {
            compute,
            values: HashMap::new(),
            _marker: PhantomData,
        }
    }

    fn get(&mut self, arg: I) -> O {
        self.values.entry(arg.clone())
            .or_insert_with(|| (self.compute)(arg))
            .clone()
        // self.values.entry(arg)
        //     .or_insert((self.compute)(arg)).clone()
        // match self.values.get(&arg) {
        //     Some(v) => v.clone(),
        //     None => {
        //         let v = (self.compute)(arg);
        //         self.values.insert(arg, v);
        //         v.clone()
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
