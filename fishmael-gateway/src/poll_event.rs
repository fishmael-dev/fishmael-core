use futures::Stream;
use std::{
    fs,
    future::Future,
    pin::Pin,
    task::{
        ready,
        Context as AsyncContext,
        Poll
    },
};
use twilight_model::gateway::event::Event;

use crate::{
    deserialize::deserialize,
    message::Message,
    error::{ReceiveError, ReceiveErrorKind},
};


pub struct PollEvent<'a, St: ?Sized> {
    stream: &'a mut St,
}


impl<'a, St: ?Sized> PollEvent<'a, St> {
    pub fn new(stream: &'a mut St) -> Self {
        Self{stream}
    }
}


impl<'a, St: ?Sized + Stream<Item = Result<Message, ReceiveError>> + Unpin> Future for PollEvent<'a, St> {
    type Output = Option<Result<Event, ReceiveError>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut AsyncContext<'_>) -> Poll<Self::Output> {
        let try_from_message = |message| match message {
            Message::Text(json) => deserialize(json).map(|o| o.map(Into::into)),
            Message::Close(frame) => Ok(Some(Event::GatewayClose(frame))),
        };
        
        loop {
            match ready!(Pin::new(&mut self.stream).poll_next(cx)) {
                Some(item) => {
                    match item.and_then(try_from_message) {
                        Ok(Some(event)) => {
                            return Poll::Ready(Some(Ok(event)));
                        },
                        Ok(None) => {println!("skipping event...");}
                        Err(ReceiveError{kind: ReceiveErrorKind::Deserializing{event}, source: Some(source)}) => {
                            println!("failed to deserialise event: {}...\n\twith reason: {}", &event[..100], source);

                            fs::write("failed.json", event).unwrap();
                            panic!("wee");
                        },
                        Err(err) => {
                            println!("failed to deserialise event with reason: {}", err)
                        }
                    }
                }
                None => {
                    return Poll::Ready(None)
                },
            }
        }
    }
}
