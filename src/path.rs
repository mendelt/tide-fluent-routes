/// Newtype for collecting path segments into a path
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Path(String);

impl Path {
    pub fn new() -> Self {
        Path(String::new())
    }

    pub fn prepend(self, segment: &str) -> Path {
        if self.0.is_empty() {
            Path(segment.to_string())
        } else {
            let mut path = segment.trim_end_matches('/').to_string();
            path.push('/');
            path.push_str(self.0.trim_start_matches('/'));
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
            .prepend("tst")
            .prepend("tst")
            .prepend("/tst/")
            .prepend("tst///")
            .prepend("////tst");

        assert_eq!(path.to_string(), "tst/tst/tst/tst/tst");
    }

    #[test]
    fn should_preserve_prefix_slash() {
        let path = Path::new().prepend("/tst").prepend("tst");

        assert_eq!(path.to_string(), "/tst/tst");
    }

    #[test]
    fn should_preserve_trailing_slash() {
        let path = Path::new().prepend("tst").prepend("tst/");

        assert_eq!(path.to_string(), "tst/tst/");
    }
}
