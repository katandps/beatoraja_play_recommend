use crate::*;

pub(super) fn rank<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score: &Scores,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::RankGraph(
        table
            .make_detail(songs, score, updated_at)
            .make_rank_graph(),
    )
}
