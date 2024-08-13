use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Define the Task struct
#[derive(Serialize, Deserialize)]
struct Task {
    id: Uuid,
    title: String,
    description: String,
    due_date: Option<DateTime<Utc>>,
    completed: bool,
}

// Define the Event struct
#[derive(Serialize, Deserialize)]
struct Event {
    id: Uuid,
    title: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

// Define the Stopwatch struct
struct Stopwatch {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl Stopwatch {
    fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.elapsed += start_time.elapsed();
            self.start_time = None;
        }
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::new(0, 0);
    }

    fn elapsed(&self) -> Duration {
        if let Some(start_time) = self.start_time {
            self.elapsed + start_time.elapsed()
        } else {
            self.elapsed
        }
    }
}

// Handlers for tasks
async fn create_task(task: web::Json<Task>) -> impl Responder {
    web::Json(task.into_inner())
}

async fn get_task(task_id: web::Path<Uuid>) -> impl Responder {
    format!("Get task with ID: {}", task_id)
}

async fn update_task(task_id: web::Path<Uuid>, task: web::Json<Task>) -> impl Responder {
    format!("Update task with ID: {}", task_id)
}

async fn delete_task(task_id: web::Path<Uuid>) -> impl Responder {
    format!("Delete task with ID: {}", task_id)
}

// Handlers for events
async fn create_event(event: web::Json<Event>) -> impl Responder {
    web::Json(event.into_inner())
}

async fn get_event(event_id: web::Path<Uuid>) -> impl Responder {
    format!("Get event with ID: {}", event_id)
}

async fn update_event(event_id: web::Path<Uuid>, event: web::Json<Event>) -> impl Responder {
    format!("Update event with ID: {}", event_id)
}

async fn delete_event(event_id: web::Path<Uuid>) -> impl Responder {
    format!("Delete event with ID: {}", event_id)
}

// Handlers for stopwatch
async fn start_stopwatch(stopwatch: web::Data<Arc<Mutex<Stopwatch>>>) -> impl Responder {
    let mut stopwatch = stopwatch.lock().unwrap();
    stopwatch.start();
    "Stopwatch started"
}

async fn stop_stopwatch(stopwatch: web::Data<Arc<Mutex<Stopwatch>>>) -> impl Responder {
    let mut stopwatch = stopwatch.lock().unwrap();
    stopwatch.stop();
    "Stopwatch stopped"
}

async fn reset_stopwatch(stopwatch: web::Data<Arc<Mutex<Stopwatch>>>) -> impl Responder {
    let mut stopwatch = stopwatch.lock().unwrap();
    stopwatch.reset();
    "Stopwatch reset"
}

async fn get_elapsed_time(stopwatch: web::Data<Arc<Mutex<Stopwatch>>>) -> impl Responder {
    let stopwatch = stopwatch.lock().unwrap();
    format!("Elapsed time: {:?}", stopwatch.elapsed())
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stopwatch = Arc::new(Mutex::new(Stopwatch::new()));

    HttpServer::new(move || {
        App::new()
            .data(stopwatch.clone())
            .route("/", web::get().to(|| async { "Hello, this is your backend!" }))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            .route("/events", web::post().to(create_event))
            .route("/events/{id}", web::get().to(get_event))
            .route("/events/{id}", web::put().to(update_event))
            .route("/events/{id}", web::delete().to(delete_event))
            .route("/stopwatch/start", web::post().to(start_stopwatch))
            .route("/stopwatch/stop", web::post().to(stop_stopwatch))
            .route("/stopwatch/reset", web::post().to(reset_stopwatch))
            .route("/stopwatch/elapsed", web::get().to(get_elapsed_time))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
