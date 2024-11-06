use std::{convert::Infallible, io, iter::Once, net::SocketAddr};

use axum::{
    body::{Bytes, StreamBody},
    http::StatusCode,
    response::{AppendHeaders, IntoResponse},
    routing::get,
    Router,
};
use futures::{
    stream::{self, Iter},
    Stream,
};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

pub async fn run() {
    let app = Router::new()
        .route("/play", get(play))
        .route("/headers", get(set_headers));
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub async fn set_headers() -> impl IntoResponse {
    let headers = AppendHeaders([("x-my-header", "hello")]);
    let message = "hello from axum".to_owned();

    (StatusCode::IM_A_TEAPOT, headers, message)
}

pub async fn play() -> impl IntoResponse {
    let mut file = File::open("./videos/interlaced.png").await.unwrap();
    let mut image = vec![];
    let _ = file.read_to_end(&mut image).await.unwrap();
    // // let stream = futures::stream::tokio-util(file, |mut f| async move {
    // //     let mut buffer = [0; 2048];
    // //     let _read = f.read(&mut buffer[..]).await.unwrap();
    // //     Some((
    // //         Ok::<_, Infallible>(axum::body::Bytes::copy_from_slice(&buffer)),
    // //         f,
    // //     ))
    // // });
    // // let stream_body = StreamBody::new(stream);
    // let stream_body = StreamBody::new(ReaderStream::new(file));
    let headers = AppendHeaders([("Content-Type", "image/png"), ("x-my-header", "hello")]);
    // (headers, stream_body)
    (headers, image)
}
