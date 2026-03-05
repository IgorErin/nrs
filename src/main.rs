use tokio::net::TcpListener;

use nfs_mamont::serializer;

struct Reader(tokio::net::TcpStream);

impl serializer::Reader for Reader {
    fn readable(&mut self) -> impl std::future::Future<Output = std::io::Result<()>> {
        self.0.readable()
    }

    fn try_read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.0.try_read_vectored(bufs)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2049").await?;

    let mut tasks = tokio::task::JoinSet::new();

    loop {
        let (conn, addr) = listener.accept().await?;
        eprintln!("coonnected addr: {:?}", addr);

        tasks.spawn(async move { nfs_mamont::serializer::read(&mut Reader(conn)).await } );
    }
}