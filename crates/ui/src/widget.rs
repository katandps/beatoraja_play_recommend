use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

static CSS_EXT: &'static str = include_str!("../res/grid.css");

pub fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(CSS_EXT)
        .build()
}

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let element = vec![
            "noplay",
            "failed",
            "a-easy",
            "la-easy",
            "easy",
            "clear",
            "hard",
            "exhard",
            "fullcombo",
            "perfect",
            "maxclear",
        ];
        let text = vec![
            "NoPlay",
            "Failed",
            "A-Easy",
            "LA-Easy",
            "Easy",
            "Clear",
            "Hard",
            "ExHard",
            "FullCombo",
            "Perfect",
            "Max",
        ];

        let mut grid = Grid::create().columns(columns(text.len())).rows(rows(3));
        for i in 0..(text.len()) {
            grid = grid
                .child(lamp_grid(i, 1, element[i].to_string(), text[i].to_string(), ctx).build(ctx))
        }

        self.name("MainView").child(grid.build(ctx))
    }
}

fn columns(len: usize) -> Columns {
    let mut columns = Columns::create();
    for _i in 0..len {
        columns = columns.column("*");
    }
    columns.build()
}

fn rows(len: usize) -> Rows {
    let mut rows = Rows::create();
    for _i in 0..len {
        rows = rows.row("*");
    }
    rows.build()
}

fn lamp_grid(
    column: usize,
    row: usize,
    element: String,
    text: String,
    ctx: &mut BuildContext,
) -> Grid {
    Grid::create()
        .element(&element)
        .attach(Grid::column(column))
        .attach(Grid::row(row))
        .child(
            TextBlock::create()
                .element(&element)
                .text(text)
                .horizontal_alignment("center")
                .vertical_alignment("center")
                .build(ctx),
        )
}
