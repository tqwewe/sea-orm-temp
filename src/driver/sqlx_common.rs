use crate::DbErr;

pub fn sqlx_error_to_exec_err(err: sqlx::Error) -> DbErr {
    DbErr::Exec(err.to_string())
}

pub fn sqlx_error_to_query_err(err: sqlx::Error) -> DbErr {
    DbErr::Query(err.to_string())
}
