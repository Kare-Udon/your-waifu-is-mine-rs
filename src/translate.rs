pub mod translate {
    use rustlate;

    pub fn translate(text: String) -> Result<String, E> {
        let translator_struct = rustlate::Translator {
            to: "zh-CN",
            from: "auto",
        };
        translator_struct.translate(&*text)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn translate_test() {
            use super::*;
            let translated = translate("I had a good time!".to_string());
            assert_eq!(translated, "我玩得很开心！");
        }
    }
}