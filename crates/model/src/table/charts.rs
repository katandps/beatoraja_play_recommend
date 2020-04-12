use super::*;

#[derive(Serialize, Deserialize)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

impl Charts {
    pub fn new(charts: Vec<Chart>) -> Charts {
        Charts { charts }
    }
    pub fn level_specified(&self, level: &Level) -> Charts {
        let charts = self
            .charts
            .iter()
            .filter_map(|c| if &c.level == level { Some(c) } else { None })
            .cloned()
            .collect();
        Charts::new(charts)
    }

    pub fn get_levels(&self) -> Vec<Level> {
        let mut set = HashSet::new();
        for level in self.charts.iter().map(|c| c.level.clone()) {
            set.insert(level);
        }
        let mut vec: Vec<Level> = set.iter().cloned().collect();
        vec.sort_unstable();
        vec
    }

    pub fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable {
        ScoredTable::new(
            self.charts
                .iter()
                .flat_map(|chart| match song_data.song_id(&chart.md5) {
                    Some(song_id) => scores.merge(song_id, chart),
                    _ => None,
                })
                .collect(),
        )
    }

    pub fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts
            .iter()
            .flat_map(|c| match song_data.song(&c.md5) {
                Some(s) => Some(s),
                _ => None,
            })
            .collect()
    }
}

impl fmt::Display for Charts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec: Vec<String> = self.charts.iter().map(|c| c.string()).collect();
        write!(f, "{}", vec.join("\n"))
    }
}
