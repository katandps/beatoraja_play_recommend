use crate::*;
use std::borrow::Borrow;

pub(super) fn rank<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    CommandResult::RankGraph(Graph::make(
        table.name(),
        levels
            .iter()
            .map(|level| {
                let song_vec = table.level_specified(level).get_song(songs);
                CountByLevel::make(make_summary(song_vec, score_log, updated_at))
            })
            .collect(),
    ))
}

pub fn make_summary(
    songs: Vec<&Song>,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> Summary<ClearRank> {
    let mut summary = Summary::new();
    for song in songs {
        summary.push(
            &SongWithSnap::make(
                &song,
                score_log.get_snap(&song.song_id(), &updated_at).borrow(),
            )
            .clear_rank(),
        )
    }
    summary
}
