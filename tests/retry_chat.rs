use kwaak::commands::Command;
use kwaak::config::Config;
use kwaak::frontend::{ui, UIEvent, UserInputCommand};
use kwaak::test_utils::{setup_integration_with_config, IntegrationContext};
use kwaak::{assert_agent_responded, assert_command_done};

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn retry_chat() {
    let IntegrationContext {
        mut app,
        uuid,
        mut terminal,

        handler_guard,
        repository_guard: _repository_guard,
        ..
    } = setup_integration().await.unwrap();

    // First, let's start a noop agent so an environment is running
    app.dispatch_command(
        uuid,
        Command::Chat {
            message: "hello".to_string(),
        },
    );

    assert_agent_responded!(app, uuid);
    assert_command_done!(app, uuid);

    terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    insta::assert_snapshot!("before_retry", terminal.backend());

    // Let's retry the chat
    app.send_ui_event(UIEvent::UserInputCommand(uuid, UserInputCommand::Retry));

    assert_agent_responded!(app, uuid);
    assert_command_done!(app, uuid);

    // It should now show 2 messages

    terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    insta::assert_snapshot!("after_retry", terminal.backend());

    // Force dropping it, for some reason it's not being dropped
    drop(handler_guard);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn auto_retry_chat() {
    let mut config = Config::default();
    config.autoretry = 2;

    let IntegrationContext {
        mut app,
        uuid,
        mut terminal,
        handler_guard,
        repository_guard: _repository_guard,
        ..
    } = setup_integration_with_config(config).await.unwrap();

    // Start a failing chat that should auto-retry
    app.dispatch_command(
        uuid,
        Command::Chat {
            message: "trigger_failure".to_string(),
        },
    );

    assert_agent_responded!(app, uuid);
    assert_command_done!(app, uuid);

    terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    insta::assert_snapshot!("auto_retry", terminal.backend());

    // Force dropping it
    drop(handler_guard);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn auto_retry_and_manual_retry() {
    let mut config = Config::default();
    config.autoretry = 2;

    let IntegrationContext {
        mut app,
        uuid,
        mut terminal,
        handler_guard,
        repository_guard: _repository_guard,
        ..
    } = setup_integration_with_config(config).await.unwrap();

    // Start a failing chat that should auto-retry
    app.dispatch_command(
        uuid,
        Command::Chat {
            message: "trigger_failure".to_string(),
        },
    );

    assert_agent_responded!(app, uuid);
    assert_command_done!(app, uuid);

    // Now try manual retry which should also trigger auto-retry
    app.send_ui_event(UIEvent::UserInputCommand(uuid, UserInputCommand::Retry));

    assert_agent_responded!(app, uuid);
    assert_command_done!(app, uuid);

    terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    insta::assert_snapshot!("auto_retry_and_manual", terminal.backend());

    // Force dropping it
    drop(handler_guard);
}
