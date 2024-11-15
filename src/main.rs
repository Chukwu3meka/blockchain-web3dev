use std::collections::BTreeMap;

mod balances;

fn main() {
    println!("Hello, world!");

    // let mut map = BTreeMap::new();
    // map.insert("alice", 100);

    // // assert_eq!(map.get(&"alice"), Some(&100));
    // assert_eq!(map.get(&"alice").unwrap_or(&0), (&100)); // will set value as zero when no value is found
    // assert_eq!(map.get(&"bob"), None);

    // let maybe_value: Option<&i32> = map.get("alice");

    // match maybe_value {
    //     Some(value) => {
    //         println!("{}", value)
    //     }
    //     None => println!("No value"),
    // }
}

#[test]
fn init_balances() {
    let mut balances = balances::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);

    balances.set_balance(&"alice".to_string(), 100);

    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);
}

#[test]
fn fail_test() {
    assert_eq!(2, 2)
}
