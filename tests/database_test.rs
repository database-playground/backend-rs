#![feature(async_closure, assert_matches)]

use mimalloc_rust::GlobalMiMalloc;

mod common;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

#[cfg(all(test, feature = "test_database"))]
mod tests {
    use std::assert_matches::assert_matches;

    use backend::{
        self,
        db::{self, Cursor},
    };
    use tokio;

    mod schema {
        #[tokio::test]
        async fn test_get_schema() {
            let schema = crate::common::run_onetime(async |c| {
                backend::db::seeder::seed_connection(c).await?;

                let result = backend::db::get_schema(c, "shop").await?;
                Ok(result)
            })
            .await
            .expect("failed to get schema");

            assert_eq!(schema.schema_id, "shop");
            assert_eq!(schema.picture, None);
            assert_eq!(schema.description, "The schema that is for a shop");

            println!("{schema:?}");
        }

        #[tokio::test]
        async fn test_get_schema_initial_sql() {
            let initial_sql = crate::common::run_onetime(async |c| {
                backend::db::seeder::seed_connection(c).await?;

                let result = backend::db::get_schema_initial_sql(c, "shop").await?;
                Ok(result)
            })
            .await
            .expect("failed to get schema initial sql");

            assert!(
                initial_sql.contains("CREATE TABLE products"),
                "initial sql does not contain CREATE TABLE products"
            );

            println!("{initial_sql}");
        }
    }

    #[tokio::test]
    async fn test_list_questions_default_cursor() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(c, Cursor::default()).await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 10, "default offset=0, limit=10");
        assert_eq!(questions[0].title, "Find a product in the shop");
    }

    #[tokio::test]
    async fn test_list_questions_offset_1_limit_5() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(
                c,
                Cursor {
                    offset: Some(1),
                    limit: Some(5),
                },
            )
            .await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 5, "offset=1, limit=5");
        assert_eq!(questions[0].title, "List all customers");
    }

    #[tokio::test]
    async fn test_list_questions_offset_5_limit_5() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(
                c,
                Cursor {
                    offset: Some(5),
                    limit: Some(5),
                },
            )
            .await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 5, "offset=5, limit=5");
        assert_eq!(questions[0].title, "Find a book by title");
    }

    #[tokio::test]
    async fn test_list_questions_offset_0_limit_5() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(
                c,
                Cursor {
                    offset: Some(0),
                    limit: Some(5),
                },
            )
            .await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 5, "offset=0, limit=5");
        assert_eq!(questions[0].title, "Find a product in the shop");
    }

    #[tokio::test]
    async fn test_list_questions_limit_0() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(
                c,
                Cursor {
                    limit: Some(0),
                    ..Default::default()
                },
            )
            .await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 0, "limit=0 should returns nothing");
    }

    #[tokio::test]
    async fn test_list_questions_limit_none() {
        let questions = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::list_questions(
                c,
                Cursor {
                    limit: Some(10),
                    ..Default::default()
                },
            )
            .await?;
            Ok(result)
        })
        .await
        .expect("failed to list questions");

        assert_eq!(questions.len(), 10, "limit=None should returns 10");
        assert_eq!(questions[0].title, "Find a product in the shop");
    }

    #[tokio::test]
    async fn test_get_question() {
        let question = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question(c, 1).await?)
        })
        .await
        .expect("failed to get question");

        assert_eq!(question.title, "Find a product in the shop");
    }

    #[tokio::test]
    async fn test_get_question_answer() {
        let question_answer = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_answer(c, 1).await?)
        })
        .await
        .expect("failed to get question answer");

        assert_eq!(
            question_answer,
            "SELECT * FROM products WHERE product_name = 'Laptop';"
        );
    }

    #[tokio::test]
    async fn test_get_question_solution_not_null() {
        let question_solution = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_solution(c, 1).await?)
        })
        .await
        .expect("failed to get question solution");

        assert_matches!(
            question_solution,
            Some(url) if url == "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        );
    }

    #[tokio::test]
    async fn test_get_question_solution_null() {
        let question_solution = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_solution(c, 2).await?)
        })
        .await
        .expect("failed to get question solution");

        assert_eq!(question_solution, None);
    }

    #[tokio::test]
    async fn test_get_question_not_found() {
        let question = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question(c, 114514).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(
            question,
            Err(db::Error::NotFound {
                entity: "question",
                id,
            }) if id == "114514",
            "question should not be found"
        );
    }

    #[tokio::test]
    async fn test_get_question_answer_not_found() {
        let question_answer = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_answer(c, 114514).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(
            question_answer,
            Err(db::Error::NotFound {
                entity: "question",
                id,
            }) if id == "114514",
            "question answer should not be found"
        );
    }

    #[tokio::test]
    async fn test_get_question_solution_not_found() {
        let question_solution = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_solution(c, 114514).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(
            question_solution,
            Err(db::Error::NotFound {
                entity: "question",
                id,
            }) if id == "114514",
            "question solution should not be found"
        );
    }

    #[tokio::test]
    async fn test_get_question_deleted() {
        let question = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question(c, 20).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(question, Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "20");
    }

    #[tokio::test]
    async fn test_get_question_answer_deleted() {
        let question_answer = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_answer(c, 20).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(question_answer, Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "20");
    }

    #[tokio::test]
    async fn test_get_question_solution_deleted() {
        let question_solution = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;
            Ok(backend::db::get_question_solution(c, 20).await?)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(question_solution, Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "20");
    }
}
