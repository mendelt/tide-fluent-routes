/// Newtype for collecting path segments into a path
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Path(String);

impl Path {
    pub(crate) fn new() -> Self {
        Path(String::new())
    }

    pub(crate) fn append(self, segment: &str) -> Path {
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
    fn should_handle_slashes_between_segments() {
        let path = Path::new()
            .append("/tst1/")
            .append("tst2")
            .append("/tst3/")
            .append("//tst4///")
            .append("/tst5/");

        assert_eq!(path.to_string(), "/tst1/tst2/tst3/tst4/tst5/");
    }

    #[test]
    fn should_preserve_prefix_slash() {
        let path = Path::new().append("/tst1").append("tst2");

        assert_eq!(path.to_string(), "/tst1/tst2");
    }

    #[test]
    fn should_preserve_trailing_slash() {
        let path = Path::new().append("tst1").append("tst2/");

        assert_eq!(path.to_string(), "tst1/tst2/");
    }
}
