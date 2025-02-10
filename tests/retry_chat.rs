use kwaak::commands::Command;
use kwaak::frontend::{ui, UIEvent, UserInputCommand};
use kwaak::test_utils::{setup_integration, IntegrationContext};
use kwaak::{assert_agent_responded, assert_command_done};
use std::fs;
use toml::Value;

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

// Test that the autoretry config option works
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_autoretry_config() {
    let mut ctx = setup_integration().await.unwrap();
    
    // Modify the config file to enable autoretry
    let config_content = fs::read_to_string("kwaak.toml").unwrap();
    let mut config: Value = config_content.parse().unwrap();
    
    // Add autoretry = 2 to the config
    config.as_table_mut().unwrap().insert("autoretry".into(), Value::Integer(2));
    fs::write("kwaak.toml", toml::to_string(&config).unwrap()).unwrap();

    // Start a new chat that will fail
    ctx.app.dispatch_command(
        ctx.uuid,
        Command::Chat {
            message: "this will fail".to_string(),
        },
    );

    assert_agent_responded!(ctx.app, ctx.uuid);
    assert_command_done!(ctx.app, ctx.uuid);

    // The agent should have tried 3 times (initial + 2 retries)
    // We can verify this by checking the system messages

    ctx.terminal.draw(|f| ui(f, f.area(), &mut ctx.app)).unwrap();
    let term_output = ctx.terminal.backend().to_string();
    
    // Ensure we have retry messages
    assert!(term_output.contains("Retry 1 failed"));
    assert!(term_output.contains("Retry 2 failed"));

    drop(ctx.handler_guard);
}
