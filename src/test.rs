use crate::CancellationToken;

use futures::executor::block_on;
use std::thread;

#[test]
fn cancel_token() {
    let token = CancellationToken::new();
    let token1 = token.clone();

    let th1 = thread::spawn(move || {
        block_on(async {
            token1.cancelled().await;
        });
    });

    let th2 = thread::spawn(move || {
        token.cancel();
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
}

#[test]
fn cancel_token_owned() {
    let token = CancellationToken::new();
    let token1 = token.clone();

    let th1 = thread::spawn(move || {
        block_on(async {
            token1.cancelled_owned().await;
        });
    });

    let th2 = thread::spawn(move || {
        token.cancel();
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
}

#[test]
fn cancel_with_child() {
    let token = CancellationToken::new();
    let token1 = token.clone();
    let token2 = token.clone();
    let child_token = token.child_token();

    let th1 = thread::spawn(move || {
        block_on(async {
            token1.cancelled().await;
        });
    });

    let th2 = thread::spawn(move || {
        token2.cancel();
    });

    let th3 = thread::spawn(move || {
        block_on(async {
            child_token.cancelled().await;
        });
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
    assert!(th3.join().is_ok());
}

#[test]
fn drop_token_no_child() {
    let token = CancellationToken::new();
    let token1 = token.clone();
    let token2 = token.clone();

    let th1 = thread::spawn(move || {
        drop(token1);
    });

    let th2 = thread::spawn(move || {
        drop(token2);
    });

    let th3 = thread::spawn(move || {
        drop(token);
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
    assert!(th3.join().is_ok());
}

#[test]
fn drop_token_with_children() {
    let token1 = CancellationToken::new();
    let child_token1 = token1.child_token();
    let child_token2 = token1.child_token();

    let th1 = thread::spawn(move || {
        drop(token1);
    });

    let th2 = thread::spawn(move || {
        drop(child_token1);
    });

    let th3 = thread::spawn(move || {
        drop(child_token2);
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
    assert!(th3.join().is_ok());
}

#[test]
fn drop_and_cancel_token() {
    let token1 = CancellationToken::new();
    let token2 = token1.clone();
    let child_token = token1.child_token();

    let th1 = thread::spawn(move || {
        drop(token1);
    });

    let th2 = thread::spawn(move || {
        token2.cancel();
    });

    let th3 = thread::spawn(move || {
        drop(child_token);
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
    assert!(th3.join().is_ok());
}

#[test]
fn cancel_parent_and_child() {
    let token1 = CancellationToken::new();
    let token2 = token1.clone();
    let child_token = token1.child_token();

    let th1 = thread::spawn(move || {
        drop(token1);
    });

    let th2 = thread::spawn(move || {
        token2.cancel();
    });

    let th3 = thread::spawn(move || {
        child_token.cancel();
    });

    assert!(th1.join().is_ok());
    assert!(th2.join().is_ok());
    assert!(th3.join().is_ok());
}
