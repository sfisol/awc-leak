# awc-leak

This example shows a small memory leak (probably in DNS resolver) using actix client. It fetches static data from self hosted server using `localhost` or `127.0.0.1`.

Example output for `127.0.0.1`:

```
START SERVER
1. 4519
2. 4556
2127. 4562
2383. 4568
2596. 4574
2736. 4580
85848. 4586
347614. 4592
2214972. 4598
2216387. 4604
8811441. 4610
10078874. 4616
```

Example output for `localhost`:

```
START SERVER
1. 5389
2. 5421
2082. 5427
2277. 5433
2472. 5439
2741. 5445
2903. 5451
83365. 5479
167194. 5535
250930. 5592
334678. 5648
418212. 5705
501891. 5761
585731. 5817
669193. 5873
752390. 5928
835617. 5984
918772. 6040
1002355. 6096
1085998. 6151
1169285. 6207
1252619. 6263
1336388. 6319
1419913. 6386
1503515. 6443
1587131. 6499
1670848. 6555
1754347. 6614
1837743. 6668
1921154. 6725
2004724. 6783
2088407. 6839
2172063. 6895
2255781. 6951
2339173. 7006
2422899. 7062
2506543. 7118
2589822. 7174
2709565. 7180
2839875. 7244
2923396. 7311
3090428. 7377
3341695. 7443
3425575. 7509
3509562. 7576
3593225. 7641
3676793. 7707
3760290. 7773
3882708. 7779
3883903. 7785
3922366. 7851
4089515. 7914
4173140. 7980
4256931. 8046
4591661. 8111
4675170. 8177
4758637. 8243
4842135. 8309
5009784. 8374
5093731. 8440
5177660. 8506
5261596. 8564
5414261. 8570
8120933. 8576
8351505. 8638
11883293. 8644
11884657. 8650
```

Valgrind log from `valgrind --leak-check=full --trace-children=yes target/debug/actix-client-leak` (configured to connect to `localhost`):

