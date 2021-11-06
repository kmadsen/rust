use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    value_map: HashMap<K, V>
}

impl<T, K : Eq + Hash + Copy, V : Copy> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    {
        fn new(calculation: T) -> Cacher<T, K, V> {
            Cacher {
                calculation,
                value_map: HashMap::new()
            }
        }

        fn value(&mut self, key: K) -> V {
            match self.value_map.entry(key) {
                Entry::Occupied(v) => *v.get(),
                Entry::Vacant(o) => {
                    let v = (self.calculation)(*o.key());
                    *o.insert(v)
                }
            }   
        }
    }

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity))
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_string_values() {
    let mut c = Cacher::new(|a: &str| a.len());

    let v1 = c.value("string literal");
    let v2 = c.value("another string literal");
    let v3 = c.value("string literal");

    assert_eq!(v2, 22);
    assert_eq!(v1, v3);
}

#[test]
fn call_with_string_slice() {
    let mut c = Cacher::new(|s: &String| -> usize {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
    
        s.len()
    });

    let v1 = c.value(&std::string::String::from("what"));

    assert_eq!(v1, 4);
}
