#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use anyhow::Result;
    use indoc::indoc;
    use mockito::mock;
    use octocrab::models::issues::{Issue, IssueState};
    use test_utils::{test_app, test_repository};

    #[tokio::test]
    async fn test_gh_issue_command_handler() -> Result<()> {
        let mock_issue = mock("GET", "/repos/owner/repo/issues/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(indoc! {r#"
                {
                    "number": 1,
                    "title": "Test Issue",
                    "body": "This is a test issue",
                    "state": "open",
                    "user": { "login": "testuser" }
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
                            "user": { "login": "commenter" },
                            "body": "Test comment"
                        }
                    ]
                }
            "#})
            .create();

        let (mut repository, _) = test_repository();
        repository.config_mut().github_api_key = Some("test_token".into());
        repository.config_mut().git.owner = Some("owner".into());
        repository.config_mut().git.repository = Some("repo".into());

        let mut handler = CommandHandler::from_repository(repository);
        let (mut app, _runtime) = test_app();
        handler.register_ui(&mut app);

        let event = CommandEvent::builder()
            .command(Command::GhIssue { issue_number: 1 })
            .uuid(uuid::Uuid::new_v4())
            .responder(Arc::new(app))
            .build()?;

        handler.handle_command_event(&event.into(), &event, &event.command()).await?;

        mock_issue.assert();
        mock_comments.assert();

        Ok(())
    }

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
                    "user": { "login": "testuser" }
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
                            "user": { "login": "commenter" },
                            "body": "Test comment"
                        }
                    ]
                }
            "#})
            .create();

        let (mut repository, _) = test_repository();
        repository.config_mut().github_api_key = Some("test_token".into());
        repository.config_mut().git.owner = Some("owner".into());
        repository.config_mut().git.repository = Some("repo".into());

        let session = GithubSession::from_repository(&repository)?;
        let issue_with_comments = session.get_issue(1).await?;

        assert_eq!(issue_with_comments.issue.number, 1);
        assert_eq!(issue_with_comments.issue.title, "Test Issue");
        assert_eq!(issue_with_comments.issue.body.unwrap(), "This is a test issue");
        assert_eq!(issue_with_comments.issue.state, IssueState::Open);
        assert_eq!(issue_with_comments.issue.user.login, "testuser");

        assert_eq!(issue_with_comments.comments.len(), 1);
        assert_eq!(issue_with_comments.comments[0].user.login, "commenter");
        assert_eq!(issue_with_comments.comments[0].body.unwrap(), "Test comment");

        mock_issue.assert();
        mock_comments.assert();

        Ok(())
    }
}
