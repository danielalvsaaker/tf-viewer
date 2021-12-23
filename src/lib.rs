use axum::response::{IntoResponse, Response};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> IntoResponse for Either<A, B>
where
    A: IntoResponse,
    B: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Self::Left(left) => left.into_response(),
            Self::Right(right) => right.into_response(),
        }
    }
}
