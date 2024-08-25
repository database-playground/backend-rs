#![cfg(all(test, feature = "test_database"))]

mod test_get_or_initialize_user {
    use sqlx::PgPool;

    #[sqlx::test(fixtures("group", "user"))]
    async fn test_created_without_group(pool: PgPool) {
        let user = backend::db::get_or_initialize_user(&pool, "usergeneric0")
            .await
            .expect("failed to get user");

        assert_eq!(user.user_id, "usergeneric0");
        assert_eq!(user.group_id, None);
    }

    #[sqlx::test(fixtures("group", "user"))]
    async fn test_created_with_group(pool: PgPool) {
        let user = backend::db::get_or_initialize_user(&pool, "usergroup1")
            .await
            .expect("failed to get user");

        assert_eq!(user.user_id, "usergroup1");
        assert_eq!(user.group_id, Some(1));
    }

    #[sqlx::test]
    async fn test_not_created(pool: PgPool) {
        let user = backend::db::get_or_initialize_user(&pool, "usertest0")
            .await
            .expect("failed to create user");

        assert_eq!(user.user_id, "usertest0");
        assert_eq!(user.group_id, None);
    }
}

mod test_create_group {
    use backend::db::GroupCreateParameter;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_group_without_description(pool: PgPool) {
        let group_id = backend::db::create_group(
            &pool,
            GroupCreateParameter {
                name: "group0",
                description: None,
            },
        )
        .await
        .expect("failed to create group");

        let group = sqlx::query!(
            r#"SELECT name, description FROM dp_groups WHERE group_id = $1;"#,
            group_id
        )
        .fetch_one(&pool)
        .await
        .expect("failed to fetch group");
        assert_eq!(group.name, "group0");
        assert_eq!(group.description, "");
    }

    #[sqlx::test]
    async fn test_group_with_description(pool: PgPool) {
        let group_id = backend::db::create_group(
            &pool,
            GroupCreateParameter {
                name: "group1",
                description: Some("description1"),
            },
        )
        .await
        .expect("failed to create group");

        let group = sqlx::query!(
            r#"SELECT name, description FROM dp_groups WHERE group_id = $1;"#,
            group_id
        )
        .fetch_one(&pool)
        .await
        .expect("failed to fetch group");
        assert_eq!(group.name, "group1");
        assert_eq!(group.description, "description1");
    }
}

mod test_get_group {
    use sqlx::PgPool;

    #[sqlx::test(fixtures("group"))]
    async fn test_get_group_with_description(pool: PgPool) {
        let group = backend::db::get_group(&pool, 1)
            .await
            .expect("failed to get group");

        assert_eq!(group.name, "group1");
        assert_eq!(group.description, "description1");
    }

    #[sqlx::test(fixtures("group"))]
    async fn test_get_group_without_description(pool: PgPool) {
        let group = backend::db::get_group(&pool, 3)
            .await
            .expect("failed to get group");

        assert_eq!(group.name, "group3");
        assert_eq!(group.description, "");
    }
}
