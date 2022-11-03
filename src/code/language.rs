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

    pub fn module_name(&self) -> &str {
        match self {
            Language::Java => "java",
            Language::Kotlin => "kotlin",
        }
    }
}
