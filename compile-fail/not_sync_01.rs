use opera::PhantomNotSync;

pub fn assert_sync<T: Sync>() {

}

struct X {
    marker: PhantomNotSync,
}

fn main() {
    assert_sync::<X>();
}
