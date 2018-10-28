use clap;

#[derive(Clone)]
pub struct Arguments {
    inputs: Vec<String>,
    video: String,
    english: Vec<String>,
    ukrainian: Vec<String>,
    russian: Vec<String>,
    other: Vec<String>,
    subtitles: Vec<String>,
    track_names: Vec<String>,
    language: String,
    title: String,
    destination: String,
    output_path: String,
    dummy: bool,
}

impl Arguments {
    pub fn from_cmd() -> Result<Self, ()> {
        let mut iter = 0..<usize>::max_value();
        let args = clap::App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                clap::Arg::with_name("input")
                    .display_order(iter.next().unwrap())
                    .help("Input media files")
                    .short("i")
                    .long("input")
                    .value_name("STRING")
                    .takes_value(true)
                    .required(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("video")
                    .display_order(iter.next().unwrap())
                    .help("The video stream specifier")
                    .short("v")
                    .long("video")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .default_value("0"),
            )
            .arg(
                clap::Arg::with_name("english")
                    .display_order(iter.next().unwrap())
                    .help("English audio stream specifiers")
                    .short("e")
                    .long("audio-english")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("ukrainian")
                    .display_order(iter.next().unwrap())
                    .help("Ukrainian audio stream specifiers")
                    .short("u")
                    .long("audio-ukrainian")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("russian")
                    .display_order(iter.next().unwrap())
                    .help("Russian audio stream specifiers")
                    .short("r")
                    .long("audio-russian")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("other")
                    .display_order(iter.next().unwrap())
                    .help("Other audio stream specifiers")
                    .short("o")
                    .long("audio-other")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("subtitles")
                    .display_order(iter.next().unwrap())
                    .help("Subtitle stream specifiers")
                    .short("s")
                    .long("subtitles")
                    .value_name("INT:INT")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("track-names")
                    .display_order(iter.next().unwrap())
                    .help("Track names")
                    .short("t")
                    .long("track-names")
                    .value_name("STRING")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                clap::Arg::with_name("language")
                    .display_order(iter.next().unwrap())
                    .help("The output movie language")
                    .short("l")
                    .long("language")
                    .value_name("STRING")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("prefix")
                    .display_order(iter.next().unwrap())
                    .help("The output movie name prefix")
                    .short("p")
                    .long("prefix")
                    .value_name("STRING")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("name")
                    .display_order(iter.next().unwrap())
                    .help("The output movie name")
                    .short("n")
                    .long("name")
                    .value_name("STRING")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("destination")
                    .display_order(iter.next().unwrap())
                    .help("The output movie destination directory")
                    .short("d")
                    .long("destination")
                    .value_name("STRING")
                    .takes_value(true)
                    .required(true)
                    .default_value("."),
            )
            .get_matches();

        let language = args.value_of("language").unwrap_or("eng").to_owned();
        let title = args.value_of("name").unwrap_or_default();
        let destination = args.value_of("destination").unwrap_or(".").to_owned();
        let dummy = args.value_of("name").is_none();

        Ok(Arguments {
            inputs: args
                .values_of("input")
                .unwrap()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            video: args.value_of("video").unwrap_or_default().to_owned(),
            english: args
                .values_of("english")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            ukrainian: args
                .values_of("ukrainian")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            russian: args
                .values_of("russian")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            other: args
                .values_of("other")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            subtitles: args
                .values_of("subtitles")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect::<Vec<String>>(),
            track_names: args
                .values_of("track-names")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .map(|v| {
                    if v.to_lowercase() == "o" {
                        "Original".to_owned()
                    } else {
                        v
                    }
                })
                .map(|v| {
                    if v.to_lowercase() == "d" {
                        "Dub".to_owned()
                    } else {
                        v
                    }
                })
                .collect::<Vec<String>>(),
            language: language.to_owned(),
            title: title.to_owned(),
            destination: destination.to_owned(),
            output_path: destination + "/" + args.value_of("prefix").map(|v| v.to_owned() + ".").unwrap_or_default().as_str() + title + ".mkv",
            dummy,
        })
    }

    pub fn inputs(&self) -> &Vec<String> {
        &self.inputs
    }

    pub fn video_stream(&self) -> &str {
        &self.video
    }

    pub fn audio_streams(&self) -> Vec<String> {
        let mut streams = Vec::with_capacity(
            self.english.len() + self.ukrainian.len() + self.russian.len() + self.other.len(),
        );
        streams.append(self.english.clone().as_mut());
        streams.append(self.ukrainian.clone().as_mut());
        streams.append(self.russian.clone().as_mut());
        streams.append(self.other.clone().as_mut());
        streams
    }

    pub fn english_streams(&self) -> &Vec<String> {
        &self.english
    }

    pub fn ukrainian_streams(&self) -> &Vec<String> {
        &self.ukrainian
    }

    pub fn russian_streams(&self) -> &Vec<String> {
        &self.russian
    }

    pub fn other_streams(&self) -> &Vec<String> {
        &self.other
    }

    pub fn subtitle_streams(&self) -> &Vec<String> {
        &self.subtitles
    }

    pub fn track_names(&self) -> &Vec<String> {
        &self.track_names
    }

    pub fn language(&self) -> &str {
        self.language.as_str()
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn output_path(&self) -> &str {
        self.output_path.as_str()
    }

    pub fn dummy(&self) -> bool {
        self.dummy
    }
}

impl ::std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(f, "{0} ARGUMENTS {0}", "_".repeat(32))?;
        for i in 0..self.inputs.len() {
            writeln!(f, "Input #{:04}: {}", i + 1, self.inputs.get(i).unwrap())?;
        }
        if self.dummy {
            writeln!(f, "Dummy mode!")?;
            return Ok(());
        }
        writeln!(f, "Stream HEVC: {}", self.video)?;
        writeln!(f, "Streams ENG: {:?}", self.english)?;
        writeln!(f, "Streams UKR: {:?}", self.ukrainian)?;
        writeln!(f, "Streams RUS: {:?}", self.russian)?;
        writeln!(f, "Streams OTH: {:?}", self.other)?;
        writeln!(f, "Streams SUB: {:?}", self.subtitles)?;
        writeln!(f, "Track names: {:?}", self.track_names)?;
        writeln!(f, "Language   : {}", self.language)?;
        writeln!(f, "Output file: {}", self.title)?;
        writeln!(f, "Output path: {}", self.output_path)?;
        writeln!(f, "{}", "_".repeat(75))?;
        Ok(())
    }
}