```
==154116== 
==154116== Process terminating with default action of signal 2 (SIGINT)
==154116==    at 0x4F612CA: recv (recv.c:28)
==154116==    by 0x93DBD6: recv_with_flags (net.rs:199)
==154116==    by 0x93DBD6: read (net.rs:205)
==154116==    by 0x93DBD6: read (net.rs:264)
==154116==    by 0x93DBD6: <&std::net::tcp::TcpStream as std::io::Read>::read (tcp.rs:592)
==154116==    by 0x9014EC: <&mio::sys::unix::tcp::TcpStream as std::io::Read>::read (tcp.rs:142)
==154116==    by 0x905E13: <&mio::net::tcp::TcpStream as std::io::Read>::read (tcp.rs:417)
==154116==    by 0x8DA7F7: tokio::net::tcp::stream::TcpStream::poll_read_priv (stream.rs:709)
==154116==    by 0x8DB034: <tokio::net::tcp::stream::TcpStream as tokio::io::async_read::AsyncRead>::poll_read (stream.rs:876)
==154116==    by 0x2FF7A6: <tokio_openssl::StreamWrapper<S> as std::io::Read>::read::{{closure}} (lib.rs:101)
==154116==    by 0x2F80A2: tokio_openssl::StreamWrapper<S>::with_context (lib.rs:91)
==154116==    by 0x2FF6AE: <tokio_openssl::StreamWrapper<S> as std::io::Read>::read (lib.rs:101)
==154116==    by 0x255B11: openssl::ssl::bio::bread::{{closure}} (bio.rs:115)
==154116==    by 0x299993: core::ops::function::FnOnce::call_once (function.rs:227)
==154116==    by 0x2ED80A: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once (panic.rs:308)
==154116== 
==154116== HEAP SUMMARY:
==154116==     in use at exit: 218,725 bytes in 3,869 blocks
==154116==   total heap usage: 6,397,570 allocs, 6,393,701 frees, 124,066,345,098 bytes allocated
==154116== 
==154116== 2 bytes in 1 blocks are possibly lost in loss record 1 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x9216CB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x921789: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x922089: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x922C1E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x9233EC: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x9228BE: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x91FEBE: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x91F4C3: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x920BBA: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x91BA57: bytes::bytes::Bytes::copy_from_slice (bytes.rs:180)
==154116==    by 0x3E4345: http::header::value::HeaderValue::from_str::{{closure}} (value.rs:101)
==154116== 
==154116== 16 bytes in 1 blocks are possibly lost in loss record 20 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x56EACB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x56EB89: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x570599: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x55325E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x55595C: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x551E7E: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x58617E: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x5B8963: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x5B890A: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x58827B: <alloc::vec::Vec<T> as core::clone::Clone>::clone (vec.rs:1904)
==154116==    by 0x2B6E0A: <trust_dns_proto::rr::domain::name::Name as core::clone::Clone>::clone (name.rs:31)
==154116== 
==154116== 23 bytes in 1 blocks are possibly lost in loss record 26 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x9216CB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x921789: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x922089: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x922C1E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x9233EC: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x9228BE: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x91FEBE: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x91F4C3: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x920BBA: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x91BA57: bytes::bytes::Bytes::copy_from_slice (bytes.rs:180)
==154116==    by 0x3DA4C8: <http::uri::Uri as core::convert::TryFrom<&[u8]>>::try_from (mod.rs:677)
==154116== 
==154116== 24 bytes in 1 blocks are possibly lost in loss record 585 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A859: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x4AC665: trust_dns_resolver::lookup_state::CachingClient<C>::with_cache (lookup_state.rs:84)
==154116==    by 0x5132F1: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:255)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x512756: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:203)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 660 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x9216CB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x921789: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x922089: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x92162C: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x91D863: new<bytes::bytes::Shared> (boxed.rs:175)
==154116==    by 0x91D863: bytes::bytes::shallow_clone_vec (bytes.rs:976)
==154116==    by 0x91CCE2: bytes::bytes::promotable_even_clone (bytes.rs:856)
==154116==    by 0x91C718: <bytes::bytes::Bytes as core::clone::Clone>::clone (bytes.rs:499)
==154116==    by 0x91C4B1: bytes::bytes::Bytes::split_to (bytes.rs:394)
==154116==    by 0x89E830: http::uri::parse_full (mod.rs:794)
==154116==    by 0x89D9D6: http::uri::Uri::from_shared (mod.rs:301)
==154116==    by 0x3DA4D8: <http::uri::Uri as core::convert::TryFrom<&[u8]>>::try_from (mod.rs:677)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 661 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A707: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D90: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:66)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DD231: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 662 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A707: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D90: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:66)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D8083: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 663 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A707: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D90: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:66)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DCEF1: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 664 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A707: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D90: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:66)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D73D3: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 32 bytes in 1 blocks are possibly lost in loss record 665 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x4B597E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x4B93EC: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x4B524E: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x4D654E: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x2CF593: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x2ABD3A: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x2FEECB: <alloc::vec::Vec<T> as core::clone::Clone>::clone (vec.rs:1904)
==154116==    by 0x32A052: <trust_dns_resolver::config::ResolverConfig as core::clone::Clone>::clone (config.rs:35)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 674 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F14D8: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x5125A1: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:201)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116==    by 0x502733: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x23B5E8: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<trust_dns_resolver::name_server::connection_provider::tokio_runtime::TokioRuntime>>::tokio::{{closure}} (async_resolver.rs:130)
==154116==    by 0x2473C3: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x24B3E0: actix_connect::get_default_resolver::{{closure}} (lib.rs:63)
==154116==    by 0x247053: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 675 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1383: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x503CD1: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DD231: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116==    by 0x4DBE88: <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter (vec.rs:2119)
==154116==    by 0x4DE0E6: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter (vec.rs:1959)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 676 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1238: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x4D07A9: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DD231: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116==    by 0x4DBE88: <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter (vec.rs:2119)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 677 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1383: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x503CD1: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D8083: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116==    by 0x4DB3C3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116==    by 0x4DD3AF: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2096)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 678 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1238: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x4D07A9: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D8083: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116==    by 0x4DB3C3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 679 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1383: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x503CD1: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DCEF1: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116==    by 0x4DC058: <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter (vec.rs:2119)
==154116==    by 0x4DE016: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter (vec.rs:1959)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 680 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1238: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x4D07A9: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DCEF1: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116==    by 0x4DC058: <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter (vec.rs:2119)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 681 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1383: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x503CD1: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D73D3: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116==    by 0x4DB5D3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116==    by 0x4DD06F: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2096)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 682 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x94125C: alloc (alloc.rs:75)
==154116==    by 0x94125C: alloc_impl (alloc.rs:154)
==154116==    by 0x94125C: alloc (alloc.rs:213)
==154116==    by 0x94125C: exchange_malloc (alloc.rs:303)
==154116==    by 0x94125C: new<std::sys::unix::mutex::Mutex> (boxed.rs:175)
==154116==    by 0x94125C: from<std::sys::unix::mutex::Mutex> (boxed.rs:737)
==154116==    by 0x94125C: std::sys_common::mutex::MovableMutex::new (mutex.rs:64)
==154116==    by 0x4B4421: std::sync::mutex::Mutex<T>::new (mutex.rs:217)
==154116==    by 0x4F1238: futures_util::lock::mutex::Mutex<T>::new (mutex.rs:78)
==154116==    by 0x4D07A9: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D73D3: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116==    by 0x4DB5D3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 683 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50B767: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x48AFDB: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider (name_server_pool.rs:104)
==154116==    by 0x512DB9: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:225)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x512756: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:203)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116== 
==154116== 40 bytes in 1 blocks are possibly lost in loss record 684 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50B767: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x48B028: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider (name_server_pool.rs:105)
==154116==    by 0x512DB9: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:225)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x512756: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:203)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116== 
==154116== 64 bytes in 1 blocks are possibly lost in loss record 722 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50B8DD: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x5130BF: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:246)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x512756: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:203)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116==    by 0x502733: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 745 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AB5D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503CE4: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DD231: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 746 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A53D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D2C: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DD231: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 747 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AB5D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503CE4: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D8083: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 748 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A53D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D2C: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D8083: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 749 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AB5D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503CE4: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DCEF1: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 750 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A53D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D2C: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4DCEF1: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2082)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 751 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AB5D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503CE4: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:64)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D73D3: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 96 bytes in 1 blocks are possibly lost in loss record 752 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A53D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x503D2C: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116==    by 0x4D73D3: alloc::vec::Vec<T>::extend_desugared (vec.rs:2390)
==154116== 
==154116== 116 bytes in 1 blocks are possibly lost in loss record 759 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x3CFC2B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x3EF036: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==154116==    by 0x3EF326: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==154116==    by 0x3316E2: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==154116==    by 0x32C273: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==154116==    by 0x331E81: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==154116==    by 0x330966: hashbrown::raw::RawTable<T>::insert (mod.rs:926)
==154116==    by 0x2571E7: hashbrown::map::HashMap<K,V,S>::insert (map.rs:991)
==154116==    by 0x28333D: std::collections::hash::map::HashMap<K,V,S>::insert (map.rs:840)
==154116==    by 0x2442D1: actix_rt::arbiter::Arbiter::set_item::{{closure}} (arbiter.rs:259)
==154116==    by 0x33A971: std::thread::local::LocalKey<T>::try_with (local.rs:272)
==154116== 
==154116== 144 bytes in 2 blocks are possibly lost in loss record 781 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AEED: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x4D0791: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B2E2: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:81)
==154116==    by 0x481E21: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB686: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x4968A7: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116== 
==154116== 144 bytes in 2 blocks are possibly lost in loss record 782 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50AEED: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x4D0791: trust_dns_resolver::name_server::name_server_state::NameServerState::init (name_server_state.rs:72)
==154116==    by 0x503D19: trust_dns_resolver::name_server::name_server::NameServer<C,P>::new_with_provider (name_server.rs:65)
==154116==    by 0x48B462: trust_dns_resolver::name_server::name_server_pool::NameServerPool<C,P>::from_config_with_provider::{{closure}} (name_server_pool.rs:99)
==154116==    by 0x481DF1: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once (function.rs:280)
==154116==    by 0x4EB876: core::option::Option<T>::map (option.rs:453)
==154116==    by 0x496967: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next (mod.rs:924)
==154116== 
==154116== 200 bytes in 5 blocks are possibly lost in loss record 799 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50B437: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x521903: trust_dns_resolver::lookup::Lookup::append (lookup.rs:118)
==154116==    by 0x4DE712: trust_dns_resolver::hosts::Hosts::insert (hosts.rs:82)
==154116==    by 0x4DFBDF: trust_dns_resolver::hosts::read_hosts_conf (hosts.rs:160)
==154116==    by 0x4DE131: trust_dns_resolver::hosts::Hosts::new (hosts.rs:38)
==154116==    by 0x5130AC: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:246)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116== 
==154116== 216 bytes in 1 blocks are possibly lost in loss record 800 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50A99D: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x5125B4: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::new_with_conn::{{closure}} (async_resolver.rs:201)
==154116==    by 0x501F23: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x5120E7: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<R>>::new::{{closure}} (async_resolver.rs:163)
==154116==    by 0x502733: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x23B5E8: trust_dns_resolver::async_resolver::AsyncResolver<trust_dns_resolver::name_server::connection_provider::GenericConnection,trust_dns_resolver::name_server::connection_provider::GenericConnectionProvider<trust_dns_resolver::name_server::connection_provider::tokio_runtime::TokioRuntime>>::tokio::{{closure}} (async_resolver.rs:130)
==154116==    by 0x2473C3: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116== 
==154116== 256 bytes in 1 blocks are possibly lost in loss record 809 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x4B6A5E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x4B93AC: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x4B527E: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x4D645E: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x2CF503: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x32934A: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x2FEE7B: <alloc::vec::Vec<T> as core::clone::Clone>::clone (vec.rs:1904)
==154116==    by 0x32A467: <trust_dns_resolver::config::NameServerConfigGroup as core::clone::Clone>::clone (config.rs:335)
==154116== 
==154116== 276 bytes in 1 blocks are possibly lost in loss record 814 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x2A893B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x32F046: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==154116==    by 0x32F946: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==154116==    by 0x330B42: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==154116==    by 0x32C063: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==154116==    by 0x331F21: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==154116==    by 0x25758A: hashbrown::map::HashMap<K,V,S>::reserve (map.rs:670)
==154116==    by 0x256C30: hashbrown::rustc_entry::<impl hashbrown::map::HashMap<K,V,S>>::rustc_entry (rustc_entry.rs:45)
==154116==    by 0x2832FB: std::collections::hash::map::HashMap<K,V,S>::entry (map.rs:704)
==154116==    by 0x2F04B2: actix_http::client::pool::Inner<Io>::release_conn (pool.rs:361)
==154116==    by 0x2F35AC: actix_http::client::pool::Acquired<T>::release (pool.rs:630)
==154116== 
==154116== 464 bytes in 1 blocks are possibly lost in loss record 826 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x2A893B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x2A89F9: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x2B5EC9: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x2A889C: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x24428A: new<actix_connect::DefaultResolver> (boxed.rs:175)
==154116==    by 0x24428A: actix_rt::arbiter::Arbiter::set_item::{{closure}} (arbiter.rs:259)
==154116==    by 0x33A971: std::thread::local::LocalKey<T>::try_with (local.rs:272)
==154116==    by 0x336CA2: std::thread::local::LocalKey<T>::with (local.rs:248)
==154116==    by 0x2441A4: actix_rt::arbiter::Arbiter::set_item (arbiter.rs:259)
==154116==    by 0x24B6B8: actix_connect::get_default_resolver::{{closure}} (lib.rs:65)
==154116==    by 0x247053: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116==    by 0x2EB5AE: <actix_connect::resolve::Resolver<T> as actix_service::Service>::call::{{closure}} (resolve.rs:132)
==154116== 
==154116== 576 bytes in 1 blocks are possibly lost in loss record 838 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x2A893B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x2A89F9: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x2B5EC9: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x3A46AE: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x3A68BC: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x3A2C5E: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x3A1921: alloc::collections::vec_deque::VecDeque<T>::with_capacity (vec_deque.rs:518)
==154116==    by 0x3A20A2: alloc::collections::vec_deque::VecDeque<T>::new (vec_deque.rs:500)
==154116==    by 0x299D70: core::ops::function::FnOnce::call_once (function.rs:227)
==154116==    by 0x28304B: std::collections::hash::map::Entry<K,V>::or_insert_with (map.rs:2221)
==154116==    by 0x2F04C2: actix_http::client::pool::Inner<Io>::release_conn (pool.rs:361)
==154116== 
==154116== 680 bytes in 17 blocks are possibly lost in loss record 847 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x50CD29: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x50EF39: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x50CBCC: alloc::alloc::exchange_malloc (alloc.rs:303)
==154116==    by 0x50B437: alloc::sync::Arc<T>::new (sync.rs:315)
==154116==    by 0x521903: trust_dns_resolver::lookup::Lookup::append (lookup.rs:118)
==154116==    by 0x4DE712: trust_dns_resolver::hosts::Hosts::insert (hosts.rs:82)
==154116==    by 0x4DF9C4: trust_dns_resolver::hosts::read_hosts_conf (hosts.rs:155)
==154116==    by 0x4DE131: trust_dns_resolver::hosts::Hosts::new (hosts.rs:38)
==154116==    by 0x5130AC: trust_dns_resolver::async_resolver::AsyncResolver<C,P>::with_cache_with_provider::{{closure}} (async_resolver.rs:246)
==154116==    by 0x503503: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (mod.rs:80)
==154116== 
==154116== 720 bytes in 27 blocks are possibly lost in loss record 852 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x56EACB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x56EB89: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x570599: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x5B8893: alloc::sync::Arc<[T]>::allocate_for_slice::{{closure}} (sync.rs:1038)
==154116==    by 0x5B825C: alloc::sync::Arc<T>::allocate_for_layout (sync.rs:983)
==154116==    by 0x5B8840: alloc::sync::Arc<[T]>::allocate_for_slice (sync.rs:1036)
==154116==    by 0x5B874D: alloc::sync::Arc<[T]>::copy_from_slice (sync.rs:1062)
==154116==    by 0x5BA6B3: <alloc::sync::Arc<[T]> as alloc::sync::ArcFromSlice<T>>::from_slice (sync.rs:1134)
==154116==    by 0x5BA733: <alloc::sync::Arc<[T]> as core::convert::From<&[T]>>::from (sync.rs:2169)
==154116==    by 0x5BA979: trust_dns_proto::rr::domain::label::Label::from_raw_bytes (label.rs:43)
==154116==    by 0x5BB22A: trust_dns_proto::rr::domain::label::Label::from_ascii (label.rs:81)
==154116== 
==154116== 960 bytes in 5 blocks are possibly lost in loss record 863 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x56EACB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x56EB89: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x570599: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x552B9E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x555A1C: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x551DEE: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x5861CE: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x52174A: trust_dns_resolver::lookup::Lookup::append (lookup.rs:112)
==154116==    by 0x4DE712: trust_dns_resolver::hosts::Hosts::insert (hosts.rs:82)
==154116==    by 0x4DFBDF: trust_dns_resolver::hosts::read_hosts_conf (hosts.rs:160)
==154116==    by 0x4DE131: trust_dns_resolver::hosts::Hosts::new (hosts.rs:38)
==154116== 
==154116== 1,056 bytes in 1 blocks are possibly lost in loss record 873 of 908
==154116==    at 0x483AD7B: realloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x658FFC: alloc::alloc::realloc (alloc.rs:111)
==154116==    by 0x658CDC: alloc::alloc::Global::grow_impl (alloc.rs:186)
==154116==    by 0x659163: <alloc::alloc::Global as core::alloc::AllocRef>::grow (alloc.rs:238)
==154116==    by 0x6529AC: alloc::raw_vec::finish_grow (raw_vec.rs:491)
==154116==    by 0x4B80D6: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:427)
==154116==    by 0x4B6FC3: alloc::raw_vec::RawVec<T,A>::try_reserve (raw_vec.rs:316)
==154116==    by 0x4B9C32: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:310)
==154116==    by 0x4D9148: alloc::vec::Vec<T>::reserve (vec.rs:505)
==154116==    by 0x4D817D: alloc::vec::Vec<T>::extend_desugared (vec.rs:2394)
==154116==    by 0x4DB3C3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116==    by 0x4DD3AF: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2096)
==154116== 
==154116== 1,056 bytes in 1 blocks are possibly lost in loss record 874 of 908
==154116==    at 0x483AD7B: realloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x658FFC: alloc::alloc::realloc (alloc.rs:111)
==154116==    by 0x658CDC: alloc::alloc::Global::grow_impl (alloc.rs:186)
==154116==    by 0x659163: <alloc::alloc::Global as core::alloc::AllocRef>::grow (alloc.rs:238)
==154116==    by 0x6529AC: alloc::raw_vec::finish_grow (raw_vec.rs:491)
==154116==    by 0x4B80D6: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:427)
==154116==    by 0x4B6FC3: alloc::raw_vec::RawVec<T,A>::try_reserve (raw_vec.rs:316)
==154116==    by 0x4B9C32: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:310)
==154116==    by 0x4D9148: alloc::vec::Vec<T>::reserve (vec.rs:505)
==154116==    by 0x4D74CD: alloc::vec::Vec<T>::extend_desugared (vec.rs:2394)
==154116==    by 0x4DB5D3: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend (vec.rs:2313)
==154116==    by 0x4DD06F: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter (vec.rs:2096)
==154116== 
==154116== 1,200 bytes in 66 blocks are possibly lost in loss record 877 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x56EACB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x56EB89: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x570599: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x55325E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x55595C: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x551E7E: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x58617E: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x5B8963: alloc::slice::hack::to_vec (slice.rs:159)
==154116==    by 0x5B890A: alloc::slice::<impl [T]>::to_vec (slice.rs:395)
==154116==    by 0x58827B: <alloc::vec::Vec<T> as core::clone::Clone>::clone (vec.rs:1904)
==154116==    by 0x489F3A: <trust_dns_proto::rr::domain::name::Name as core::clone::Clone>::clone (name.rs:31)
==154116== 
==154116== 2,864 bytes in 1 blocks are possibly lost in loss record 894 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x3CFC2B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x3EED36: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==154116==    by 0x3EF616: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==154116==    by 0x3EC7E1: hashbrown::raw::RawTable<T>::with_capacity (mod.rs:459)
==154116==    by 0x3F7856: hashbrown::map::HashMap<K,V,S>::with_capacity_and_hasher (map.rs:315)
==154116==    by 0x3D43A3: std::collections::hash::map::HashMap<K,V,S>::with_capacity_and_hasher (map.rs:298)
==154116==    by 0x3F4B40: actix_http::header::map::HeaderMap::with_capacity (map.rs:74)
==154116==    by 0x3DE992: <actix_http::message::RequestHead as core::default::Default>::default (message.rs:58)
==154116==    by 0x1EC957: awc::request::ClientRequest::new (request.rs:75)
==154116==    by 0x1F41E7: awc::Client::request (lib.rs:183)
==154116==    by 0x1F4111: awc::Client::get (lib.rs:213)
==154116== 
==154116== 3,456 bytes in 17 blocks are possibly lost in loss record 901 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x56EACB: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x56EB89: alloc::alloc::Global::alloc_impl (alloc.rs:154)
==154116==    by 0x570599: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:213)
==154116==    by 0x552B9E: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:188)
==154116==    by 0x555A1C: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:163)
==154116==    by 0x551DEE: alloc::raw_vec::RawVec<T>::with_capacity (raw_vec.rs:93)
==154116==    by 0x5861CE: alloc::vec::Vec<T>::with_capacity (vec.rs:363)
==154116==    by 0x52174A: trust_dns_resolver::lookup::Lookup::append (lookup.rs:112)
==154116==    by 0x4DE712: trust_dns_resolver::hosts::Hosts::insert (hosts.rs:82)
==154116==    by 0x4DF9C4: trust_dns_resolver::hosts::read_hosts_conf (hosts.rs:155)
==154116==    by 0x4DE131: trust_dns_resolver::hosts::Hosts::new (hosts.rs:38)
==154116== 
==154116== 5,168 bytes in 1 blocks are possibly lost in loss record 902 of 908
==154116==    at 0x483877F: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==154116==    by 0x50CC6B: alloc::alloc::alloc (alloc.rs:75)
==154116==    by 0x531CF6: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==154116==    by 0x532636: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==154116==    by 0x5368D2: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==154116==    by 0x52C7F3: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==154116==    by 0x537291: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==154116==    by 0x4A7BA1: hashbrown::map::HashMap<K,V,S>::reserve (map.rs:670)
==154116==    by 0x4A5E06: hashbrown::rustc_entry::<impl hashbrown::map::HashMap<K,V,S>>::rustc_entry (rustc_entry.rs:45)
==154116==    by 0x4A077B: std::collections::hash::map::HashMap<K,V,S>::entry (map.rs:704)
==154116==    by 0x4DE3E4: trust_dns_resolver::hosts::Hosts::insert (hosts.rs:61)
==154116==    by 0x4DF9C4: trust_dns_resolver::hosts::read_hosts_conf (hosts.rs:155)
==154116== 
==154116== LEAK SUMMARY:
==154116==    definitely lost: 0 bytes in 0 blocks
==154116==    indirectly lost: 0 bytes in 0 blocks
==154116==      possibly lost: 21,081 bytes in 181 blocks
==154116==    still reachable: 197,644 bytes in 3,688 blocks
==154116==         suppressed: 0 bytes in 0 blocks
==154116== Reachable blocks (those to which a pointer was found) are not shown.
==154116== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==154116== 
==154116== For lists of detected and suppressed errors, rerun with: -s
==154116== ERROR SUMMARY: 48 errors from 48 contexts (suppressed: 0 from 0)
```
