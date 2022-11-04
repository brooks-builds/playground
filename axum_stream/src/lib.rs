use std::{convert::Infallible, io, iter::Once, net::SocketAddr};

use axum::{
    body::{Bytes, StreamBody},
    routing::get,
    Router,
};
use futures::{
    stream::{self, Iter},
    Stream,
};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn run() {
    let app = Router::new().route("/play", get(play));
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub async fn play() -> StreamBody<impl Stream<Item = Result<Bytes, Infallible>>> {
    let file = File::open("./videos/other_test.mp4").await.unwrap();
    let stream = futures::stream::unfold(file, |mut f| async move {
        let mut buffer = [0; 2048];
        let _read = f.read(&mut buffer[..]).await.unwrap();
        Some((
            Ok::<_, Infallible>(axum::body::Bytes::copy_from_slice(&buffer)),
            f,
        ))
    });
    let stream_body = StreamBody::new(stream);
    stream_body
}
