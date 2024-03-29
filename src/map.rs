use std::rc::Rc;

/// A total map is as function that returns a default value when
/// looked up.
pub type TotalMap<K, V> = Rc<dyn Fn(K) -> V>;

/// Function tm_empty yields an empty total map given a default
/// element. This map always returns the default element when applied
/// to any key.
pub fn tm_empty<K: 'static + PartialEq, V: 'static + Clone>(default_v: V) -> TotalMap<K, V> {
    Rc::new(move |_| default_v.clone())
}

pub fn tm_update<K: 'static + PartialEq, V: 'static + Clone>(
    m: TotalMap<K, V>,
    k: K,
    v: V,
) -> TotalMap<K, V> {
    Rc::new(move |k_1| if k_1 == k { v.clone() } else { m(k_1) })
}

#[macro_export]
macro_rules! total_map {
    ($default_v: expr, $({$k: expr, $v: expr}), *) => {
        {
            let mut map = tm_empty ($default_v);
            $(
                map = tm_update(map, $k, $v);
            )*
            map
        }
    };
}

#[cfg(test)]
mod test_total_map {
    use super::{tm_empty, tm_update, TotalMap};

    /// Fetch a map:
    /// "bar" !-> true;
    /// "foo" !-> true;
    /// _     !-> false;
    fn example_map() -> TotalMap<String, bool> {
        tm_update(
            tm_update(tm_empty(false), "foo".to_string(), true),
            "bar".to_string(),
            true,
        )
    }

    #[allow(unused)]
    fn macro_example() -> TotalMap<String, bool> {
        total_map!(false, {"foo".to_string(), true}, {"bar".to_string(), true})
    }

    #[test]
    fn test_example_map_foo() {
        assert_eq!(example_map()("foo".to_string()), true)
    }

    #[test]
    fn test_example_map_bar() {
        assert_eq!(example_map()("bar".to_string()), true)
    }

    #[test]
    fn test_example_map_other() {
        assert_eq!(example_map()("box".to_string()), false)
    }
}

pub type PartialMap<K, V> = TotalMap<K, Option<V>>;

pub fn pm_empty<K: 'static + PartialEq, V: 'static + Clone>() -> PartialMap<K, V> {
    tm_empty(None)
}

pub fn pm_update<K: 'static + PartialEq, V: 'static + Clone>(
    m: PartialMap<K, V>,
    k: K,
    v: V,
) -> PartialMap<K, V> {
    tm_update(m, k, Some(v))
}

#[macro_export]
macro_rules! partial_map {
    ($({$k: expr, $v: expr}), *) => {
        {
            let mut map = pm_empty();
            $(
                map = pm_update(map, $k, $v);
            )*
            map
        }
    };
}

#[cfg(test)]
mod test_partial_map {
    use super::{pm_empty, pm_update, PartialMap};

    fn example_map() -> PartialMap<String, bool> {
        pm_update(
            pm_update(pm_empty(), "Church".to_string(), true),
            "Turing".to_string(),
            false,
        )
    }

    #[allow(unused)]
    fn macro_example() -> PartialMap<String, bool> {
        partial_map!({"Church".to_string(), true}, {"Turing".to_string(), false})
    }

    #[test]
    fn test_example_map_church() {
        assert_eq!(example_map()("Church".to_string()), Some(true));
    }

    #[test]
    fn test_example_map_turing() {
        assert_eq!(example_map()("Turing".to_string()), Some(false));
    }

    #[test]
    fn test_example_map_other() {
        assert_eq!(example_map()("Other".to_string()), None)
    }
}
