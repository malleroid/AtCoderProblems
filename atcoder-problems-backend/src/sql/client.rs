use super::models::*;
use super::schema::*;
use super::{FIRST_AGC_EPOCH_SECOND, UNRATED_STATE};

use diesel::dsl::insert_into;
use diesel::pg::upsert::excluded;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryResult;

pub trait SqlClient {
    fn insert_submissions(&self, values: &[Submission]) -> QueryResult<usize>;
    fn insert_contests(&self, values: &[Contest]) -> QueryResult<usize>;
    fn insert_problems(&self, values: &[Problem]) -> QueryResult<usize>;
    fn insert_contest_problem_pair(
        &self,
        contest_problem_pairs: &[(&str, &str)],
    ) -> QueryResult<usize>;
    fn insert_performances(&self, performances: &[Performance]) -> QueryResult<usize>;

    fn get_problems(&self) -> QueryResult<Vec<Problem>>;
    fn get_contests(&self) -> QueryResult<Vec<Contest>>;
    fn get_submissions(&self, user_id: &str) -> QueryResult<Vec<Submission>>;
    fn get_contests_without_performances(&self) -> QueryResult<Vec<String>>;
}

impl SqlClient for PgConnection {
    fn insert_submissions(&self, values: &[Submission]) -> QueryResult<usize> {
        insert_into(submissions::table)
            .values(values)
            .on_conflict(submissions::id)
            .do_update()
            .set((
                submissions::user_id.eq(excluded(submissions::user_id)),
                submissions::result.eq(excluded(submissions::result)),
                submissions::point.eq(excluded(submissions::point)),
                submissions::execution_time.eq(excluded(submissions::execution_time)),
            ))
            .execute(self)
    }

    fn insert_contests(&self, values: &[Contest]) -> QueryResult<usize> {
        insert_into(contests::table)
            .values(values)
            .on_conflict(contests::id)
            .do_nothing()
            .execute(self)
    }

    fn insert_problems(&self, values: &[Problem]) -> QueryResult<usize> {
        insert_into(problems::table)
            .values(values)
            .on_conflict(problems::id)
            .do_nothing()
            .execute(self)
    }

