use crate::impl_send_sync;
use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct WordsFile {
    words: Vec<HashSet<String>>,
}

impl_send_sync!(WordsFile);

impl WordsFile {
    pub fn from_str(s: &str) -> Self {
        toml::from_str(s).unwrap()
    }

    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::from_str(&contents)
    }

    pub fn slice(&self, start: usize, end: usize) -> HashSet<&String> {
        let mut result = HashSet::new();
        self.words[start..end + 1]
            .iter()
            .for_each(|v| result.extend(v.iter()));
        result
    }
}

#[derive(Debug, Resource)]
pub struct AllWords(WordsFile);

impl Default for AllWords {
    fn default() -> Self {
        Self(WordsFile::from_file("assets/words.toml"))
    }
}

#[derive(Debug, Resource)]
pub struct CommonWords(WordsFile);

impl Default for CommonWords {
    fn default() -> Self {
        Self(WordsFile::from_file("assets/common.toml"))
    }
}

pub fn setup_words(mut commands: Commands) {
    commands.init_resource::<AllWords>();
    commands.init_resource::<CommonWords>();
}
