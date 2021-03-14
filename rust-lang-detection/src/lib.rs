use lingua::{Language, LanguageDetectorBuilder};
use lingua::{
    Language::{English, French, German, Spanish},
    LanguageDetector,
};

/// Creates a detector - only need to do this once per run, and it takes quite a lot of time.
pub fn create_detector() -> LanguageDetector {
    let languages = vec![English, French, German, Spanish];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    detector
}

/// Gets the language that was detected as the one text was in most likely
pub fn get_lang(detector: &LanguageDetector, text: &str) -> Option<Language> {
    let detected_language: Option<Language> = detector.detect_language_of(text);
    detected_language
}

/// Gets a list of languages that the text was possibly in, with likelihoods in tuples
pub fn get_lang_list(detector: &LanguageDetector, text: &str) -> Vec<(Language, f64)> {
    let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(text);

    dbg!(&confidence_values);
    confidence_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en_lang_check() {
        let detector = create_detector();

        let list = get_lang_list(&detector, "a typical sentence with only a few words.");
        assert_eq!(list[0], (English, 1.0));

        let lang = get_lang(&detector, "text english text words and books and letters");
        assert_eq!(lang, Some(English));

        let lang = get_lang(&detector, "another sentence with many words. how do you do");
        assert_eq!(lang, Some(English));

        let lang = get_lang(&detector, "there once lived a marry mary");
        assert_eq!(lang, Some(English));

        let lang = get_lang(&detector, "destruction destitution frustration programming");
        assert_eq!(lang, Some(English));

        let lang = get_lang(&detector, "how does one think about things in abstract");
        assert_eq!(lang, Some(English));
    }
}
