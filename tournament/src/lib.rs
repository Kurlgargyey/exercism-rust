use std::{ cmp::Ordering, collections::{ BTreeSet, HashMap } };

pub fn tally(match_results: &str) -> String {
    let mut tally = Tally::new();

    let matches: Vec<_> = match_results.split("\n").collect();

    for result in matches {
        tally.push(result);
    }

    tally.to_string()
}

#[derive(Debug)]
struct Tally<'a>(HashMap<&'a str, Team<'a>>);

impl<'a> Tally<'a> {
    fn new() -> Self {
        Tally(HashMap::<&'a str, Team<'a>>::new())
    }
    fn push(&mut self, result: &'a str) {
        let mut details = result.split(';');

        if
            let (Some(home_team), Some(away_team), Some(outcome)) = (
                details.next(),
                details.next(),
                details.next(),
            )
        {
            match outcome {
                "win" => self.mark_win(home_team, away_team),
                "draw" => self.mark_draw(home_team, away_team),
                "loss" => self.mark_win(away_team, home_team),
                _ => (),
            }
        }
    }
    fn mark_win(&mut self, home_team: &'a str, away_team: &'a str) {
        let home_team = self.0.entry(home_team).or_insert(Team::new(home_team));
        home_team.mark_outcome(Outcome::Win);
        let away_team = self.0.entry(away_team).or_insert(Team::new(away_team));
        away_team.mark_outcome(Outcome::Loss)
    }
    fn mark_draw(&mut self, home_team: &'a str, away_team: &'a str) {
        let home_team = self.0.entry(home_team).or_insert(Team::new(home_team));
        home_team.mark_outcome(Outcome::Draw);
        let away_team = self.0.entry(away_team).or_insert(Team::new(away_team));
        away_team.mark_outcome(Outcome::Draw)
    }
}

impl ToString for Tally<'_> {
    fn to_string(&self) -> String {
        let mut ordered_teams = BTreeSet::<&Team>::new();

        for team in self.0.values() {
            println!("{:?}",ordered_teams.insert(team));
        }

        let team_stats = ordered_teams
            .into_iter()
            .map(|team| { team.to_string() })
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

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Team<'a> {
    name: &'a str,
    outcomes: Vec<Outcome>,
}

impl<'a> Team<'a> {
    fn new(name: &'a str) -> Self {
        Team { name, outcomes: Vec::<Outcome>::new() }
    }

    fn matches(&self) -> u16 {
        self.outcomes.len() as u16
    }
    fn wins(&self) -> u16 {
        self.outcomes
            .iter()
            .filter(|outcome| **outcome == Outcome::Win)
            .count() as u16
    }
    fn draws(&self) -> u16 {
        self.outcomes
            .iter()
            .filter(|outcome| **outcome == Outcome::Draw)
            .count() as u16
    }
    fn losses(&self) -> u16 {
        self.outcomes
            .iter()
            .filter(|outcome| **outcome == Outcome::Loss)
            .count() as u16
    }
    fn points(&self) -> u16 {
        self.outcomes
            .iter()
            .map(|outcome| *outcome as u16)
            .sum()
    }
    fn mark_outcome(&mut self, outcome: Outcome) {
        self.outcomes.push(outcome)
    }
}

impl Ord for Team<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let point_order = self.points().cmp(&other.points()).reverse();
        if point_order == Ordering::Equal {
            return self.name.cmp(other.name);
        }
        point_order
    }
}

impl ToString for Team<'_> {
    fn to_string(&self) -> String {
        create_row(
            self.name,
            &self.matches().to_string(),
            &self.wins().to_string(),
            &self.draws().to_string(),
            &self.losses().to_string(),
            &self.points().to_string()
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Outcome {
    Win = 3,
    Loss = 0,
    Draw = 1,
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
