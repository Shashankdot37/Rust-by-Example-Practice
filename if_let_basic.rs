enum LogEvent {
    Info(String),
    Warning(String),
    Error(u32, String), // error code and message
    Shutdown,
}

fn main(){
    let logs = vec![
    LogEvent::Info("System boot".to_string()),
    LogEvent::Warning("Low disk space".to_string()),
    LogEvent::Error(404, "Not Found".to_string()),
    LogEvent::Error(500, "Internal Server Error".to_string()),
    LogEvent::Shutdown,
    LogEvent::Info("Shutdown complete".to_string()),
];

for log in logs.iter(){
    if let LogEvent::Info(msg) = log{
        println!("Info: {}", msg);
    }
    else if let LogEvent::Warning(msg) = log{
        println!("Warning: {}", msg);
    }
    else if let LogEvent::Error(500, _) = log{
        println!("Critical Error");
    }
    else if let LogEvent::Error(code @ 400..=499, _) = log{
        println!("{}",code);
    }
    else if let LogEvent::Shutdown = log{
        println!("System is shutting down.");
    }
}
}
