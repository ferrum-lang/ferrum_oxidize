#[derive(Clone, Debug, Copy)]
pub struct Span {
    pub from: SpanPoint,
    pub to: SpanPoint,
}

impl From<SpanPoint> for Span {
    fn from(value: SpanPoint) -> Self {
        return Self {
            from: value.clone(),
            to: value,
        };
    }
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        return Self {
            from: SpanPoint::from(value.clone()),
            to: SpanPoint::from(value),
        };
    }
}

impl<A: Into<SpanPoint>, B: Into<SpanPoint>> From<(A, B)> for Span {
    fn from(values: (A, B)) -> Self {
        return Self {
            from: values.0.into(),
            to: values.1.into(),
        };
    }
}

impl From<(Span, Span)> for Span {
    fn from(values: (Span, Span)) -> Self {
        return Self {
            from: values.0.from,
            to: values.1.to,
        };
    }
}

impl From<(Option<Span>, Span)> for Span {
    fn from(values: (Option<Span>, Span)) -> Self {
        return Self {
            from: values.0.unwrap_or(values.1).from,
            to: values.1.to,
        };
    }
}

#[derive(Clone, Debug, Copy)]
pub struct SpanPoint {
    pub line: usize,
    pub column: usize,
}

impl From<(usize, usize)> for SpanPoint {
    fn from(value: (usize, usize)) -> Self {
        return Self {
            line: value.0,
            column: value.1,
        };
    }
}

