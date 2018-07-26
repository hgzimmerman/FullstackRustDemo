use stdweb::web::event::ResizeEvent;
use stdweb::Value;
use yew::callback::Callback;


pub struct ResizeService {}

pub struct ResizeTask(Option<Value>);

impl ResizeService {
    pub fn new() -> ResizeService {
        ResizeService {}
    }

    pub fn register(&mut self, callback: Callback<()>) -> ResizeTask {
        let callback = move || {
            callback.emit(());
        };
        // let ms = to_ms(duration);
        let handle = js! {
            var callback = @{callback};
            var action = function() {
                callback();
            };
            return window.addEventListener("resize", action);
        };
        return ResizeTask(Some(handle))
    }
}

impl Drop for ResizeTask {
    fn drop(&mut self) {
        let handle = self.0.take().expect("Resize task already empty.");
        js! {
            @(no_return)
            var handle = @{handle};
            handle.callback.drop();
        }
    }
}