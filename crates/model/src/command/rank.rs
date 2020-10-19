use crate::*;

pub(super) fn rank<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score: &Scores,
    _score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::RankGraph(
        table
            .make_detail(songs, score, updated_at)
            .make_rank_graph(),
    )
}
