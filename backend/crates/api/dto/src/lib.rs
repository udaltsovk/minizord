#![allow(clippy::empty_docs)] // TODO: remove this

use garde::Validate;
use serde::Deserialize;
use utoipa::IntoParams;

pub mod auth;
pub mod image;
pub mod profile;
pub mod review;
pub mod team;
pub mod tour;
pub mod user;

///
#[derive(Deserialize, Validate, IntoParams, Debug)]
#[into_params(style = Form, parameter_in = Query)]
pub struct Pagination {
    ///
    #[param(format = UInt16, minimum = 0, maximum = 57, default = 7)]
    #[garde(range(min = 0))]
    limit: Option<i64>,

    ///
    #[param(format = UInt64, minimum = 0, default = 0)]
    #[garde(range(min = 0))]
    offset: Option<i64>,
}
impl From<Pagination> for (u16, u64) {
    fn from(
        Pagination {
            limit,
            offset,
        }: Pagination,
    ) -> Self {
        (
            limit
                .map(|limit| limit.min(57))
                .map(u16::try_from)
                .and_then(Result::ok)
                .unwrap_or(7),
            offset.map(u64::try_from).and_then(Result::ok).unwrap_or(0),
        )
    }
}
