// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

macro_rules! try_from_str {
    ($t:ty) => {
        impl TryFrom<$t> for Status {
            type Error = ();
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                match value.to_lowercase().as_str() {
                    "todo" => Ok(Status::ToDo),
                    "inprogress" => Ok(Status::InProgress),
                    "done" => Ok(Status::Done),
                    _ => Err(()),
                }
            }
        }
    };
}

try_from_str!(String);
try_from_str!(&str);

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
