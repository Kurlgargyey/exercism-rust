use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd)]
struct Team<'a> {
    name: &'a str,
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

impl Team<'_> {}

impl Ord for Team<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points.cmp(&other.points)
    }
}

impl ToString for Team<'_> {
    fn to_string(&self) -> String {
        create_row(
            self.name,
            &self.matches.to_string(),
            &self.wins.to_string(),
            &self.draws.to_string(),
            &self.losses.to_string(),
            &self.points.to_string()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let header = create_row("Team", "MP", "W", "D", "L", "P");
    let mut row = Vec::<&str>::new();
    row.push(&header);
    let matches = match_results.split("\n");

    for result in matches {
        let mut results = result.split(';');
        let (home_team, away_team, outcome) = (results.nth(0), results.nth(1), results.nth(2));
    }

    let table = row.join("\n");
    table
}

fn create_row(
    team: &str,
    matches: &str,
    wins: &str,
    draws: &str,
    losses: &str,
    points: &str
) -> String {
    format!(
        "{0: <30} | {1: >2} | {2: >2} | {3: >2} | {4: >2} | {5: >2}",
        team,
        matches,
        wins,
        draws,
        losses,
        points
    )
}
