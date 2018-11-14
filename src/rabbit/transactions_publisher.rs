use std::io::Error as StdIoError;

use super::error::*;
use super::r2d2::RabbitConnectionManager;
use super::r2d2::RabbitPool;
use futures::future;
use futures_cpupool::CpuPool;
use lapin_futures::channel::{Channel, ExchangeDeclareOptions, QueueDeclareOptions};
use models::*;
use prelude::*;
use r2d2::PooledConnection;
use serde_json;
use tokio::net::tcp::TcpStream;

pub trait TransactionPublisher: Send + Sync + 'static {
    fn push(&self, push: PushNotifications) -> Box<Future<Item = (), Error = Error> + Send>;
    fn callback(&self, callback: Callback) -> Box<Future<Item = (), Error = Error> + Send>;
}

#[derive(Clone)]
pub struct TransactionPublisherImpl {
    rabbit_pool: RabbitPool,
    thread_pool: CpuPool,
}

impl TransactionPublisherImpl {
    pub fn new(rabbit_pool: RabbitPool, thread_pool: CpuPool) -> Self {
        Self { rabbit_pool, thread_pool }
    }

    pub fn init(&self) -> impl Future<Item = (), Error = Error> {
        let self_clone = self.clone();
        self.get_channel().and_then(move |channel| self_clone.declare(&channel))
    }

    fn get_channel(&self) -> impl Future<Item = PooledConnection<RabbitConnectionManager>, Error = Error> {
        // unresolved at the moment - ideally we want to call get on other thread, since it's blocking
        // on the other hand doing so we escape from the thread that has tokio core reference and
        // therefore cannot do spawns
        // let rabbit_pool = self.rabbit_pool.clone();
        // self.thread_pool
        //     .spawn_fn(move || rabbit_pool.get().map_err(ectx!(ErrorSource::Lapin, ErrorKind::Internal)))
        self.rabbit_pool
            .get()
            .map_err(ectx!(ErrorSource::Lapin, ErrorKind::Internal))
            .into_future()
    }

    fn declare(&self, channel: &Channel<TcpStream>) -> impl Future<Item = (), Error = Error> {
        let f1: Box<Future<Item = (), Error = StdIoError>> = Box::new(channel.exchange_declare(
            "notifications",
            "direct",
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default(),
        ));
        let f2: Box<Future<Item = (), Error = StdIoError>> = Box::new(
            channel
                .queue_declare(
                    "pushes",
                    QueueDeclareOptions {
                        durable: true,
                        ..Default::default()
                    },
                    Default::default(),
                ).map(|_| ()),
        );
        let f3: Box<Future<Item = (), Error = StdIoError>> = Box::new(
            channel
                .queue_declare(
                    "callbacks",
                    QueueDeclareOptions {
                        durable: true,
                        ..Default::default()
                    },
                    Default::default(),
                ).map(|_| ()),
        );
        let f4: Box<Future<Item = (), Error = StdIoError>> =
            Box::new(channel.queue_bind("pushes", "notifications", "pushes", Default::default(), Default::default()));
        let f5: Box<Future<Item = (), Error = StdIoError>> =
            Box::new(channel.queue_bind("callbacks", "notifications", "callbacks", Default::default(), Default::default()));
        future::join_all(vec![f1, f2, f3, f4, f5])
            .map(|_| ())
            .map_err(ectx!(ErrorSource::Lapin, ErrorKind::Internal))
    }
}

impl TransactionPublisher for TransactionPublisherImpl {
    fn push(&self, push: PushNotifications) -> Box<Future<Item = (), Error = Error> + Send> {
        Box::new(
            self.get_channel()
                .and_then(move |channel| {
                    let payload = serde_json::to_string(&push).unwrap().into_bytes();
                    channel
                        .clone()
                        .basic_publish("notifications", "pushes", payload, Default::default(), Default::default())
                        .map_err(ectx!(ErrorSource::Lapin, ErrorKind::Internal))
                }).map(|_| ()),
        )
    }
    fn callback(&self, callback: Callback) -> Box<Future<Item = (), Error = Error> + Send> {
        Box::new(
            self.get_channel()
                .and_then(move |channel| {
                    let payload = serde_json::to_string(&callback).unwrap().into_bytes();
                    channel
                        .clone()
                        .basic_publish("notifications", "callbacks", payload, Default::default(), Default::default())
                        .map_err(ectx!(ErrorSource::Lapin, ErrorKind::Internal))
                }).map(|_| ()),
        )
    }
}