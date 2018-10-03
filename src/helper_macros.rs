
#[macro_export]
macro_rules! db_find {
    ($em:expr, $sql:expr, $tykey:ty) => ({
        let mut vec_ret = $em.execute_sql_with_return(&$sql, &[]).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}

#[macro_export]
macro_rules! db_filter {
    ($em:expr, $sql:expr, $tykey:ty) => ({
        $em.execute_sql_with_return(&$sql, &[]).unwrap_or(Vec::<$tykey>::new())
    })
}

#[macro_export]
macro_rules! db_insert {
    ($em:expr, $instance:expr, $tykey:ty) => ({
        let mut vec_ret = $em.insert(&[$instance]).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}

#[macro_export]
macro_rules! db_update {
    ($em:expr, $sql:expr, $tykey:ty) => ({
        let mut vec_ret = $em.execute_sql_with_return(&$sql, &[]).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}
