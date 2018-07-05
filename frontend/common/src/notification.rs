use std::collections::HashSet;
use std::collections::VecDeque;
use yew::prelude::worker::*;

pub enum Msg {

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Notification {

}

pub struct NotificationSink {
    repository: Box<Bridge<NotificationRepository>>

}

impl Agent for NotificationSink {
    type Reach = Context;
    type Message = ();
    type Input = Notification;
    type Output = ();

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|_| ());
        NotificationSink {
            repository: NotificationRepository::bridge(callback)
        }

    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        self.repository.send(request)
    }
}


pub struct NotificationRepository {
    link: AgentLink<NotificationRepository>,
    subscribers: HashSet<HandlerId>,
    history: VecDeque<Notification>
}

impl Agent for NotificationRepository {
    type Reach = Context;
    type Message = Msg;
    type Input = Notification;
    type Output = Notification;


    fn create(link: AgentLink<Self>) -> Self {

        NotificationRepository {
            link,
            subscribers: HashSet::new(),
            history: VecDeque::new()
        }

    }


    fn update(&mut self, msg: Self::Message) {

    }


    fn handle(&mut self, notification: Self::Input, who: HandlerId) {
        self.history.push_back(notification);
        for sub in self.subscribers.iter() {
            self.link.response(*sub, notification.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
