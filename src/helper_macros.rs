
#[macro_export]
macro_rules! db_find_by_sql {
    ($em:expr, $sql:expr, $tykey:ty) => ({
        let mut vec_ret = $em.execute_sql_with_return(&$sql, &[]).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}

#[macro_export]
macro_rules! db_select_by_sql {
    ($em:expr, $sql:expr, $tykey:ty) => ({
        $em.execute_sql_with_return(&$sql, &[]).unwrap_or(Vec::<$tykey>::new())
    })
}

#[macro_export]
macro_rules! db_find {
    ($em:expr, $head_clause:expr, $from_clause:expr, $rest_clause:expr, $tykey:ty) => ({
        let mut vec_ret = $em.select($head_clause, $from_clause, $rest_clause).unwrap_or(Vec::<$tykey>::new())
        vec_ret.pop()
    })
}

#[macro_export]
macro_rules! db_select {
    ($em:expr, $head_clause:expr, $from_clause:expr, $rest_clause:expr, $tykey:ty) => ({
        $em.select($head_clause, $from_clause, $rest_clause).unwrap_or(Vec::<$tykey>::new())
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
    ($em:expr, $instance:expr, $rest_clause:expr, $tykey:ty) => ({
        let mut vec_ret = $em.update($instance, $rest_clause).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}

#[macro_export]
macro_rules! db_delete {
    ($em:expr, $rest_clause:expr, $tykey:ty) => ({
        let mut vec_ret = $em.delete($rest_clause).unwrap_or(Vec::<$tykey>::new()); 
        vec_ret.pop()
    })
}

