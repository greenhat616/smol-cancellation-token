# Smol Cancellation Token

A simple cancellation token implementation for Smol, copied from tokio-util's cancellation token, and replace the `waker` with `EventListener` to make it cross async runtime compatible.
