trait MapperCache<T, U, F>
where
    F: Fn(T) -> U,
{
    fn new(mapper: F) -> Self;
    fn call(&mut self, val: T) -> U;
}

struct Cache<T, U>
where
    T: Fn(U) -> U,
{
    calc: T,
    result: Option<U>,
}

impl<T, U> MapperCache<U, U, T> for Cache<T, U>
where
    U: Clone,
    T: Fn(U) -> U,
{
    fn new(mapper: T) -> Self {
        Self {
            calc: mapper,
            result: None,
        }
    }

    fn call(&mut self, val: U) -> U {
        if self.result.is_none() {
            let res = (self.calc)(val);
            self.result = Some(res);
        }
        self.result.clone().unwrap()
    }
}

fn main() {
    let mut square_func = Cache::new(|x| x * x);
    let res = square_func.call(3.0);
    println!("3 ^ 3 = {}", res);
}
