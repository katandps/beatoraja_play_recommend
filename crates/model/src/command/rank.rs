use crate::*;
use std::borrow::Borrow;

pub(super) fn rank<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    let mut vec = Vec::new();
    for level in levels.iter() {
        let song_vec = table.level_specified(level).get_song(songs);
        let mut summary = Summary::new();
        for song in song_vec {
            summary.push(
                &SongWithSnap::make(
                    &song,
                    score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                )
                .clear_rank(),
            )
        }
        vec.push(CountByLevel::make(summary));
    }
    CommandResult::RankGraph(Graph::make(table.name(), vec))
}
