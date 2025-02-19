#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use anyhow::Result;
    use indoc::indoc;
    use mockito::mock;
    use octocrab::models::issues::Issue;

    #[tokio::test]
    async fn test_github_session_get_issue() -> Result<()> {
        let mock_issue = mock("GET", "/repos/owner/repo/issues/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(indoc! {r#"
                {
                    "number": 1,
                    "title": "Test Issue",
                    "body": "This is a test issue",
                    "state": "open",
                    "user": {
                        "login": "testuser",
                        "id": 1,
                        "node_id": "test_node",
                        "avatar_url": "http://example.com",
                        "url": "http://example.com"
                    }
                }
            "#})
            .create();

        let mock_comments = mock("GET", "/repos/owner/repo/issues/1/comments")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(indoc! {r#"
                {
                    "items": [
                        {
                            "user": {
                                "login": "commenter",
                                "id": 2,
                                "node_id": "test_node",
                                "avatar_url": "http://example.com",
                                "url": "http://example.com"
                            },
                            "body": "Test comment"
                        }
                    ]
                }
            "#})
            .create();

        let (mut repository, _) = test_utils::test_repository();
        repository.config_mut().github_api_key = Some("test_token".into());
        repository.config_mut().git.owner = Some("owner".into());
        repository.config_mut().git.repository = Some("repo".into());

        let session = GithubSession::from_repository(&repository)?;
        let issue_with_comments = session.get_issue(1).await?;

        assert_eq!(issue_with_comments.issue.number, 1);
        assert_eq!(issue_with_comments.issue.title, "Test Issue");
        assert_eq!(issue_with_comments.issue.body.unwrap(), "This is a test issue");
        assert_eq!(issue_with_comments.issue.user.login, "testuser");

        assert_eq!(issue_with_comments.comments.len(), 1);
        assert_eq!(issue_with_comments.comments[0].user.login, "commenter");
        assert_eq!(issue_with_comments.comments[0].body.unwrap(), "Test comment");

        mock_issue.assert();
        mock_comments.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_get_issue_error_when_github_disabled() -> Result<()> {
        let (repository, _) = test_utils::test_repository();
        let session = GithubSession::from_repository(&repository);

        assert!(session.is_err());
        assert_eq!(
            session.unwrap_err().to_string(),
            "Github is not enabled; make it is properly configured."
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_issue_error_no_owner() -> Result<()> {
        let (mut repository, _) = test_utils::test_repository();
        repository.config_mut().github_api_key = Some("test_token".into());
        repository.config_mut().git.repository = Some("repo".into());

        let session = GithubSession::from_repository(&repository)?;
        let result = session.get_issue(1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No owner configured"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_issue_error_no_repository() -> Result<()> {
        let (mut repository, _) = test_utils::test_repository();
        repository.config_mut().github_api_key = Some("test_token".into());
        repository.config_mut().git.owner = Some("owner".into());

        let session = GithubSession::from_repository(&repository)?;
        let result = session.get_issue(1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No repository configured"
        );

        Ok(())
    }
}
