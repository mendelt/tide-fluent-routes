/// Newtype for collecting path segments into a path
pub(crate) struct Path(String);

impl Path {
    pub fn new() -> Self {
        Path(String::new())
    }

    pub fn join(self, segment: &str) -> Path {
        if self.0.is_empty() {
            Path(segment.to_string())
        } else {
            let mut path = self.0.trim_end_matches('/').to_string();
            path.push('/');
            path.push_str(segment.trim_start_matches('/'));
            Path(path)
        }
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_add_slash_between_segments() {
        let path = Path::new().join("tst").join("tst");

        assert_eq!(path.to_string(), "tst/tst");
    }

    #[test]
    fn should_preserve_slash_prefix() {
        let path = Path::new().join("/tst").join("tst");

        assert_eq!(path.to_string(), "/tst/tst");
    }

    #[test]
    fn should_preserve_trailing_slash() {
        let path = Path::new().join("tst").join("tst/");

        assert_eq!(path.to_string(), "tst/tst/");
    }

    #[test]
    fn should_not_add_slashes_when_provided() {
        let path = Path::new().join("/tst").join("/tst/");

        assert_eq!(path.to_string(), "/tst/tst/");
    }

    #[test]
    fn should_remove_extra_slashes() {
        let path = Path::new().join("tst/").join("/tst/");

        assert_eq!(path.to_string(), "tst/tst/");
    }

    #[test]
    fn should_remove_multiple_extra_slashes() {
        let path = Path::new().join("/tst////").join("///tst/");

        assert_eq!(path.to_string(), "/tst/tst/");
    }
}
