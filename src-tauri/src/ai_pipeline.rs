use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::time::Duration;
use crate::db::Db;

#[derive(Debug, Clone)]
pub struct AiTask {
    pub photo_id: String,
    pub task_type: String,
}

pub struct AiPipeline {
    sender: mpsc::Sender<AiTask>,
}

impl AiPipeline {
    pub fn new(db: Arc<StdMutex<Db>>, concurrency_limit: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<AiTask>(100);
        let receiver = Arc::new(Mutex::new(receiver));
        
        for _ in 0..concurrency_limit {
            let rx = receiver.clone();
            let db_clone = db.clone();
            
            tokio::spawn(async move {
                loop {
                    let task = {
                        let mut rx = rx.lock().await;
                        rx.recv().await
                    };
                    
                    match task {
                        Some(task) => {
                            Self::process_task(task, db_clone.clone()).await;
                        }
                        None => break, // Channel closed
                    }
                }
            });
        }
        
        Self { sender }
    }
    
    pub async fn submit_task(&self, task: AiTask) {
        let _ = self.sender.send(task).await;
    }
    
    async fn process_task(task: AiTask, db: Arc<StdMutex<Db>>) {
        println!("Processing task: {:?}", task);
        
        // Mock processing time
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        match task.task_type.as_str() {
            "CLIP" => {
                // Mock CLIP embedding processing
                println!("Finished CLIP processing for photo {}", task.photo_id);
                let photo_id_num = task.photo_id.parse::<i64>().unwrap_or(0);
                if photo_id_num > 0 {
                    let db_lock = db.lock().unwrap();
                    let _ = db_lock.add_tag(photo_id_num, "ai-clip-processed");
                }
            }
            "FACE" => {
                // Mock face detection
                println!("Finished Face detection for photo {}", task.photo_id);
                let photo_id_num = task.photo_id.parse::<i64>().unwrap_or(0);
                if photo_id_num > 0 {
                    let db_lock = db.lock().unwrap();
                    let _ = db_lock.add_tag(photo_id_num, "ai-face-processed");
                }
            }
            "OCR" => {
                // Mock OCR extraction
                println!("Finished OCR for photo {}", task.photo_id);
                let photo_id_num = task.photo_id.parse::<i64>().unwrap_or(0);
                if photo_id_num > 0 {
                    let db_lock = db.lock().unwrap();
                    let _ = db_lock.add_tag(photo_id_num, "ai-ocr-processed");
                }
            }
            _ => {
                println!("Unknown task type: {}", task.task_type);
            }
        }
    }
}
