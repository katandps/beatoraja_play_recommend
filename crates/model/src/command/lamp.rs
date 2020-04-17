use crate::*;

pub(super) fn lamp<T>(
    songs: &Songs,
    table: &T,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult
where
    T: TableTrait,
{
    let mut vec = Vec::new();
    for level in levels.iter() {
        let song_vec = table.level_specified(level).get_song(songs);
        let mut summary = Summary::new();
        for song in song_vec {
            summary.push(
                score_log
                    .get_snap(&song.song_id(), &updated_at)
                    .clear_type(),
            )
        }
        vec.push(CountByLevel::make(summary))
    }
    CommandResult::LampGraph(Graph::make(table.name(), vec))
}
