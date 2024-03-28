use std::collections::{ HashMap, BTreeSet };

pub fn tally(match_results: &str) -> String {
    let mut tally = Tally::new();

    let matches = match_results.split("\n");

    for result in matches {
        tally.push(result);
    }

    tally.to_string()
}
struct Tally<'a>(HashMap<&'a str, Team<'a>>);

impl<'a> Tally<'a> {
    fn new() -> Self {
        Tally(HashMap::<&'a str, Team<'a>>::new())
    }
    fn push(&mut self, result: &str) {
        let mut details = result.split(';');
        let (home_team, away_team, outcome) = (
            details.nth(0).expect("incorrectly formatted row! (missing home team)"),
            details.nth(1).expect("incorrectly formatted row! (missing away team)"),
            details.nth(2).expect("incorrectly formatted row! (missing outcome)"),
        );
        match outcome {
            "win" => self.mark_win(home_team, away_team),
            "draw" => self.mark_draw(home_team, away_team),
            "loss" => self.mark_win(away_team, home_team),
            _ => (),
        }
    }
    fn mark_win(&mut self, home_team: &str, away_team: &str) {}
    fn mark_draw(&mut self, home_team: &str, away_team: &str) {}
}

impl ToString for Tally<'_> {
    fn to_string(&self) -> String {
        let mut ordered_teams = BTreeSet::<&Team>::new();

        for team in self.0.values() {
            ordered_teams.insert(team);
        }

        let team_stats = ordered_teams
            .into_iter()
            .map(|team| team.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut result = create_header();
        if !team_stats.is_empty() {
            result.push_str("\n");
            result.push_str(&team_stats);
        }
        result
    }
}

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

fn create_header() -> String {
    create_row("Team", "MP", "W", "D", "L", "P")
}
