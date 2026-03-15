use helios_proto::distributed_compiler_server::DistributedCompiler;
use helios_proto::{Compile_Response, CompileTask, NodeInfo, NodeStatus};
use tonic::{Request, Response, Status};
use std::fs;
use std::time::Instant;

#[derive(default)]
pub struct HeliosCompilerService;

#[tonic::async_trait]
impl DistributedCompiler for HeliosCompilerService {
    async fn dispatch_task(
        &self,
        request: Request<CompileTask>,
    ) -> Result<Response<CompileResponse>, Status> {
        let request = request.into_inner();
        println!("gRPC: Received request: {}", req.task_id);
        let start_time = Instant::now();
        let task_id = req.task_id.clone();

        let response = tokio::task::spawn_blocking(move || {
            let temp_dir = std::env::temp_dir().join(format!("helios_task_{}", task_id));
            let _ = fs::create_dir_all(&temp_dir);

            let source_path = temp_dir.join(&req.source_filename);
            let _ = fs::write(&source_path, &req.source_content);
            let compiler_exe = if req.compiler_version.is_empty() {
                "clang++".to_string()
            } else {
                req.compiler_version.clone()
            };

            let mut args = req.compiler_flags.clone();
            args.push(source_path.to_string_lossy().to_string());

            let obj_filename = format!("{}.o", req.source_filename);
            let obj_path = temp_dir.join(&obj_filename);
            args.push("-o".to_string());
            args.push(obj_path.to_string_lossy().to_string());

            let (exit_code, stdout, stderr) = crate::execution::executor::executor_compiler_task(
                &compiler_exe,
                &args,
            ).unwrap_or((-1, String::new(), "Failed to execute compiler sandbox".to_string()));
            let object_file = fs::read(&obj_path).unwrap_or_default();
            let _ = fs::remove_dir_all(&temp_dir);

            CompileResponse {
                task_id: req.task_id,
                exit_code,
                object_file,
                stdout_logs: stdout,
                stderr_logs: stderr,
                duration_ms: start_time.elapsed().as_millis() as u64,
            }
        }).await.map_err(|e| Status::internal(format!("Task panicked: {}", e)))?;
        Ok(Response::new(response))
    }

    async fn ping(&self, _request: Request<NodeInfo>) -> Result<Response<NodeStatus>, Status> {
        let max_threads = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);

        Ok(Response::new(NodeStatus {
            accepts_tasks: true,
            active_jobs: 0,
            max_concurrency: max_threads,
        }))
    }
}