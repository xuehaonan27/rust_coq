use std::{cell::RefCell, rc::Rc};

pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

#[macro_export]
macro_rules! church {
    ($x: expr) => {
        from_usize($x)
    };
}

pub fn zero<T: 'static>() -> Church<T> {
    // This is a function(closure) that takes a function `_f` and returns
    // another function `move |x| x`. Obviously this closure's type satisfies
    // `Fn` trait because it doesn't have any side effect. And the whole
    // closure's type satisfies `Fn` trait as well.
    //
    // `zero` is to apply a function `f` to `x` zero time.
    Rc::new(move |_f| Rc::new(move |x| x))
}

// `one` is to apply a function `f` to `x` once.
pub fn one<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(x)))
}

// `two` is to apply a function `f` to `x` twice.
pub fn two<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(x))))
}

// `three` is to apply a function `f` to `x` three times.
pub fn three<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(f(x)))))
}

pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let f_n = n(Rc::clone(&f));
        Rc::new(move |x| f(f_n(x)))
    })
}

pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
    let mut result = zero();
    for _ in 0..n {
        result = succ(result);
    }
    result
}

// Same reason, we cannot write `impl<T> Into<usize> for Church<T>`.
pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
    let count = Rc::new(RefCell::new(0));
    let c = Rc::clone(&count);

    // We could utilize the function `f` itself to calculate how much layers
    // we have gone through.
    //
    // i.e. we could have written `f` like this:
    /*
    let f: Rc<dyn Fn(T)->T> = Rc::new (
        move|x| x
    );
    */
    // But we choose to write like this to utilize `f` to convert the Church
    // number (layers of functions) to a `usize` number.
    let f: Rc<dyn Fn(T) -> T> = Rc::new(move |x| {
        let mut count_mut = c.borrow_mut();
        *count_mut += 1;
        x
    });

    // Apply the function `n` times.
    let result_f = n(f);

    // Just pass a default value of `T` to `result_f`.
    // After all, we care neither about the functionality of `f` nor
    // the final value of that `T` typed value.
    // We just want the "size effect" of `f` to count how much layers
    // we have expanded.
    let _ = result_f(Default::default());

    let result = *count.borrow();
    result
}

// `add` is to add two Church numbers `n` and `m`.
// i.e. call `f` on `x` n times, and then another `m` times.
pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        // Apply `f` n times first.
        let f_n = n(Rc::clone(&f));
        // And then apply `f` m times.
        let f_m = m(Rc::clone(&f));
        Rc::new(move |x| f_m(f_n(x)))
    })
}

// `mult`. Applying "calling `f` on `x` n times" m times.
pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        // Apply `f` n times first.
        let f_n = n(Rc::clone(&f));
        // And then apply "applying `f` n times" m times.
        let f_m_n = m(Rc::clone(&f_n));
        Rc::new(move |x| f_m_n(x))
    })
}

// `exp`. Most difficult one.
// Well, try to get some inspiration from the type annotaion of `m`.
pub fn exp<T: 'static>(n: Church<T>, m: Church<Rc<dyn Fn(T) -> T>>) -> Church<T> {
    Rc::new(move |f| {
        let n: Church<T> = n.clone();
        let f_n_to_m = m(n)(f.clone());
        Rc::new(move |x| f_n_to_m(x))
    })
}

#[cfg(test)]
mod test_church {
    use super::*;
    type T = ();

    fn id(n: usize) -> usize {
        to_usize(from_usize::<()>(n))
    }

    #[test]
    pub fn check_identity() {
        assert_eq!(5, id(5));
    }

    #[test]
    pub fn check_zero() {
        assert_eq!(0, to_usize(zero::<T>()));
    }

    #[test]
    pub fn check_one() {
        assert_eq!(1, to_usize(one::<T>()));
    }

    #[test]
    pub fn check_two() {
        assert_eq!(2, to_usize(two::<T>()));
    }

    #[test]
    pub fn check_three() {
        assert_eq!(3, to_usize(three::<T>()))
    }

    #[test]
    pub fn zero_succ_is_one() {
        assert_eq!(to_usize(one::<T>()), to_usize(succ(zero::<T>())))
    }

    #[test]
    pub fn one_succ_is_two() {
        assert_eq!(to_usize(two::<T>()), to_usize(succ(one::<T>())))
    }

    #[test]
    pub fn two_succ_is_three() {
        assert_eq!(to_usize(three::<T>()), to_usize(succ(two::<T>())))
    }

    #[test]
    pub fn plus_one_is_succ_once() {
        assert_eq!(
            to_usize(add(two::<T>(), one::<T>())),
            to_usize(succ(two::<T>()))
        )
    }

    #[test]
    pub fn plus_two_is_succ_twice() {
        assert_eq!(
            to_usize(add(one::<T>(), two::<T>())),
            to_usize(succ(succ(one::<T>())))
        )
    }

    #[test]
    pub fn two_plus_two_is_two_times_two() {
        assert_eq!(
            to_usize(add(two::<T>(), two::<T>())),
            to_usize(mult(two::<T>(), two::<T>()))
        )
    }

    #[test]
    pub fn church_add_1_2() {
        let church_three: Church<T> = add(one::<T>(), two::<T>());
        assert_eq!(to_usize(church_three), 3)
    }

    #[test]
    pub fn church_mult_2_3() {
        let church_six: Church<T> = from_usize(6);
        assert_eq!(
            to_usize(church_six),
            to_usize(mult(two::<T>(), three::<T>()))
        );
    }

    #[test]
    pub fn church_exp_2_3() {
        let church_eight: Church<T> = from_usize(8);
        assert_eq!(to_usize(church_eight), to_usize(exp(two::<T>(), three())));
    }

    #[test]
    pub fn church_exp_3_5() {
        let church_3: Church<T> = from_usize(3);
        let church_5: Church<Rc<dyn Fn(T) -> T>> = from_usize(5);
        let church_243: Church<T> = from_usize(243);
        assert_eq!(to_usize(church_243), to_usize(exp(church_3, church_5)))
    }
}