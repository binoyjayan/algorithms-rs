use hashmap::HashMap;

fn main() {
    let mut map = HashMap::new();
    assert_eq!(map.entry("foo").or_insert(42), &42);

    let mut player_stats = HashMap::new();
    player_stats.entry("health").or_insert(100);
    player_stats.entry("defence").or_insert_with(|| 42);
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += 10;
    assert_eq!(player_stats["attack"], 110);
}
