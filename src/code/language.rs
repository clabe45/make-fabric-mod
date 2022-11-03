pub enum Language {
    Java,
    Kotlin,
}

impl Language {
    pub fn extension(&self) -> &str {
        match self {
            Language::Java => "java",
            Language::Kotlin => "kt",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Language::Java => "java",
            Language::Kotlin => "kotlin",
        }
    }
}
