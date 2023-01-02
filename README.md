# MacEats

We have developed a full-stack reimplementation of <https://maceats.mcmaster.ca/>. Using Rust on the backend, and the groundbreaking Next.js 13 on the frontend, we have fabricated 3 essential components to solve this issue.

## [The Crate](/crate)

The first step of the solution is a way to easily access all the data provided by the original MacEats. This crate is publicly available for all to use at <https://crates.io/crates/maceats>.

## [The Backend](/backend)

The second step of this solution is a caching REST backend, also made using Rust. This backend provides faster-than-light response time using a state-of-the-art caching system, combined with the speed of our chosen HTTP server crate, warp (<https://crates.io/crates/warp>). This backend is served at <https://maceats-server.fly.dev/>.

## [The Frontend](/frontend)

The final step of this solution is a state-of-the-art frontend using the brand new Next.js 13. We used new React standards, such as async/server components to do all the rendering server-side, making the client-side load very low. This frontend is accessible at <https://maceats.vidhan.io/>.
