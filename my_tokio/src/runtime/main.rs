use tokio;

/// + 通过 #[tokio::main] 和 注解（annotation）async main  让自身成为一个async runtime
/// + #[tokio::main] 创建的是多线程runtime,还要一下几种方式也可以创建多线程runtime
/// 1. #[tokio::main(flavor = "multi_thread")] // 等价#[tokio::main]
/// 2. #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
/// 3. #[tokio::main(worker_threads = 10)]
/// 还等价于
/// ```
/// fn main(){
///      // 创建带有线程池的runtime；线程即rust线程即os线程
///      let rt = tokio::runtime::Builder::new_multi_thread()
///          .worker_threads(N)// N个工作线程
///          .enable_all()
///          .build()//创建runtime
///          .unwrap()
///          .block_on(async{...});
/// }
/// ```
///
/// + 创建单线程 #[tokio::main(flavor = "current_thread")] 等价于
/// ```
/// /// fn main(){
///      // 创建带有线程池的runtime；线程即rust线程即os线程
///      let rt = tokio::runtime::Builder::new_current_thread()
///          .enable_all()
///          .build()//创建runtime
///          .unwrap()
///          .block_on(async{...});
/// }
/// ```
#[tokio::main]
async fn main(){}