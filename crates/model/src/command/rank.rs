use crate::*;

pub(super) fn rank<T: TableTrait>(
    songs: &Songs,
    table: &T,
    _score: &Scores,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::RankGraph(table.make_graph(songs, score_log, updated_at))
}
