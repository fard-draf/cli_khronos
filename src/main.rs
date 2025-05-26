use cli_khronos::{error::AppError, service::FileService, tracing::init_tracing};

fn main() -> Result<(), AppError> {
    init_tracing();
    let mut setup = FileService::new("saved_logged_tasks.json")?;

    setup.initializing()?;

    setup.instance.iter().for_each(|(id, task)| {
        println!("{:?}", task.title);
        println!("{:?}", task.timeline);
    });

    Ok(())
}
