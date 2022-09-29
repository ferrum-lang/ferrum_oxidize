#[derive(Debug, Clone)]
pub enum Punctuated<T: std::fmt::Debug, D: std::fmt::Debug> {
    Empty,
    NonEmpty {
        first: T,
        rest: Vec<(D, T)>,
        trailing: Option<D>,
    },
}

impl<T: std::fmt::Debug, D: std::fmt::Debug> Punctuated<T, D> {
    pub fn new() -> Self {
        return Self::Empty;
    }

    pub fn push_first(&mut self, value: T) {
        match self {
            Self::Empty => {
                *self = Self::NonEmpty {
                    first: value,
                    rest: vec![],
                    trailing: None,
                };
            }
            _ => panic!("Cannot push first to self: {self:#?}"),
        }
    }

    pub fn push_delim(&mut self, delim: D, value: T) {
        match self {
            Self::NonEmpty { rest, .. } => {
                rest.push((delim, value));
            }
            _ => panic!("Cannot push delimited to self: {self:#?}"),
        }
    }

    pub fn push_trailing(&mut self, delim: D) {
        match self {
            Self::NonEmpty { trailing, .. } => {
                *trailing = Some(delim);
            }
            _ => panic!("Cannot push trailing to self: {self:#?}"),
        }
    }

    pub fn push(&mut self, delim: Option<D>, value: T) {
        match delim {
            None => self.push_first(value),
            Some(delim) => self.push_delim(delim, value),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => return 0,
            Self::NonEmpty { rest, .. } => return 1 + rest.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    pub fn take(self) -> Option<(T, Vec<(D, T)>, Option<D>)> {
        match self {
            Self::Empty => return None,
            Self::NonEmpty {
                first,
                rest,
                trailing,
            } => return Some((first, rest, trailing)),
        }
    }

    pub fn take_as_vec(self) -> Vec<(T, Option<D>)> {
        match self {
            Self::Empty => return vec![],
            Self::NonEmpty {
                first,
                rest,
                trailing,
            } => {
                let mut values = vec![];

                let mut to_push = (first, None);

                for (delim, value) in rest {
                    to_push.1 = Some(delim);
                    values.push(to_push);

                    to_push = (value, None);
                }

                to_push.1 = trailing;
                values.push(to_push);

                return values;
            }
        }
    }

    pub fn take_values(self) -> Vec<T> {
        match self {
            Self::Empty => return vec![],
            Self::NonEmpty {
                first,
                rest,
                trailing,
            } => {
                let mut values = vec![first];

                for (_, value) in rest {
                    values.push(value);
                }

                return values;
            }
        }
    }
}
