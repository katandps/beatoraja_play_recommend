use crate::*;

pub(super) fn lamp<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score: &Scores,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::LampGraph(
        table
            .make_detail(songs, score, updated_at)
            .make_lamp_graph(),
    )
}