    fn insert_contest_problem_pair(
        &self,
        contest_problem_pairs: &[(&str, &str)],
    ) -> QueryResult<usize> {
        insert_into(contest_problem::table)
            .values(
                contest_problem_pairs
                    .iter()
                    .map(|&(contest, problem)| {
                        (
                            contest_problem::contest_id.eq(contest),
                            contest_problem::problem_id.eq(problem),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .on_conflict((contest_problem::contest_id, contest_problem::problem_id))
            .do_nothing()
            .execute(self)
    }

    fn insert_performances(&self, performances: &[Performance]) -> QueryResult<usize> {
        insert_into(performances::table)
            .values(performances)
            .on_conflict((performances::contest_id, performances::user_id))
            .do_nothing()
            .execute(self)
    }

    fn get_problems(&self) -> QueryResult<Vec<Problem>> {
        problems::dsl::problems.load::<Problem>(self)
    }

    fn get_contests(&self) -> QueryResult<Vec<Contest>> {
        contests::dsl::contests.load::<Contest>(self)
    }

    fn get_submissions(&self, user_id: &str) -> QueryResult<Vec<Submission>> {
        submissions::dsl::submissions
            .filter(submissions::user_id.eq(user_id))
            .load::<Submission>(self)
    }

    fn get_contests_without_performances(&self) -> QueryResult<Vec<String>> {
        contests::table
            .left_join(performances::table.on(performances::contest_id.eq(contests::id)))
            .filter(performances::contest_id.is_null())
            .filter(contests::start_epoch_second.ge(FIRST_AGC_EPOCH_SECOND))
            .filter(contests::rate_change.ne(UNRATED_STATE))
            .select(contests::id)
            .load::<String>(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::connection::SimpleConnection;
    use diesel::Connection;
    use diesel::PgConnection;
    use std::fs::File;
    use std::io::prelude::*;

    fn connect_to_test_sql() -> PgConnection {
        let mut file = File::open("../config/database-definition.sql").unwrap();
        let mut sql = String::new();
        file.read_to_string(&mut sql).unwrap();
        let conn = PgConnection::establish("postgresql://kenkoooo:pass@localhost/test").unwrap();
        conn.batch_execute(&sql).unwrap();
        conn
    }

    #[test]
    fn test_insert_submission() {
        let mut v = vec![Submission {
            id: 0,
            epoch_second: 0,
            problem_id: "".to_owned(),
            contest_id: "".to_owned(),
            user_id: "".to_owned(),
            language: "".to_owned(),
            point: 0.0,
            length: 0,
            result: "".to_owned(),
            execution_time: None,
        }];

        let conn = connect_to_test_sql();
        v[0].id = 1;
        conn.insert_submissions(&v).unwrap();

        let count = submissions::dsl::submissions
            .load::<Submission>(&conn)
            .unwrap()
            .into_iter()
            .count();
        assert_eq!(count, 1);

        v[0].id = 2;
        conn.insert_submissions(&v).unwrap();
        let count = submissions::dsl::submissions
            .load::<Submission>(&conn)
            .unwrap()
            .into_iter()
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_update_submission() {
        let mut v = vec![Submission {
            id: 0,
            epoch_second: 0,
            problem_id: "".to_owned(),
            contest_id: "".to_owned(),
            user_id: "".to_owned(),
            language: "".to_owned(),
            point: 0.0,
            length: 0,
            result: "".to_owned(),
            execution_time: None,
        }];

        let conn = connect_to_test_sql();

        v[0].user_id = "kenkoooo".to_owned();
        v[0].result = "WJ".to_owned();
        v[0].execution_time = None;
        v[0].point = 0.0;
        conn.insert_submissions(&v).unwrap();
        assert_eq!(conn.get_submissions("kenkoooo").unwrap().len(), 1);

        let submissions = conn.get_submissions("kenkoooo").unwrap();
        assert_eq!(submissions[0].result, "WJ".to_owned());
        assert_eq!(submissions[0].user_id, "kenkoooo".to_owned());
        assert_eq!(submissions[0].execution_time, None);
        assert_eq!(submissions[0].point, 0.0);

        v[0].user_id = "a".to_owned();
        v[0].result = "AC".to_owned();
        v[0].execution_time = Some(10);
        v[0].point = 100.0;
        conn.insert_submissions(&v).unwrap();
        assert_eq!(conn.get_submissions("kenkoooo").unwrap().len(), 0);
        assert_eq!(conn.get_submissions("a").unwrap().len(), 1);

        let submissions = conn.get_submissions("a").unwrap();
        assert_eq!(submissions[0].result, "AC".to_owned());
        assert_eq!(submissions[0].user_id, "a".to_owned());
        assert_eq!(submissions[0].execution_time, Some(10));
        assert_eq!(submissions[0].point, 100.0);
    }

    #[test]
    fn test_insert_problems() {
        let conn = connect_to_test_sql();

        assert_eq!(conn.get_problems().unwrap().len(), 0);

        let problems = vec![
            Problem {
                id: "arc001_a".to_owned(),
                contest_id: "arc001".to_owned(),
                title: "Problem 1".to_owned(),
            },
            Problem {
                id: "arc001_b".to_owned(),
                contest_id: "arc001".to_owned(),
                title: "Problem 2".to_owned(),
            },
        ];
        conn.insert_problems(&problems).unwrap();
        assert_eq!(conn.get_problems().unwrap().len(), 2);
    }

    #[test]
    fn test_insert_contests() {
        let conn = connect_to_test_sql();

        assert_eq!(conn.get_contests().unwrap().len(), 0);

        let contests = vec![
            Contest {
                id: "arc001".to_owned(),
                start_epoch_second: 0,
                duration_second: 0,
                title: "Contest 1".to_owned(),
                rate_change: "-".to_owned(),
            },
            Contest {
                id: "arc002".to_owned(),
                start_epoch_second: 0,
                duration_second: 0,
                title: "Contest 2".to_owned(),
                rate_change: "-".to_owned(),
            },
        ];
        conn.insert_contests(&contests).unwrap();

        assert_eq!(conn.get_contests().unwrap().len(), 2);
    }

    #[test]
    fn test_insert_performances() {
        let conn = connect_to_test_sql();

        let contest_id = "contest_id";

        conn.insert_contests(&[
            Contest {
                id: "too_old_contest".to_owned(),
                start_epoch_second: 0,
                duration_second: 0,
                title: "Too Old Contest".to_owned(),
                rate_change: "All".to_owned(),
            },
            Contest {
                id: "unrated_contest".to_owned(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 0,
                title: "Unrated Contest".to_owned(),
                rate_change: "-".to_owned(),
            },
            Contest {
                id: contest_id.to_owned(),
                start_epoch_second: FIRST_AGC_EPOCH_SECOND,
                duration_second: 0,
                title: "Contest 1".to_owned(),
                rate_change: "All".to_owned(),
            },
        ])
        .unwrap();

        let contests_without_performances = conn
            .get_contests_without_performances()
            .expect("Invalid contest extraction query");

        assert_eq!(contests_without_performances, vec![contest_id.to_owned()]);

        conn.insert_performances(&[Performance {
            inner_performance: 100,
            user_id: "kenkoooo".to_owned(),
            contest_id: contest_id.to_owned(),
        }])
        .unwrap();

        let contests_without_performances = conn
            .get_contests_without_performances()
            .expect("Invalid contest extraction query");

        assert!(contests_without_performances.is_empty());
    }
}
