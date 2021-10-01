use opera::PhantomNotSend;

pub fn assert_send<T: Send>() {

}

struct X {
    marker: PhantomNotSend,
}

fn main() {
    assert_send::<X>();
}
