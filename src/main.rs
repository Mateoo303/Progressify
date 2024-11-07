enum BarStyle {
    Classic,
    ConnectedLine,
    Line,
    DottedLine,
    Hash,
    Full,
    Rect,
    RectFull,
    HollowRect,

}

struct ProgressBar {
    title: String,
    percentage: f32,
    segments: i32,
    eta: i32,
    style: BarStyle,
}

impl ProgressBar {
    fn new(
        title: String,
        percentage: f32,
        segments: i32,
        eta: i32,
        style: BarStyle,
    ) -> ProgressBar {
        ProgressBar {
            title,
            percentage,
            segments,
            eta,
            style,
        }
    }

    fn generate_body(&self) -> String {
        use console::style as textstyle;

        let full_segments: i32 = (self.segments as f32 * self.percentage) as i32;
        let empty_segments: i32 = self.segments - full_segments;

        let bar_full_character = match self.style {
            BarStyle::Classic => '=',
            BarStyle::ConnectedLine => '━',
            BarStyle::Line => '━',
            BarStyle::Hash => '#',
            BarStyle::DottedLine => '━',
            BarStyle::Rect => '\u{25A0}',
            BarStyle::RectFull => '\u{25A0}',
            BarStyle::HollowRect => '\u{25A0}',
            BarStyle::Full => '\u{2588}',
            _ => 'd',
        };

        let bar_space_character = match self.style {
            BarStyle::Classic => ' ',
            BarStyle::ConnectedLine => '━',
            BarStyle::Line => '━',
            BarStyle::Hash => '-',
            BarStyle::DottedLine => '╺',
            BarStyle::Rect => ' ',
            BarStyle::RectFull => '\u{25A0}',
            BarStyle::HollowRect => '\u{25A1}',
            BarStyle::Full => '\u{2591}',
            _ => 'd',
        };

        let left_char: char = match self.style {
            BarStyle::Classic => '=',
            BarStyle::ConnectedLine => '━',
            BarStyle::Line => '━',
            BarStyle::Hash => '#',
            BarStyle::DottedLine => '━',
            _ => 'd',
        };

        let right_char: char = match self.style {
            BarStyle::Classic => '>',
            BarStyle::Line => '╸',
            _ => 'd',
        };

        let body = match self.style {
            BarStyle::Classic => {
                let space: String = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                        + right_char.to_string().as_str()
                        + bar_space_character.to_string().as_str()
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::ConnectedLine => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::Line => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                        + right_char.to_string().as_str()
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::Hash => {
                let space: String = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .yellow()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::DottedLine => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::Rect => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::RectFull => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            BarStyle::HollowRect => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                        + &space
                )
                    .cyan()
                    .bold()
                    .to_string()

            }
            BarStyle::Full => {
                let space = bar_space_character
                    .to_string()
                    .repeat(empty_segments as usize);

                textstyle(
                    bar_full_character
                        .to_string()
                        .repeat(full_segments as usize)
                )
                    .cyan()
                    .bold()
                    .to_string()
                    + &space
            }
            _ => self.title.to_string(),
        };
        body
    }

    fn get_progressbar_string(&self) -> String {
        use console::style as textstyle;

        let body = self.generate_body();

        let bar = format!(
            "{} [{}][{}%] ETA: {}",
            self.title,
            body,
            (self.percentage * 100.0) as i32,
            textstyle(self.eta).green().italic()
        );
        bar
    }
}

fn main() {
    let bar = ProgressBar::new(
        "Downloading files".to_string(),
        0.6,
        30,
        321,
        BarStyle::HollowRect,
    );

    println!("{}", bar.get_progressbar_string());
}

//TODO: change library for coloring text so I can define color in string (now color applies to whole body)
