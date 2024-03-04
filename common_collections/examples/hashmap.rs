use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT teams_map2
    // use collect here, don't implement it like above
    let teams_map2...

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}
