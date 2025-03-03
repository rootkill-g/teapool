# Teapool ☕

Teapool is a thread pool implementation for spawning threads for requests received over TCP connection and assigning the task to a worker.

## APIs Available 📝

Thread pool can be initialized with `new` associated function on `ThreadPool`, which takes an argument `size` to define the size of the thread pool.

To execute the request received over TCP Stream the Thread Pool exposes the `execute` API which takes the arguments: `handler` which is a function which takes `TcpStream` as an argument, and `stream` of type `TcpStream`.

## What happens when a thread pool is initialized? ⚡

- The thread pool initialization method creates two vectors (`senders` and `workers`) with the capacity of `size` which the initialization method takes as argument.

- Then it creates `size` number of `mpsc` (Multi Producer Single Consumer) channels and pushes the `sender` channels in `senders` vector and the `receiver` channels in the `workers` vector which contains the newly initialized instances of `Worker` types with `id` and `receiver` channel, which further spawns a new thread and call the `work` function from in the closure function. The `work` function keeps running in loop awaiting to receive `job`s  from the `sender` channels in the thread pool.

- Finally it returns the `ThreadPool` with `workers`, `senders` and `next_sender` as it's members.

## How a task is executed in the thread pool? 🛠️

The execute method creates a tuple with the received arguments `handler` and `stream` and defines it as `job`.

Now the `senders` (vector) in the pool sends the `job` over the `sender` channel on the request number index, and increments the `next_sender` variable for global access over the thread pool, and checks if the `next_sender` on increment becomes bigger than the `senders` vector's capacity, it then resets the counter for graceful edge-case handling of typical `index out of bounds` error.

The `worker` which receives this specific job (the worker `id` will be equal to the `next_sender` variable) and runs the function by taking the `job`: (`handler`, `stream`) inside the thread in which the `worker` is running.

###

### Happy Teapooling ☕

Here's an image of Ferris chilling on a pool table 🦀

![Ferris](./assets/teapool.jpg)
